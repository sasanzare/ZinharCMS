#!/usr/bin/env node

import { createHash } from "node:crypto";
import { deflateRawSync } from "node:zlib";
import { promises as fs } from "node:fs";
import path from "node:path";
import os from "node:os";

const MANIFEST_SCHEMA_VERSION = "2026-07";
const CURRENT_ZINHAR_VERSION = "3.0.0";
const MAX_PACKAGE_BYTES = 52_428_800;
const MAX_UNCOMPRESSED_BYTES = MAX_PACKAGE_BYTES * 2;
const MAX_PACKAGE_FILES = 500;
const MAX_DECLARED_ASSETS = 200;
const MAX_MANIFEST_DEPENDENCIES = 100;

const PRODUCT_TYPES = new Set([
  "component_pack",
  "design_template",
  "integration_plugin",
  "backend_extension",
]);

const PERMISSIONS = new Set([
  "content.read",
  "content.write",
  "page.read",
  "page.write",
  "media.read",
  "media.write",
  "webhook.send",
  "settings.read",
  "external_network.request",
]);

const PUBLIC_HOOK_TYPES = new Set([
  "sidebar.item",
  "dashboard.widget",
  "form.field",
  "webhook.adapter",
]);

const SUPPORTED_FEATURES = new Set([
  "component_packs",
  "design_templates",
  "integration_plugins",
  "media_assets",
  "page_builder",
]);

const RISK_RANK = { low: 0, medium: 1, high: 2, critical: 3 };

main().catch((error) => {
  console.error(`zinhar-marketplace: ${error.message}`);
  process.exitCode = 1;
});

async function main() {
  const [command, ...rest] = process.argv.slice(2);
  if (!command || command === "--help" || command === "-h") {
    printHelp();
    return;
  }

  if (command === "validate") {
    const { positional, options } = parseArgs(rest);
    const target = positional[0];
    if (!target) usage("validate requires a package directory or ZIP path");
    const report = await validateTarget(target, options);
    printReport(report, options);
    process.exitCode = report.ok ? 0 : 1;
    return;
  }

  if (command === "pack") {
    const { positional, options } = parseArgs(rest);
    const target = positional[0];
    if (!target) usage("pack requires a package directory");
    const result = await packTarget(target, options);
    printReport(result.report, options);
    if (!result.report.ok) {
      process.exitCode = 1;
      return;
    }
    if (options.json) {
      console.log(JSON.stringify({ artifact: result.artifact }, null, 2));
    } else {
      console.log(`Created ${result.artifact.path}`);
      console.log(`SHA-256 ${result.artifact.sha256}`);
    }
    return;
  }

  if (command === "submit") {
    const { positional, options } = parseArgs(rest);
    const target = positional[0];
    if (!target) usage("submit requires a package directory or ZIP path");
    const result = await submitTarget(target, options);
    if (options.json) {
      console.log(JSON.stringify(result, null, 2));
    } else {
      console.log(`Submitted package version to ${result.url}`);
      console.log(`HTTP ${result.status}`);
      console.log(JSON.stringify(result.body, null, 2));
    }
    return;
  }

  usage(`unknown command '${command}'`);
}

function printHelp() {
  console.log(`ZinharCMS Marketplace creator CLI

Usage:
  node scripts/marketplace-cli.mjs validate <package-dir|package.zip> [--manifest manifest.json] [--json]
  node scripts/marketplace-cli.mjs pack <package-dir> [--out marketplace-dist] [--force] [--json]
  node scripts/marketplace-cli.mjs submit <package-dir|package.zip> --listing-id <uuid> [--manifest manifest.json]

Submit options:
  --api-url <url>           API base URL, default ZINHAR_API_URL or http://localhost:8080
  --token <token>           Bearer token, default ZINHAR_TOKEN
  --organization-id <uuid>  Tenant header, default ZINHAR_ORGANIZATION_ID

Validation is intentionally local and conservative. The backend upload pipeline
remains the final authority for package review decisions.`);
}

function usage(message) {
  throw new Error(`${message}\nRun with --help for usage.`);
}

function parseArgs(args) {
  const positional = [];
  const options = {};
  for (let index = 0; index < args.length; index += 1) {
    const value = args[index];
    if (!value.startsWith("--")) {
      positional.push(value);
      continue;
    }
    const key = value.slice(2);
    if (["json", "force"].includes(key)) {
      options[key] = true;
      continue;
    }
    const next = args[index + 1];
    if (!next || next.startsWith("--")) {
      usage(`--${key} requires a value`);
    }
    options[toCamelCase(key)] = next;
    index += 1;
  }
  return { positional, options };
}

function toCamelCase(value) {
  return value.replace(/-([a-z])/g, (_, letter) => letter.toUpperCase());
}

async function validateTarget(target, options = {}) {
  const targetPath = path.resolve(target);
  const stat = await fs.stat(targetPath);
  if (stat.isDirectory()) {
    return validateDirectory(targetPath, options);
  }
  return validateArchive(targetPath, options);
}

async function validateDirectory(packageDir, options = {}) {
  const manifestPath = path.resolve(
    options.manifest ? options.manifest : path.join(packageDir, "manifest.json"),
  );
  const manifest = await readManifest(manifestPath);
  const entries = await collectDirectoryEntries(packageDir);
  return buildValidationReport({
    manifest,
    entries,
    artifactBytes: null,
    artifactFileName: null,
    packageRoot: packageDir,
    manifestPath,
    organizationPlan: options.plan ?? "free",
  });
}

async function validateArchive(archivePath, options = {}) {
  const manifestPath = options.manifest
    ? path.resolve(options.manifest)
    : path.resolve(path.dirname(archivePath), "manifest.json");
  const manifest = await readManifest(manifestPath);
  const bytes = await fs.readFile(archivePath);
  const entries = parseZipEntries(bytes);
  return buildValidationReport({
    manifest,
    entries,
    artifactBytes: bytes,
    artifactFileName: path.basename(archivePath),
    packageRoot: path.dirname(archivePath),
    manifestPath,
    organizationPlan: options.plan ?? "free",
  });
}

async function packTarget(target, options = {}) {
  const packageDir = path.resolve(target);
  const stat = await fs.stat(packageDir);
  if (!stat.isDirectory()) {
    usage("pack expects a package directory, not an existing ZIP");
  }

  const report = await validateDirectory(packageDir, options);
  if (!report.ok) {
    return { report, artifact: null };
  }

  const manifest = report.manifest;
  const outputPath = await resolveOutputPath(packageDir, manifest, options);
  await fs.mkdir(path.dirname(outputPath), { recursive: true });

  if (!options.force && await exists(outputPath)) {
    throw new Error(`output file already exists: ${outputPath}. Pass --force to overwrite.`);
  }

  const entries = await collectDirectoryEntries(packageDir, { exclude: [outputPath] });
  const zipBytes = await createZip(packageDir, entries);
  if (zipBytes.length === 0 || zipBytes.length > MAX_PACKAGE_BYTES) {
    throw new Error("package artifact is empty or exceeds the Marketplace size limit");
  }

  await fs.writeFile(outputPath, zipBytes);
  const sha256 = createHash("sha256").update(zipBytes).digest("hex");
  return {
    report,
    artifact: {
      path: outputPath,
      file_name: path.basename(outputPath),
      size_bytes: zipBytes.length,
      sha256,
    },
  };
}

async function submitTarget(target, options = {}) {
  const listingId = options.listingId;
  if (!listingId) usage("submit requires --listing-id");

  const apiUrl = (options.apiUrl ?? process.env.ZINHAR_API_URL ?? "http://localhost:8080").replace(/\/+$/, "");
  const token = options.token ?? process.env.ZINHAR_TOKEN;
  const organizationId = options.organizationId ?? process.env.ZINHAR_ORGANIZATION_ID;
  if (!token) usage("submit requires --token or ZINHAR_TOKEN");
  if (!organizationId) usage("submit requires --organization-id or ZINHAR_ORGANIZATION_ID");

  const targetPath = path.resolve(target);
  const stat = await fs.stat(targetPath);
  let archivePath = targetPath;
  let manifestPath = options.manifest ? path.resolve(options.manifest) : path.resolve(path.dirname(targetPath), "manifest.json");

  if (stat.isDirectory()) {
    const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "zinhar-marketplace-"));
    const packResult = await packTarget(targetPath, {
      ...options,
      out: tempDir,
      force: true,
      json: true,
    });
    if (!packResult.report.ok) {
      throw new Error("local validation failed; package was not submitted");
    }
    archivePath = packResult.artifact.path;
    manifestPath = options.manifest ? path.resolve(options.manifest) : path.resolve(targetPath, "manifest.json");
  } else {
    const preflight = await validateArchive(archivePath, { ...options, manifest: manifestPath });
    if (!preflight.ok) {
      throw new Error("local validation failed; package was not submitted");
    }
  }

  const manifest = await readManifest(manifestPath);
  const archiveBytes = await fs.readFile(archivePath);
  const form = new FormData();
  form.append("manifest", JSON.stringify(manifest));
  form.append(
    "file",
    new Blob([archiveBytes], { type: "application/zip" }),
    path.basename(archivePath),
  );

  const url = `${apiUrl}/api/marketplace/listings/${encodeURIComponent(listingId)}/versions/upload`;
  const response = await fetch(url, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "x-organization-id": organizationId,
    },
    body: form,
  });

  const text = await response.text();
  let body;
  try {
    body = text ? JSON.parse(text) : null;
  } catch {
    body = text;
  }

  if (!response.ok) {
    const message = typeof body === "string" ? body : JSON.stringify(body);
    throw new Error(`submit failed with HTTP ${response.status}: ${message}`);
  }

  return {
    status: response.status,
    url,
    body,
  };
}

async function readManifest(manifestPath) {
  const raw = await fs.readFile(manifestPath, "utf8");
  try {
    return JSON.parse(raw);
  } catch (error) {
    throw new Error(`manifest JSON is invalid at ${manifestPath}: ${error.message}`);
  }
}

function buildValidationReport(input) {
  const errors = [];
  const warnings = [];
  const findings = [];

  validateManifestShape(input.manifest, errors, warnings);
  validateProductContracts(input.manifest, errors, warnings);
  validateFileTree(input.manifest, input.entries, errors, warnings, findings);
  validateCompatibility(input.manifest, input.organizationPlan, warnings);
  scanSecurity(input.manifest, input.entries, findings);

  if (input.artifactBytes) {
    if (input.artifactBytes.length === 0) {
      errors.push("package artifact must not be empty");
    }
    if (input.artifactBytes.length > MAX_PACKAGE_BYTES) {
      errors.push("package artifact exceeds the Marketplace size limit");
    }
    if (input.artifactFileName && !input.artifactFileName.toLowerCase().endsWith(".zip")) {
      warnings.push("artifact file name should use the .zip extension");
    }
  }

  const riskLevel = findings.reduce(
    (level, finding) => RISK_RANK[finding.severity] > RISK_RANK[level] ? finding.severity : level,
    "low",
  );
  const blocked = errors.length > 0 || RISK_RANK[riskLevel] >= RISK_RANK.high;

  return {
    ok: !blocked,
    blocked,
    schema_version: MANIFEST_SCHEMA_VERSION,
    manifest_path: input.manifestPath,
    package_root: input.packageRoot,
    manifest: input.manifest,
    summary: {
      file_count: input.entries.length,
      uncompressed_size_bytes: input.entries.reduce((sum, entry) => sum + entry.uncompressed_size, 0),
      risk_level: riskLevel,
      errors: errors.length,
      warnings: warnings.length,
      findings: findings.length,
    },
    errors,
    warnings,
    security_findings: findings,
    file_tree: input.entries.map((entry) => entry.path).sort(),
  };
}

function validateManifestShape(manifest, errors, warnings) {
  if (!isPlainObject(manifest)) {
    errors.push("manifest must be a JSON object");
    return;
  }

  requireString(manifest, "manifest_version", errors, MANIFEST_SCHEMA_VERSION);
  requireString(manifest, "name", errors);
  const version = requireString(manifest, "version", errors);
  if (version && !isSemver(version)) errors.push("version must use semantic version format");

  const type = requireString(manifest, "type", errors);
  if (type && !PRODUCT_TYPES.has(type)) errors.push(`unsupported product type '${type}'`);

  validatePermissions(manifest.permissions, errors);
  validateCompatibilityShape(manifest.compatibility, errors, warnings);
  validateEntryPointsShape(manifest.entry_points, errors);
  validateAssetsShape(manifest.assets, errors);
  validateDependencies(manifest.dependencies, errors, warnings);
}

function validatePermissions(value, errors) {
  if (!Array.isArray(value)) {
    errors.push("permissions is required and must be an array");
    return;
  }
  const seen = new Set();
  for (const item of value) {
    if (typeof item !== "string" || item.trim() === "") {
      errors.push("permissions must contain only non-empty strings");
      continue;
    }
    const permission = item.trim();
    if (!PERMISSIONS.has(permission)) errors.push(`unsupported permission '${permission}'`);
    if (seen.has(permission)) errors.push(`duplicate permission '${permission}'`);
    seen.add(permission);
  }
}

function validateCompatibilityShape(value, errors, warnings) {
  if (!isPlainObject(value)) {
    errors.push("compatibility is required and must be an object");
    return;
  }
  const minVersion = requireString(value, "min_zinhar_version", errors);
  if (minVersion && !isSemver(minVersion)) {
    errors.push("compatibility.min_zinhar_version must use semantic version format");
  }
  if (value.max_zinhar_version !== undefined) {
    if (typeof value.max_zinhar_version !== "string" || value.max_zinhar_version.trim() === "") {
      errors.push("compatibility.max_zinhar_version must be a non-empty string");
    } else if (!isSemver(value.max_zinhar_version.trim())) {
      errors.push("compatibility.max_zinhar_version must use semantic version format");
    }
  }
  if (Array.isArray(value.required_features)) {
    for (const feature of value.required_features) {
      if (!SUPPORTED_FEATURES.has(feature)) {
        warnings.push(`compatibility.required_features includes unsupported feature '${feature}'`);
      }
    }
  }
}

function validateEntryPointsShape(value, errors) {
  if (!isPlainObject(value)) {
    errors.push("entry_points is required and must be an object");
    return;
  }
  if (Object.keys(value).length === 0) errors.push("entry_points must not be empty");
  for (const [key, entryPath] of Object.entries(value)) {
    if (typeof entryPath !== "string" || entryPath.trim() === "") {
      errors.push(`entry point '${key}' must be a package path string`);
      continue;
    }
    const normalized = normalizePackagePath(entryPath);
    if (!normalized.ok) errors.push(normalized.error);
  }
}

function validateAssetsShape(value, errors) {
  if (!Array.isArray(value)) {
    errors.push("assets is required and must be an array");
    return;
  }
  if (value.length > MAX_DECLARED_ASSETS) {
    errors.push(`manifest declares more than ${MAX_DECLARED_ASSETS} assets`);
  }
  for (const asset of value) {
    if (typeof asset !== "string" || asset.trim() === "") {
      errors.push("manifest assets must contain only non-empty strings");
      continue;
    }
    const normalized = normalizePackagePath(asset);
    if (!normalized.ok) errors.push(normalized.error);
  }
}

function validateDependencies(value, errors, warnings) {
  if (value === undefined) return;
  if (!isPlainObject(value)) {
    errors.push("dependencies must be an object when provided");
    return;
  }
  const entries = Object.entries(value);
  if (entries.length > MAX_MANIFEST_DEPENDENCIES) {
    errors.push(`manifest declares more than ${MAX_MANIFEST_DEPENDENCIES} dependencies`);
  }
  for (const [name, declaration] of entries) {
    if (name.trim() === "") errors.push("dependency names must not be empty");
    if (typeof declaration === "string") {
      const version = declaration.trim();
      if (!version) errors.push(`dependency '${name}' version must not be empty`);
      if (version === "*" || version.toLowerCase() === "latest") {
        warnings.push(`dependency '${name}' uses an unpinned version`);
      }
    } else if (isPlainObject(declaration)) {
      if (!("version" in declaration) && !("source" in declaration)) {
        warnings.push(`dependency '${name}' should declare a version or source`);
      }
    } else {
      errors.push(`dependency '${name}' must be a string or metadata object`);
    }
  }
}

function validateProductContracts(manifest, errors, warnings) {
  if (!isPlainObject(manifest)) return;
  if (manifest.type === "component_pack") {
    if (manifest.components !== undefined) validateComponents(manifest.components, errors);
    else warnings.push("component_pack manifests should include a components array for Phase 8 runtime use");
  }
  if (manifest.type === "design_template") {
    if (manifest.template !== undefined) validateTemplate(manifest.template, errors);
    else warnings.push("design_template manifests should include a template object for Phase 8 imports");
  }
  if (manifest.type === "integration_plugin") {
    if (manifest.hooks !== undefined) validateHooks(manifest.hooks, errors);
    else warnings.push("integration_plugin manifests should include public hook declarations");
  }
}

function validateComponents(value, errors) {
  if (!Array.isArray(value)) {
    errors.push("manifest components must be an array");
    return;
  }
  for (const item of value) {
    if (!isPlainObject(item)) {
      errors.push("manifest components must be objects");
      continue;
    }
    validateSlugField(item, "key", errors);
    validateTextField(item, "name", errors);
    validateSlugField(item, "category", errors);
    if (item.props_schema !== undefined && !isPlainObject(item.props_schema)) {
      errors.push(`component '${item.key ?? "unknown"}' props_schema must be an object`);
    }
  }
}

function validateTemplate(value, errors) {
  if (!isPlainObject(value)) {
    errors.push("manifest template must be an object");
    return;
  }
  if (value.key !== undefined) validateSlugField(value, "key", errors);
  if (!isPlainObject(value.page_json)) errors.push("manifest template.page_json must be an object");
  if (value.assets !== undefined && !Array.isArray(value.assets)) {
    errors.push("manifest template.assets must be an array when provided");
  }
}

function validateHooks(value, errors) {
  if (!Array.isArray(value)) {
    errors.push("manifest hooks must be an array");
    return;
  }
  for (const item of value) {
    if (!isPlainObject(item)) {
      errors.push("manifest hooks must be objects");
      continue;
    }
    const key = validateSlugField(item, "key", errors);
    const type = validateTextField(item, "type", errors);
    validateTextField(item, "label", errors);
    if (type && !PUBLIC_HOOK_TYPES.has(type)) {
      errors.push(`hook '${key ?? "unknown"}' uses unsupported public hook type '${type}'`);
    }
    if (item.config !== undefined && !isPlainObject(item.config)) {
      errors.push(`hook '${key ?? "unknown"}' config must be an object`);
    }
  }
}

function validateFileTree(manifest, entries, errors, warnings, findings) {
  if (entries.length === 0) errors.push("package must contain at least one file");
  if (entries.length > MAX_PACKAGE_FILES) errors.push(`package contains more than ${MAX_PACKAGE_FILES} files`);
  const totalSize = entries.reduce((sum, entry) => sum + entry.uncompressed_size, 0);
  if (totalSize > MAX_UNCOMPRESSED_BYTES) {
    errors.push(`package uncompressed size exceeds ${MAX_UNCOMPRESSED_BYTES} bytes`);
  }

  const paths = new Set();
  for (const entry of entries) {
    const normalized = normalizePackagePath(entry.path);
    if (!normalized.ok) {
      errors.push(normalized.error);
      continue;
    }
    if (paths.has(normalized.path)) errors.push(`duplicate package path '${normalized.path}'`);
    paths.add(normalized.path);
    scanPathForSecurity(normalized.path, findings);
  }

  if (isPlainObject(manifest?.entry_points)) {
    for (const [key, entryPath] of Object.entries(manifest.entry_points)) {
      const normalized = normalizePackagePath(entryPath);
      if (normalized.ok && !paths.has(normalized.path)) {
        errors.push(`entry point '${key}' path '${normalized.path}' is missing from package`);
      }
    }
  }

  if (Array.isArray(manifest?.assets)) {
    for (const asset of manifest.assets) {
      if (typeof asset !== "string") continue;
      const normalized = normalizePackagePath(asset);
      if (normalized.ok && !paths.has(normalized.path)) {
        errors.push(`manifest asset '${normalized.path}' is missing from package`);
      }
    }
  }

  if (!paths.has("manifest.json")) {
    warnings.push("package directory should include manifest.json at its root");
  }
}

function validateCompatibility(manifest, organizationPlan, warnings) {
  const compatibility = manifest?.compatibility;
  if (!isPlainObject(compatibility)) return;
  const requiredPlan = compatibility.required_plan ?? "free";
  const requiredFeatures = Array.isArray(compatibility.required_features)
    ? compatibility.required_features
    : [];
  if (comparePlan(organizationPlan, requiredPlan) < 0) {
    warnings.push(`requires plan '${requiredPlan}', current local validation plan is '${organizationPlan}'`);
  }
  for (const feature of requiredFeatures) {
    if (!SUPPORTED_FEATURES.has(feature)) warnings.push(`requires unsupported feature '${feature}'`);
  }
}

function scanSecurity(manifest, entries, findings) {
  if (manifest?.type === "backend_extension") {
    findings.push({
      severity: "high",
      code: "backend-extension-sandbox-missing",
      message: "backend extensions are blocked until sandbox execution is available",
      path: null,
    });
  }

  if (Array.isArray(manifest?.permissions)) {
    for (const permission of manifest.permissions) {
      if (permission === "external_network.request") {
        findings.push({
          severity: "high",
          code: "external-network-permission",
          message: "manifest requests external network access",
          path: null,
        });
      } else if (permission === "webhook.send" || permission === "settings.read") {
        findings.push({
          severity: "medium",
          code: "sensitive-permission",
          message: `manifest requests sensitive permission '${permission}'`,
          path: null,
        });
      } else if (typeof permission === "string" && permission.endsWith(".write")) {
        findings.push({
          severity: "medium",
          code: "write-permission",
          message: `manifest requests write permission '${permission}'`,
          path: null,
        });
      }
    }
  }

  if (isPlainObject(manifest?.dependencies)) {
    for (const [name, declaration] of Object.entries(manifest.dependencies)) {
      const raw = dependencyDeclarationString(declaration).toLowerCase();
      if (raw === "*" || raw === "latest" || raw.includes(" latest")) {
        findings.push({ severity: "medium", code: "unpinned-dependency", message: `dependency '${name}' is not pinned`, path: null });
      }
      if (raw.startsWith("http://") || raw.startsWith("https://") || raw.startsWith("git://") || raw.startsWith("ssh://") || raw.includes("github:") || raw.includes("git+")) {
        findings.push({ severity: "high", code: "remote-dependency-source", message: `dependency '${name}' uses a remote source`, path: null });
      }
    }
  }

  scanManifestStrings(manifest, "$", findings);
  for (const entry of entries) scanPathForSecurity(entry.path, findings);
}

function scanPathForSecurity(entryPath, findings) {
  const lowerPath = entryPath.toLowerCase();
  const fileName = lowerPath.split("/").at(-1);
  const forbiddenNames = new Set([
    ".env",
    ".env.local",
    ".npmrc",
    ".pypirc",
    "id_rsa",
    "id_dsa",
    "id_ed25519",
    "credentials",
    "credentials.json",
    "service-account.json",
  ]);
  if (
    forbiddenNames.has(fileName) ||
    lowerPath.endsWith(".pem") ||
    lowerPath.endsWith(".key") ||
    lowerPath.endsWith(".p12") ||
    lowerPath.endsWith(".pfx") ||
    lowerPath.endsWith(".kube/config") ||
    lowerPath.endsWith(".aws/credentials")
  ) {
    findings.push({
      severity: "critical",
      code: "forbidden-secret-file",
      message: "package contains a forbidden secret or credential file",
      path: entryPath,
    });
  }

  if ([".exe", ".dll", ".dylib", ".so", ".jar", ".bat", ".cmd", ".ps1", ".sh"].some((extension) => lowerPath.endsWith(extension))) {
    findings.push({
      severity: "high",
      code: "executable-artifact",
      message: "package contains executable or shell-script artifacts",
      path: entryPath,
    });
  }
}

function scanManifestStrings(value, jsonPath, findings) {
  if (typeof value === "string") {
    const lower = value.toLowerCase();
    if (lower.startsWith("http://") || lower.startsWith("https://") || lower.includes("://cdn.")) {
      findings.push({
        severity: lower.endsWith(".js") || jsonPath.includes("script") ? "high" : "medium",
        code: "external-reference",
        message: `manifest contains external reference at ${jsonPath}`,
        path: null,
      });
    }
    return;
  }
  if (Array.isArray(value)) {
    value.forEach((item, index) => scanManifestStrings(item, `${jsonPath}[${index}]`, findings));
    return;
  }
  if (isPlainObject(value)) {
    for (const [key, item] of Object.entries(value)) scanManifestStrings(item, `${jsonPath}.${key}`, findings);
  }
}

async function collectDirectoryEntries(root, options = {}) {
  const exclude = new Set((options.exclude ?? []).map((item) => path.resolve(item)));
  const entries = [];
  await walk(root, root, entries, exclude);
  return entries.sort((left, right) => left.path.localeCompare(right.path));
}

async function walk(root, current, entries, exclude) {
  const children = await fs.readdir(current, { withFileTypes: true });
  for (const child of children) {
    const absolute = path.join(current, child.name);
    if (exclude.has(path.resolve(absolute))) continue;
    if (child.isDirectory()) {
      if ([".git", "node_modules", "marketplace-dist", ".zinhar"].includes(child.name)) continue;
      await walk(root, absolute, entries, exclude);
      continue;
    }
    if (!child.isFile()) continue;
    const relative = path.relative(root, absolute).split(path.sep).join("/");
    const normalized = normalizePackagePath(relative);
    if (!normalized.ok) {
      entries.push({ path: relative, absolute_path: absolute, uncompressed_size: 0 });
      continue;
    }
    const stat = await fs.stat(absolute);
    entries.push({ path: normalized.path, absolute_path: absolute, uncompressed_size: stat.size });
  }
}

async function resolveOutputPath(packageDir, manifest, options) {
  const fileName = `${slugify(manifest.name ?? "marketplace-package")}-${manifest.version ?? "0.0.0"}.zip`;
  if (options.out) {
    const out = path.resolve(options.out);
    if (out.toLowerCase().endsWith(".zip")) return out;
    return path.join(out, fileName);
  }
  return path.join(process.cwd(), "marketplace-dist", fileName);
}

async function createZip(root, entries) {
  const fileParts = [];
  const centralParts = [];
  let offset = 0;
  for (const entry of entries) {
    const name = Buffer.from(entry.path, "utf8");
    const data = await fs.readFile(entry.absolute_path);
    const compressed = deflateRawSync(data);
    const crc = crc32(data);
    const localHeader = Buffer.alloc(30);
    localHeader.writeUInt32LE(0x04034b50, 0);
    localHeader.writeUInt16LE(20, 4);
    localHeader.writeUInt16LE(0x0800, 6);
    localHeader.writeUInt16LE(8, 8);
    localHeader.writeUInt16LE(0, 10);
    localHeader.writeUInt16LE(0, 12);
    localHeader.writeUInt32LE(crc, 14);
    localHeader.writeUInt32LE(compressed.length, 18);
    localHeader.writeUInt32LE(data.length, 22);
    localHeader.writeUInt16LE(name.length, 26);
    localHeader.writeUInt16LE(0, 28);
    fileParts.push(localHeader, name, compressed);

    const centralHeader = Buffer.alloc(46);
    centralHeader.writeUInt32LE(0x02014b50, 0);
    centralHeader.writeUInt16LE(20, 4);
    centralHeader.writeUInt16LE(20, 6);
    centralHeader.writeUInt16LE(0x0800, 8);
    centralHeader.writeUInt16LE(8, 10);
    centralHeader.writeUInt16LE(0, 12);
    centralHeader.writeUInt16LE(0, 14);
    centralHeader.writeUInt32LE(crc, 16);
    centralHeader.writeUInt32LE(compressed.length, 20);
    centralHeader.writeUInt32LE(data.length, 24);
    centralHeader.writeUInt16LE(name.length, 28);
    centralHeader.writeUInt16LE(0, 30);
    centralHeader.writeUInt16LE(0, 32);
    centralHeader.writeUInt16LE(0, 34);
    centralHeader.writeUInt16LE(0, 36);
    centralHeader.writeUInt32LE(0, 38);
    centralHeader.writeUInt32LE(offset, 42);
    centralParts.push(centralHeader, name);
    offset += localHeader.length + name.length + compressed.length;
  }
  const centralDirectoryOffset = offset;
  const centralDirectory = Buffer.concat(centralParts);
  const end = Buffer.alloc(22);
  end.writeUInt32LE(0x06054b50, 0);
  end.writeUInt16LE(0, 4);
  end.writeUInt16LE(0, 6);
  end.writeUInt16LE(entries.length, 8);
  end.writeUInt16LE(entries.length, 10);
  end.writeUInt32LE(centralDirectory.length, 12);
  end.writeUInt32LE(centralDirectoryOffset, 16);
  end.writeUInt16LE(0, 20);
  return Buffer.concat([...fileParts, centralDirectory, end]);
}

function parseZipEntries(bytes) {
  const buffer = Buffer.from(bytes);
  const eocdOffset = findEndOfCentralDirectory(buffer);
  if (eocdOffset < 0) throw new Error("package ZIP central directory was not found");
  const entryCount = buffer.readUInt16LE(eocdOffset + 10);
  const centralDirectoryOffset = buffer.readUInt32LE(eocdOffset + 16);
  const entries = [];
  let offset = centralDirectoryOffset;
  for (let index = 0; index < entryCount; index += 1) {
    if (buffer.readUInt32LE(offset) !== 0x02014b50) throw new Error("package ZIP central directory entry is invalid");
    const compressedSize = buffer.readUInt32LE(offset + 20);
    const uncompressedSize = buffer.readUInt32LE(offset + 24);
    const nameLength = buffer.readUInt16LE(offset + 28);
    const extraLength = buffer.readUInt16LE(offset + 30);
    const commentLength = buffer.readUInt16LE(offset + 32);
    const name = buffer.subarray(offset + 46, offset + 46 + nameLength).toString("utf8");
    if (!name.endsWith("/")) {
      entries.push({ path: name, uncompressed_size: uncompressedSize, compressed_size: compressedSize });
    }
    offset += 46 + nameLength + extraLength + commentLength;
  }
  return entries;
}

function findEndOfCentralDirectory(buffer) {
  const min = Math.max(0, buffer.length - 65_557);
  for (let offset = buffer.length - 22; offset >= min; offset -= 1) {
    if (buffer.readUInt32LE(offset) === 0x06054b50) return offset;
  }
  return -1;
}

const CRC_TABLE = (() => {
  const table = new Uint32Array(256);
  for (let index = 0; index < 256; index += 1) {
    let value = index;
    for (let bit = 0; bit < 8; bit += 1) {
      value = value & 1 ? 0xedb88320 ^ (value >>> 1) : value >>> 1;
    }
    table[index] = value >>> 0;
  }
  return table;
})();

function crc32(buffer) {
  let crc = 0xffffffff;
  for (const byte of buffer) crc = CRC_TABLE[(crc ^ byte) & 0xff] ^ (crc >>> 8);
  return (crc ^ 0xffffffff) >>> 0;
}

function normalizePackagePath(value) {
  if (typeof value !== "string") return { ok: false, error: "package path must be a string" };
  const normalized = value.trim().replaceAll("\\", "/");
  if (!normalized) return { ok: false, error: "package path must not be empty" };
  if (normalized.startsWith("/") || /^[a-z]:/i.test(normalized)) {
    return { ok: false, error: `package path '${value}' must be relative` };
  }
  const parts = normalized.split("/");
  if (parts.some((part) => part === "" || part === "." || part === "..")) {
    return { ok: false, error: `package path '${value}' must not contain empty, current, or parent segments` };
  }
  return { ok: true, path: parts.join("/") };
}

function requireString(object, field, errors, expected) {
  const value = object[field];
  if (typeof value !== "string" || value.trim() === "") {
    errors.push(`${field} is required and must be a non-empty string`);
    return null;
  }
  const trimmed = value.trim();
  if (expected && trimmed !== expected) errors.push(`${field} must be '${expected}'`);
  return trimmed;
}

function validateTextField(object, field, errors) {
  const value = object[field];
  if (typeof value !== "string" || value.trim() === "") {
    errors.push(`manifest field '${field}' is required`);
    return null;
  }
  return value.trim();
}

function validateSlugField(object, field, errors) {
  const value = validateTextField(object, field, errors);
  if (!value) return null;
  if (!isSafeSlug(value)) errors.push(`manifest field '${field}' must be a lowercase slug`);
  return value;
}

function isSafeSlug(value) {
  return /^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(value);
}

function isSemver(value) {
  const buildIndex = value.indexOf("+");
  const coreAndPre = buildIndex >= 0 ? value.slice(0, buildIndex) : value;
  const build = buildIndex >= 0 ? value.slice(buildIndex + 1) : null;
  if (build !== null && !isSemverIdentifierList(build, true)) return false;
  const prereleaseIndex = coreAndPre.indexOf("-");
  const core = prereleaseIndex >= 0 ? coreAndPre.slice(0, prereleaseIndex) : coreAndPre;
  const prerelease = prereleaseIndex >= 0 ? coreAndPre.slice(prereleaseIndex + 1) : null;
  if (prerelease !== null && !isSemverIdentifierList(prerelease, false)) return false;
  const parts = core.split(".");
  return parts.length === 3 && parts.every(isSemverNumericIdentifier);
}

function isSemverNumericIdentifier(value) {
  return /^[0-9]+$/.test(value) && (value === "0" || !value.startsWith("0"));
}

function isSemverIdentifierList(value, allowLeadingZeroNumeric) {
  return typeof value === "string" && value.length > 0 && value.split(".").every((part) => (
    part.length > 0 &&
    /^[0-9A-Za-z-]+$/.test(part) &&
    (allowLeadingZeroNumeric || !/^[0-9]+$/.test(part) || isSemverNumericIdentifier(part))
  ));
}

function dependencyDeclarationString(value) {
  if (typeof value === "string") return value.trim();
  if (isPlainObject(value)) return [value.version, value.source].filter((item) => typeof item === "string").join(" ");
  return "";
}

function comparePlan(current, required) {
  const rank = { free: 0, pro: 1, enterprise: 2 };
  return (rank[current] ?? 0) - (rank[required] ?? 0);
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function slugify(value) {
  const slug = String(value)
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .replace(/--+/g, "-");
  return slug || "marketplace-package";
}

async function exists(filePath) {
  try {
    await fs.access(filePath);
    return true;
  } catch {
    return false;
  }
}

function printReport(report, options = {}) {
  if (options.json) {
    console.log(JSON.stringify(report, null, 2));
    return;
  }
  const status = report.ok ? "PASS" : "BLOCKED";
  console.log(`Validation ${status}: ${report.manifest.name ?? "Marketplace package"} ${report.manifest.version ?? ""}`.trim());
  console.log(`Files: ${report.summary.file_count}; risk: ${report.summary.risk_level}; errors: ${report.summary.errors}; warnings: ${report.summary.warnings}; findings: ${report.summary.findings}`);
  for (const error of report.errors) console.log(`ERROR ${error}`);
  for (const warning of report.warnings) console.log(`WARN  ${warning}`);
  for (const finding of report.security_findings) {
    console.log(`${finding.severity.toUpperCase()} ${finding.code}: ${finding.message}${finding.path ? ` (${finding.path})` : ""}`);
  }
}

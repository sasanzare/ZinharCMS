import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repositoryRoot = dirname(dirname(fileURLToPath(import.meta.url)));

function read(relativePath) {
  return readFileSync(join(repositoryRoot, relativePath), "utf8");
}

function requireMatch(relativePath, pattern, description) {
  const match = read(relativePath).match(pattern);
  if (!match) {
    throw new Error(`Could not read ${description} from ${relativePath}`);
  }
  return match[1];
}

const rootPackage = JSON.parse(read("package.json"));
const frontendPackage = JSON.parse(read("frontend/package.json"));
const frontendLock = JSON.parse(read("frontend/package-lock.json"));

const versions = new Map([
  ["root package", rootPackage.version],
  ["frontend package", frontendPackage.version],
  ["frontend lockfile", frontendLock.version],
  ["frontend lockfile root package", frontendLock.packages?.[""]?.version],
  [
    "backend crate",
    requireMatch(
      "backend/Cargo.toml",
      /^\s*version\s*=\s*"([^"]+)"/m,
      "backend crate version",
    ),
  ],
  [
    "backend lockfile",
    requireMatch(
      "backend/Cargo.lock",
      /\[\[package\]\]\s+name = "cms-backend"\s+version = "([^"]+)"/,
      "backend lockfile version",
    ),
  ],
  [
    "Marketplace compatibility runtime",
    requireMatch(
      "backend/src/services/marketplace_validation.rs",
      /CURRENT_ZINHAR_VERSION:\s*&str\s*=\s*"([^"]+)"/,
      "Marketplace compatibility runtime version",
    ),
  ],
  [
    "dashboard fallback",
    requireMatch(
      "frontend/src/pages/DashboardPage.tsx",
      /health\?\.version\s*\?\?\s*"([^"]+)"/,
      "dashboard fallback version",
    ),
  ],
]);

const expectedVersion = rootPackage.version;
const invalidSources = [...versions.entries()].filter(
  ([, version]) => version !== expectedVersion,
);

if (!/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/.test(expectedVersion)) {
  throw new Error(
    `Root package version must be a stable semantic version, received ${expectedVersion}`,
  );
}

if (invalidSources.length > 0) {
  const details = invalidSources
    .map(([source, version]) => `${source}: ${version ?? "missing"}`)
    .join("\n");
  throw new Error(
    `Release version sources do not match ${expectedVersion}:\n${details}`,
  );
}

console.log(
  `Release version ${expectedVersion} is consistent across ${versions.size} sources.`,
);

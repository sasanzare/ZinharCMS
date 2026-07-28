import { readdirSync, readFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import ts from "typescript";

const frontendRoot = resolve(fileURLToPath(new URL("..", import.meta.url)));
const sourceRoot = resolve(frontendRoot, "src");
const approvedSink = "components/SafeRichText.tsx";
const findings = [];
let approvedSinkCount = 0;

for (const file of sourceFiles(sourceRoot)) {
  const relativePath = relative(sourceRoot, file).replaceAll("\\", "/");
  const source = ts.createSourceFile(
    file,
    readFileSync(file, "utf8"),
    ts.ScriptTarget.Latest,
    true,
    file.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS,
  );
  inspect(source, relativePath);
}

if (approvedSinkCount !== 1) {
  findings.push(
    `${approvedSink}: expected exactly one approved dangerouslySetInnerHTML boundary, found ${approvedSinkCount}`,
  );
}

if (findings.length > 0) {
  process.stderr.write(`${findings.join("\n")}\n`);
  process.exitCode = 1;
} else {
  process.stdout.write("HTML sink policy passed with one approved boundary.\n");
}

function sourceFiles(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = resolve(directory, entry.name);
    if (entry.isDirectory()) return sourceFiles(path);
    if (
      !/\.(ts|tsx)$/u.test(entry.name) ||
      /\.(test|spec)\.(ts|tsx)$/u.test(entry.name)
    ) {
      return [];
    }
    return [path];
  });
}

function inspect(source, relativePath) {
  const visit = (node) => {
    if (
      ts.isJsxAttribute(node) &&
      node.name.getText(source) === "dangerouslySetInnerHTML"
    ) {
      if (relativePath === approvedSink) approvedSinkCount += 1;
      else report(node, source, relativePath, "unapproved dangerouslySetInnerHTML");
    }
    if (
      ts.isPropertyAccessExpression(node) &&
      ["innerHTML", "outerHTML"].includes(node.name.text)
    ) {
      report(node, source, relativePath, `direct ${node.name.text} access`);
    }
    if (
      ts.isCallExpression(node) &&
      ts.isPropertyAccessExpression(node.expression) &&
      ["insertAdjacentHTML", "write"].includes(node.expression.name.text)
    ) {
      report(node, source, relativePath, "HTML-capable DOM call");
    }
    if (
      ts.isCallExpression(node) &&
      ts.isIdentifier(node.expression) &&
      node.expression.text === "eval"
    ) {
      report(node, source, relativePath, "eval call");
    }
    if (
      ts.isNewExpression(node) &&
      ts.isIdentifier(node.expression) &&
      node.expression.text === "Function"
    ) {
      report(node, source, relativePath, "Function constructor");
    }
    if (
      ts.isJsxAttribute(node) &&
      node.name.getText(source).toLowerCase() === "srcdoc"
    ) {
      report(node, source, relativePath, "srcDoc attribute");
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
}

function report(node, source, relativePath, message) {
  const location = source.getLineAndCharacterOfPosition(node.getStart(source));
  findings.push(`${relativePath}:${location.line + 1}: ${message}`);
}

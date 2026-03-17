#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import url from 'node:url';

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..');

const sourcePath = path.join(repoRoot, 'docs', 'documentation-catalog.json');
const retiredDocumentationPaths = new Set([
  'docs/status.md',
  'docs/architecture.md',
]);
const cliOutputPath = path.join(
  repoRoot,
  'crates',
  'vel-cli',
  'src',
  'commands',
  'docs_catalog.generated.json',
);
const webOutputPath = path.join(
  repoRoot,
  'clients',
  'web',
  'src',
  'data',
  'documentationCatalog.generated.ts',
);
const appleOutputPath = path.join(
  repoRoot,
  'clients',
  'apple',
  'VelAPI',
  'Sources',
  'VelAPI',
  'VelDocumentation.swift',
);

function readSource() {
  const raw = fs.readFileSync(sourcePath, 'utf8');
  const parsed = JSON.parse(raw);
  if (!parsed || !Array.isArray(parsed.entries)) {
    throw new Error('docs/documentation-catalog.json is missing an entries array');
  }
  for (const entry of parsed.entries) {
    if (!entry || typeof entry.path !== 'string') {
      throw new Error('docs/documentation-catalog.json entry is missing required path');
    }
    if (retiredDocumentationPaths.has(entry.path)) {
      throw new Error(`docs/documentation-catalog.json contains retired documentation path: ${entry.path}`);
    }
  }
  return parsed.entries;
}

function forSurface(entries, surface) {
  return entries.filter((entry) => Array.isArray(entry.surfaces) && entry.surfaces.includes(surface));
}

function generateCli(entries) {
  return `${JSON.stringify(
    entries.map(({ category, title, path: entryPath, description }) => ({
      category,
      title,
      path: entryPath,
      description,
    })),
    null,
    2,
  )}\n`;
}

function generateWeb(entries) {
  const tupleLine = (entry) => {
    const parts = [entry.title, entry.path, entry.description].map((value) => JSON.stringify(value));
    return `  [${parts.join(', ')}],`;
  };
  const core = entries
    .filter((entry) => entry.category === 'core')
    .map(tupleLine)
    .join('\n');
  const user = entries
    .filter((entry) => entry.category === 'user')
    .map(tupleLine)
    .join('\n');

  return `// GENERATED FILE. DO NOT EDIT.
// Source: docs/documentation-catalog.json

export type DocumentationTuple = [string, string, string];

export const CORE_DOCUMENTATION_ENTRIES: DocumentationTuple[] = [
${core}
];

export const USER_DOCUMENTATION_ENTRIES: DocumentationTuple[] = [
${user}
];
`;
}

function generateApple(entries) {
  const swiftLiteral = (value) => String(value)
    .replace(/\\/g, '\\\\')
    .replace(/"/g, '\\"')
    .replace(/\n/g, '\\n');
  const mapEntry = (entry) => `        .init(
            id: "${swiftLiteral(entry.id)}",
            category: "${swiftLiteral(entry.category)}",
            title: "${swiftLiteral(entry.title)}",
            path: "${swiftLiteral(entry.path)}",
            summary: "${swiftLiteral(entry.description)}"
        )`;
  const core = entries.filter((entry) => entry.category === 'core').map(mapEntry).join(',\n');
  const user = entries.filter((entry) => entry.category === 'user').map(mapEntry).join(',\n');

  return `// GENERATED FILE. DO NOT EDIT.
// Source: docs/documentation-catalog.json
import Foundation

public struct VelDocumentationReference: Identifiable, Sendable {
    public let id: String
    public let category: String
    public let title: String
    public let path: String
    public let summary: String

    public init(
        id: String,
        category: String,
        title: String,
        path: String,
        summary: String
    ) {
        self.id = id
        self.category = category
        self.title = title
        self.path = path
        self.summary = summary
    }
}

public enum VelDocumentationCatalog {
    public static let core: [VelDocumentationReference] = [
${core}
    ]

    public static let user: [VelDocumentationReference] = [
${user}
    ]
}
`;
}

function writeIfChanged(targetPath, content) {
  const current = fs.existsSync(targetPath) ? fs.readFileSync(targetPath, 'utf8') : null;
  if (current === content) {
    return false;
  }
  fs.writeFileSync(targetPath, content);
  return true;
}

function main() {
  const checkMode = process.argv.includes('--check');
  const entries = readSource();

  const cliEntries = forSurface(entries, 'cli');
  const webEntries = forSurface(entries, 'web');
  const appleEntries = forSurface(entries, 'apple');

  const outputs = [
    [cliOutputPath, generateCli(cliEntries)],
    [webOutputPath, generateWeb(webEntries)],
    [appleOutputPath, generateApple(appleEntries)],
  ];

  let changed = false;
  for (const [outputPath, content] of outputs) {
    if (checkMode) {
      const current = fs.existsSync(outputPath) ? fs.readFileSync(outputPath, 'utf8') : null;
      if (current !== content) {
        changed = true;
      }
      continue;
    }
    changed = writeIfChanged(outputPath, content) || changed;
  }

  if (checkMode && changed) {
    console.error('documentation catalog artifacts are out of date. Run: node scripts/sync-documentation-catalog.mjs');
    process.exit(1);
  }

  if (!checkMode && changed) {
    console.log('documentation catalog artifacts updated');
  } else if (!checkMode) {
    console.log('documentation catalog artifacts already up to date');
  }
}

main();

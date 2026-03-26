#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';

const repoRoot = process.cwd();
const commandsDir = path.join(repoRoot, 'crates/vel-cli/src/commands');
const clientFile = path.join(repoRoot, 'crates/vel-cli/src/client.rs');
const manifestPath = path.join(repoRoot, 'config/coverage/vel-cli-coverage-debt.json');

const args = new Set(process.argv.slice(2));
const checkMode = args.has('--check');
const jsonMode = args.has('--json');

function rel(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}

function readText(filePath) {
  return fs.readFileSync(filePath, 'utf8');
}

function hasLocalTests(filePath) {
  const text = readText(filePath);
  return text.includes('#[cfg(test)]')
    || text.includes('mod tests')
    || text.includes('#[test]')
    || text.includes('#[tokio::test]');
}

function ensureArray(value, label) {
  if (!Array.isArray(value)) {
    throw new Error(`${label} must be an array`);
  }
  return value;
}

function validateManifestEntry(entry, label) {
  for (const key of ['path', 'owner', 'reason', 'created', 'target_removal']) {
    if (typeof entry[key] !== 'string' || entry[key].trim() === '') {
      throw new Error(`${label} entry is missing required string field: ${key}`);
    }
  }
}

const manifest = JSON.parse(readText(manifestPath));
const integrationCovered = ensureArray(manifest.integration_covered, 'integration_covered');
const debtEntries = ensureArray(manifest.debt, 'debt');

for (const [index, entry] of integrationCovered.entries()) {
  validateManifestEntry(entry, `integration_covered[${index}]`);
}
for (const [index, entry] of debtEntries.entries()) {
  validateManifestEntry(entry, `debt[${index}]`);
}

const integrationMap = new Map(integrationCovered.map((entry) => [entry.path, entry]));
const debtMap = new Map(debtEntries.map((entry) => [entry.path, entry]));

for (const filePath of [...integrationMap.keys(), ...debtMap.keys()]) {
  const absolute = path.join(repoRoot, filePath);
  if (!fs.existsSync(absolute)) {
    throw new Error(`coverage manifest references missing file: ${filePath}`);
  }
}

for (const filePath of integrationMap.keys()) {
  if (debtMap.has(filePath)) {
    throw new Error(`coverage manifest path appears in both integration_covered and debt: ${filePath}`);
  }
}

const commandFiles = fs
  .readdirSync(commandsDir)
  .filter((name) => name.endsWith('.rs') && name !== 'mod.rs')
  .map((name) => path.join(commandsDir, name))
  .sort((a, b) => a.localeCompare(b));

const records = [];
for (const filePath of commandFiles) {
  const relativePath = rel(filePath);
  const hasTests = hasLocalTests(filePath);
  const integrationEntry = integrationMap.get(relativePath);
  const debtEntry = debtMap.get(relativePath);
  let coverageType = 'uncovered';
  if (hasTests) {
    coverageType = 'local_tests';
  } else if (integrationEntry) {
    coverageType = 'integration_tests';
  } else if (debtEntry) {
    coverageType = 'coverage_debt';
  }
  records.push({
    path: relativePath,
    coverage_type: coverageType,
    owner: integrationEntry?.owner ?? debtEntry?.owner ?? null,
    reason: integrationEntry?.reason ?? debtEntry?.reason ?? null,
    target_removal: debtEntry?.target_removal ?? null,
  });
}

const clientCovered = hasLocalTests(clientFile);
const uncovered = records.filter((record) => record.coverage_type === 'uncovered');
const summary = {
  total_command_files: records.length,
  local_test_covered: records.filter((record) => record.coverage_type === 'local_tests').length,
  integration_covered: records.filter((record) => record.coverage_type === 'integration_tests').length,
  debt_listed: records.filter((record) => record.coverage_type === 'coverage_debt').length,
  uncovered: uncovered.length,
  client_file_local_tests: clientCovered,
};

if (jsonMode) {
  console.log(JSON.stringify({ summary, records }, null, 2));
} else {
  console.log('check-cli-coverage: summary');
  console.log(`  commands total: ${summary.total_command_files}`);
  console.log(`  local test covered: ${summary.local_test_covered}`);
  console.log(`  integration covered: ${summary.integration_covered}`);
  console.log(`  debt listed: ${summary.debt_listed}`);
  console.log(`  uncovered: ${summary.uncovered}`);
  console.log(`  client.rs local tests: ${summary.client_file_local_tests ? 'yes' : 'no'}`);
  if (uncovered.length > 0) {
    console.log('check-cli-coverage: uncovered command files');
    for (const record of uncovered) {
      console.log(`  - ${record.path}`);
    }
  }
}

if (checkMode) {
  const failures = [];
  if (!clientCovered) {
    failures.push('crates/vel-cli/src/client.rs has no local test module');
  }
  for (const record of uncovered) {
    failures.push(`uncovered command file: ${record.path}`);
  }
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(`check-cli-coverage: ${failure}`);
    }
    process.exit(1);
  }
}

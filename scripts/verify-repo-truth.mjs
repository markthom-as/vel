#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';

const repoRoot = process.cwd();
const missing = [];

function ensure(condition, message) {
  if (!condition) {
    missing.push(message);
  }
}

function readFile(relativePath) {
  const filePath = path.join(repoRoot, relativePath);
  if (!fs.existsSync(filePath)) {
    ensure(false, `Missing file: ${relativePath}`);
    return '';
  }
  return fs.readFileSync(filePath, 'utf8');
}

const requiredFiles = [
  '.github/workflows/ci.yml',
  'Makefile',
  'README.md',
  'AGENTS.md',
  'docs/README.md',
  'docs/status.md',
  'docs/tickets/repo-feedback/README.md',
  'scripts/ci-smoke.sh',
  'scripts/bootstrap-demo-data.sh',
];

for (const file of requiredFiles) {
  if (!fs.existsSync(path.join(repoRoot, file))) {
    ensure(false, `Missing file: ${file}`);
  }
}

const readme = readFile('README.md');
const agents = readFile('AGENTS.md');
const docsReadme = readFile('docs/README.md');
const makefile = readFile('Makefile');
const status = readFile('docs/status.md');
const repoFeedbackReadme = readFile('docs/tickets/repo-feedback/README.md');

const requiredReadmeCommands = [
  'make build',
  'make dev',
  'make dev-api',
  'make dev-web',
  'make seed',
  'make ci',
  'make verify',
  'make smoke',
  'make bootstrap-demo-data',
];

for (const command of requiredReadmeCommands) {
  ensure(
    new RegExp(`\\\`${command}\\\``).test(readme),
    `README does not document command: ${command}`,
  );
}

const requiredMakeTargets = [
  'build',
  'build-api',
  'build-web',
  'dev',
  'dev-api',
  'dev-web',
  'seed',
  'install-web',
  'lint-web',
  'test',
  'test-api',
  'test-web',
  'verify',
  'verify-repo-truth',
  'bootstrap-demo-data',
  'ci',
  'smoke',
];

const makeTargets = new Set(
  makefile
    .split('\n')
    .map((line) => line.match(/^([a-zA-Z0-9_-]+):/))
    .filter(Boolean)
    .map((match) => match[1]),
);

for (const target of requiredMakeTargets) {
  ensure(makeTargets.has(target), `Makefile target missing: ${target}`);
}

ensure(
  fs.existsSync(path.join(repoRoot, 'docs', 'status.md')),
  'Missing docs/status.md',
);
ensure(readme.includes('docs/status.md'), 'README does not reference docs/status.md');
ensure(readme.includes('docs/README.md'), 'README does not reference docs/README.md');
ensure(agents.includes('docs/README.md'), 'AGENTS.md does not reference docs/README.md');
ensure(agents.includes('docs/status.md'), 'AGENTS.md does not reference docs/status.md');
ensure(status.includes('Chat interface'), 'docs/status.md does not mention chat interface status');
ensure(
  docsReadme.includes('## Doc Classes'),
  'docs/README.md does not define documentation classes',
);
ensure(
  docsReadme.includes('[status.md](status.md)'),
  'docs/README.md does not point to docs/status.md as current truth',
);
ensure(
  docsReadme.includes('[tickets/repo-feedback/README.md](tickets/repo-feedback/README.md)'),
  'docs/README.md does not point to repo-feedback as the active plan',
);
ensure(
  docsReadme.includes('[reviews/](reviews/)'),
  'docs/README.md does not classify reviews as historical context',
);
ensure(
  /## Current truth|### Current truth/.test(docsReadme),
  'docs/README.md does not mark a current truth section',
);
ensure(
  /## Active plan|### Active plan/.test(docsReadme),
  'docs/README.md does not mark an active plan section',
);
ensure(
  /## Historical review|### Historical review/.test(docsReadme),
  'docs/README.md does not mark a historical review section',
);
ensure(
  repoFeedbackReadme.includes('finish convergence before adding breadth'),
  'repo-feedback README lost the convergence priority statement',
);
ensure(
  status.includes('Repo-feedback follow-through'),
  'docs/status.md does not record repo-feedback follow-through',
);

if (missing.length > 0) {
  console.error('verify-repo-truth: failed');
  for (const item of missing) {
    console.error(`- ${item}`);
  }
  process.exit(1);
}

console.log('verify-repo-truth: ok');

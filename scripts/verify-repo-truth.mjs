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
  'docs/MASTER_PLAN.md',
  'docs/tickets/README.md',
  'docs/api/README.md',
  'docs/api/runtime.md',
  'docs/api/chat.md',
  'docs/user/README.md',
  'docs/templates/README.md',
  'docs/templates/spec-template.md',
  'docs/templates/agent-implementation-protocol.md',
  'docs/cognitive-agent-architecture/README.md',
  'docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md',
  'docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md',
  'docs/cognitive-agent-architecture/architecture/README.md',
  'docs/cognitive-agent-architecture/architecture/spec-draft.md',
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
const masterPlan = readFile('docs/MASTER_PLAN.md');
const ticketsReadme = readFile('docs/tickets/README.md');
const conceptSpec = readFile(
  'docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md',
);
const traitsSpec = readFile(
  'docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md',
);
const architecturePackReadme = readFile('docs/cognitive-agent-architecture/architecture/README.md');
const templatesReadme = readFile('docs/templates/README.md');
const specTemplate = readFile('docs/templates/spec-template.md');
const apiRuntime = readFile('docs/api/runtime.md');
const apiChat = readFile('docs/api/chat.md');
const makefile = readFile('Makefile');

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

ensure(readme.includes('docs/README.md'), 'README does not reference docs/README.md');
ensure(readme.includes('docs/MASTER_PLAN.md'), 'README does not reference docs/MASTER_PLAN.md');
ensure(
  readme.includes('docs/tickets/README.md'),
  'README does not reference docs/tickets/README.md',
);
ensure(
  readme.includes('docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md'),
  'README does not reference the concept spec',
);
ensure(agents.includes('docs/README.md'), 'AGENTS.md does not reference docs/README.md');
ensure(
  agents.includes('docs/MASTER_PLAN.md'),
  'AGENTS.md does not reference docs/MASTER_PLAN.md',
);
ensure(
  docsReadme.includes('[MASTER_PLAN.md](MASTER_PLAN.md)'),
  'docs/README.md does not point to docs/MASTER_PLAN.md as current truth',
);
ensure(
  docsReadme.includes('[tickets/README.md](tickets/README.md)'),
  'docs/README.md does not point to docs/tickets/README.md as queue entrypoint',
);
ensure(
  docsReadme.includes('cognitive-agent-architecture/README.md'),
  'docs/README.md does not point to the architecture pack',
);
ensure(
  docsReadme.includes('docs/tickets/phase-1') || docsReadme.includes('docs/tickets/phase-1/*.md'),
  'docs/README.md does not point to the phase ticket queues',
);
ensure(
  masterPlan.includes('Execution-Backed Verification'),
  'docs/MASTER_PLAN.md is missing execution-backed verification guidance',
);
ensure(
  masterPlan.includes('Cross-Cutting Trait Discipline'),
  'docs/MASTER_PLAN.md is missing cross-cutting trait discipline guidance',
);
ensure(
  masterPlan.includes('020-documentation-catalog-single-source.md'),
  'docs/MASTER_PLAN.md is missing the documentation catalog ticket',
);
ensure(
  ticketsReadme.includes('020-documentation-catalog-single-source.md'),
  'docs/tickets/README.md is missing the documentation catalog ticket',
);
ensure(
  conceptSpec.includes('Single Orchestrator By Default'),
  'concept spec is missing the orchestrator-first principle',
);
ensure(
  conceptSpec.includes('Capability Mediation Over Raw Access'),
  'concept spec is missing capability mediation guidance',
);
ensure(
  traitsSpec.includes('modularity')
    && traitsSpec.includes('accessibility')
    && traitsSpec.includes('configurability')
    && traitsSpec.includes('rewind/replay')
    && traitsSpec.includes('composability'),
  'cross-cutting traits spec is missing one or more required traits',
);
ensure(
  architecturePackReadme.includes('spec-draft.md'),
  'architecture sub-pack README does not point to the default spec draft file',
);
ensure(
  templatesReadme.includes('docs/cognitive-agent-architecture/')
    && templatesReadme.includes('docs/tickets/phase-*/'),
  'docs/templates/README.md does not describe current doc placement rules',
);
ensure(
  specTemplate.includes('docs/MASTER_PLAN.md')
    && specTemplate.includes('Cross-Cutting Traits'),
  'docs/templates/spec-template.md is missing current authority or trait guidance',
);
ensure(
  apiRuntime.includes('### `GET /v1/cluster/workers`')
    && apiRuntime.includes('### `POST /v1/evaluate`'),
  'docs/api/runtime.md does not document key mounted runtime routes',
);
ensure(
  apiChat.includes('### `GET /ws`') && apiChat.includes('/api/integrations'),
  'docs/api/chat.md does not document websocket or integration operator surfaces',
);

if (missing.length > 0) {
  console.error('verify-repo-truth: failed');
  for (const item of missing) {
    console.error(`- ${item}`);
  }
  process.exit(1);
}

console.log('verify-repo-truth: ok');

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

function readJson(relativePath) {
  const content = readFile(relativePath);
  if (!content) {
    return null;
  }
  try {
    return JSON.parse(content);
  } catch (error) {
    ensure(false, `Invalid JSON in ${relativePath}: ${error.message}`);
    return null;
  }
}

const requiredFiles = [
  '.github/workflows/ci.yml',
  'Makefile',
  'README.md',
  'AGENTS.md',
  'docs/documentation-catalog.json',
  'config/README.md',
  'config/contracts-manifest.json',
  'config/examples/connector-manifest.example.json',
  'config/examples/self-model-envelope.example.json',
  'config/templates/vel.toml.template',
  'config/templates/agent-specs.template.yaml',
  'config/templates/policies.template.yaml',
  'config/schemas/app-config.schema.json',
  'config/schemas/agent-specs.schema.json',
  'config/schemas/policies.schema.json',
  'config/schemas/model-profile.schema.json',
  'config/schemas/model-routing.schema.json',
  'config/schemas/connector-manifest.schema.json',
  'config/schemas/self-model-envelope.schema.json',
  'configs/models/templates/profile.template.toml',
  'configs/models/templates/routing.template.toml',
  'docs/README.md',
  'docs/MASTER_PLAN.md',
  'docs/tickets/README.md',
  'docs/tickets/architecture-first-parallel-queue.md',
  'docs/tickets/phase-1/parallel-execution-board.md',
  'docs/tickets/phase-1/021-canonical-schema-and-config-contracts.md',
  'docs/tickets/phase-1/022-data-sources-and-connector-architecture.md',
  'docs/tickets/phase-1/023-self-awareness-and-supervised-self-modification.md',
  'docs/tickets/phase-1/024-machine-readable-schema-and-manifest-publication.md',
  'docs/tickets/phase-1/025-config-and-contract-fixture-parity.md',
  'docs/api/README.md',
  'docs/api/runtime.md',
  'docs/api/chat.md',
  'docs/user/README.md',
  'docs/templates/README.md',
  'docs/templates/spec-template.md',
  'docs/templates/schema-template.md',
  'docs/templates/ticket-template.md',
  'docs/templates/agent-implementation-protocol.md',
  'docs/cognitive-agent-architecture/README.md',
  'docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md',
  'docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md',
  'docs/cognitive-agent-architecture/architecture/README.md',
  'docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md',
  'docs/cognitive-agent-architecture/architecture/cross-cutting-trait-audit.md',
  'docs/cognitive-agent-architecture/architecture/spec-draft.md',
  'docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md',
  'docs/cognitive-agent-architecture/integrations/data-source-catalog.md',
  'docs/cognitive-agent-architecture/cognition/self-awareness-and-supervised-self-modification.md',
  'docs/user/integrations/local-sources.md',
  'scripts/ci-smoke.sh',
  'scripts/bootstrap-demo-data.sh',
  'scripts/sync-documentation-catalog.mjs',
  'crates/vel-cli/src/commands/docs_catalog.generated.json',
  'clients/web/src/data/documentationCatalog.generated.ts',
  'clients/apple/VelAPI/Sources/VelAPI/VelDocumentation.swift',
];

for (const file of requiredFiles) {
  if (!fs.existsSync(path.join(repoRoot, file))) {
    ensure(false, `Missing file: ${file}`);
  }
}

const readme = readFile('README.md');
const agents = readFile('AGENTS.md');
const docsCatalog = readJson('docs/documentation-catalog.json');
const configReadme = readFile('config/README.md');
const contractManifest = readJson('config/contracts-manifest.json');
const cliDocsCatalog = readJson('crates/vel-cli/src/commands/docs_catalog.generated.json');
const webDocsCatalog = readFile('clients/web/src/data/documentationCatalog.generated.ts');
const appleDocsCatalog = readFile('clients/apple/VelAPI/Sources/VelAPI/VelDocumentation.swift');
const connectorManifestExample = readJson('config/examples/connector-manifest.example.json');
const selfModelExample = readJson('config/examples/self-model-envelope.example.json');
const docsReadme = readFile('docs/README.md');
const masterPlan = readFile('docs/MASTER_PLAN.md');
const ticketsReadme = readFile('docs/tickets/README.md');
const architectureFirstQueue = readFile('docs/tickets/architecture-first-parallel-queue.md');
const phaseOneParallelBoard = readFile('docs/tickets/phase-1/parallel-execution-board.md');
const conceptSpec = readFile(
  'docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md',
);
const traitsSpec = readFile(
  'docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md',
);
const canonicalSchemas = readFile(
  'docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md',
);
const traitAudit = readFile(
  'docs/cognitive-agent-architecture/architecture/cross-cutting-trait-audit.md',
);
const canonicalConnectors = readFile(
  'docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md',
);
const dataSourceCatalog = readFile(
  'docs/cognitive-agent-architecture/integrations/data-source-catalog.md',
);
const selfAwareness = readFile(
  'docs/cognitive-agent-architecture/cognition/self-awareness-and-supervised-self-modification.md',
);
const localSources = readFile('docs/user/integrations/local-sources.md');
const architecturePackReadme = readFile('docs/cognitive-agent-architecture/architecture/README.md');
const templatesReadme = readFile('docs/templates/README.md');
const specTemplate = readFile('docs/templates/spec-template.md');
const schemaTemplate = readFile('docs/templates/schema-template.md');
const ticketTemplate = readFile('docs/templates/ticket-template.md');
const agentProtocol = readFile('docs/templates/agent-implementation-protocol.md');
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
ensure(readme.includes('config/README.md'), 'README does not reference config/README.md');
ensure(
  readme.includes('docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md'),
  'README does not reference the canonical schemas doc',
);
ensure(agents.includes('docs/README.md'), 'AGENTS.md does not reference docs/README.md');
ensure(
  agents.includes('docs/MASTER_PLAN.md'),
  'AGENTS.md does not reference docs/MASTER_PLAN.md',
);
ensure(agents.includes('config/README.md'), 'AGENTS.md does not reference config/README.md');
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
  docsReadme.includes('[../config/README.md](../config/README.md)'),
  'docs/README.md does not point to config/README.md',
);
ensure(
  docsReadme.includes('[documentation-catalog.json](documentation-catalog.json)'),
  'docs/README.md does not point to the canonical documentation catalog manifest',
);
ensure(
  docsCatalog && Array.isArray(docsCatalog.entries),
  'docs/documentation-catalog.json is missing an entries array',
);
const expectedDocsBySurface = (surface) =>
  (docsCatalog?.entries ?? [])
    .filter((entry) => Array.isArray(entry.surfaces) && entry.surfaces.includes(surface))
    .map(({ category, title, path, description }) => ({ category, title, path, description }));
ensure(
  JSON.stringify(cliDocsCatalog) === JSON.stringify(expectedDocsBySurface('cli')),
  'CLI documentation catalog is not synchronized with docs/documentation-catalog.json',
);
ensure(
  webDocsCatalog.includes('// GENERATED FILE. DO NOT EDIT.')
    && webDocsCatalog.includes('// Source: docs/documentation-catalog.json'),
  'web documentation catalog is missing generated-file provenance markers',
);
for (const entry of expectedDocsBySurface('web')) {
  ensure(
    webDocsCatalog.includes(entry.path),
    `web documentation catalog is missing path: ${entry.path}`,
  );
}
ensure(
  appleDocsCatalog.includes('// GENERATED FILE. DO NOT EDIT.')
    && appleDocsCatalog.includes('// Source: docs/documentation-catalog.json'),
  'Apple documentation catalog is missing generated-file provenance markers',
);
for (const entry of expectedDocsBySurface('apple')) {
  ensure(
    appleDocsCatalog.includes(entry.path),
    `Apple documentation catalog is missing path: ${entry.path}`,
  );
}
ensure(
  docsReadme.includes('docs/tickets/phase-1') || docsReadme.includes('docs/tickets/phase-1/*.md'),
  'docs/README.md does not point to the phase ticket queues',
);
ensure(
  masterPlan.includes('Architecture Lock-In Queue'),
  'docs/MASTER_PLAN.md is missing the architecture lock-in queue section',
);
ensure(
  masterPlan.includes('parallel-execution-board.md'),
  'docs/MASTER_PLAN.md is missing the phase-1 parallel execution board reference',
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
for (const ticket of [
  '021-canonical-schema-and-config-contracts.md',
  '022-data-sources-and-connector-architecture.md',
  '023-self-awareness-and-supervised-self-modification.md',
  '024-machine-readable-schema-and-manifest-publication.md',
  '025-config-and-contract-fixture-parity.md',
]) {
  ensure(
    masterPlan.includes(ticket),
    `docs/MASTER_PLAN.md is missing foundational contract ticket: ${ticket}`,
  );
  ensure(
    ticketsReadme.includes(ticket),
    `docs/tickets/README.md is missing foundational contract ticket: ${ticket}`,
  );
  ensure(
    architectureFirstQueue.includes(ticket),
    `architecture-first queue is missing foundational contract ticket: ${ticket}`,
  );
}
ensure(
  ticketsReadme.includes('020-documentation-catalog-single-source.md'),
  'docs/tickets/README.md is missing the documentation catalog ticket',
);
ensure(
  ticketsReadme.includes('parallel-execution-board.md'),
  'docs/tickets/README.md is missing the phase-1 parallel execution board reference',
);
ensure(
  architectureFirstQueue.includes('parallel-execution-board.md'),
  'architecture-first queue is missing the phase-1 parallel execution board reference',
);
ensure(
  phaseOneParallelBoard.includes('Lane') && phaseOneParallelBoard.includes('Primary Write Scope'),
  'phase-1 parallel execution board is missing lane or write-scope structure',
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
  traitsSpec.includes('cross-cutting-trait-audit.md'),
  'cross-cutting traits spec does not reference the current trait audit artifact',
);
ensure(
  traitAudit.includes('Subsystem Audit Matrix') && traitAudit.includes('Gap Classification'),
  'cross-cutting trait audit doc is missing matrix or gap classification sections',
);
ensure(
  architecturePackReadme.includes('spec-draft.md'),
  'architecture sub-pack README does not point to the default spec draft file',
);
ensure(
  architecturePackReadme.includes('canonical-schemas-and-contracts.md'),
  'architecture sub-pack README does not point to canonical-schemas-and-contracts.md',
);
ensure(
  architecturePackReadme.includes('cross-cutting-trait-audit.md'),
  'architecture sub-pack README does not point to cross-cutting-trait-audit.md',
);
ensure(
  templatesReadme.includes('docs/cognitive-agent-architecture/')
    && templatesReadme.includes('docs/tickets/phase-*/'),
  'docs/templates/README.md does not describe current doc placement rules',
);
ensure(
  templatesReadme.includes('schema-template.md'),
  'docs/templates/README.md does not include schema-template.md',
);
ensure(
  specTemplate.includes('docs/MASTER_PLAN.md')
    && specTemplate.includes('Cross-Cutting Traits')
    && specTemplate.includes('Schema And Manifest Artifacts')
    && specTemplate.includes('Scientific Substrate And Symbolic Layer'),
  'docs/templates/spec-template.md is missing current authority or trait guidance',
);
ensure(
  schemaTemplate.includes('Versioning And Migration')
    && schemaTemplate.includes('Publication'),
  'docs/templates/schema-template.md is missing versioning or publication guidance',
);
ensure(
  ticketTemplate.includes('apply_patch') && ticketTemplate.includes('Repo-Aware Scope'),
  'docs/templates/ticket-template.md is missing scoped patch or repo-aware scope guidance',
);
ensure(
  agentProtocol.includes('Contract-first discipline')
    && agentProtocol.includes('Repo-aware supervision'),
  'docs/templates/agent-implementation-protocol.md is missing contract-first or repo-aware guidance',
);
ensure(
  canonicalSchemas.includes('config/contracts-manifest.json')
    && canonicalSchemas.includes('Scientific Substrate And Symbolic Layer'),
  'canonical schemas doc is missing contract manifest or scientific/symbolic guidance',
);
ensure(
  canonicalConnectors.includes('data-source-catalog.md')
    && canonicalConnectors.includes('credential_api')
    && canonicalConnectors.includes('delegated_connector'),
  'canonical connectors doc is missing synchronized source-mode vocabulary',
);
ensure(
  dataSourceCatalog.includes('credential_api')
    && dataSourceCatalog.includes('delegated_connector'),
  'data source catalog is missing synchronized source-mode vocabulary',
);
ensure(
  selfAwareness.includes('Write classes that should always require explicit operator authorization')
    && selfAwareness.includes('Scientific Substrate And Symbolic Proposals'),
  'self-awareness doc is missing approval-class or scientific/symbolic guidance',
);
ensure(
  localSources.includes('subset of the full connector model')
    && localSources.includes('Scope Clarification'),
  'local sources doc does not clearly scope itself to local-source modes',
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
ensure(
  configReadme.includes('contracts-manifest.json')
    && configReadme.includes('Scientific Substrate vs Symbolic Layer'),
  'config/README.md is missing contract manifest or scientific/symbolic guidance',
);
ensure(
  contractManifest
    && Array.isArray(contractManifest.live_configs)
    && Array.isArray(contractManifest.templates)
    && Array.isArray(contractManifest.contract_examples)
    && Array.isArray(contractManifest.authority_docs),
  'config/contracts-manifest.json is missing one or more top-level manifest arrays',
);
ensure(
  contractManifest?.authority_docs?.includes(
    'docs/tickets/phase-1/024-machine-readable-schema-and-manifest-publication.md',
  ) && contractManifest?.authority_docs?.includes(
    'docs/tickets/phase-1/025-config-and-contract-fixture-parity.md',
  ),
  'config/contracts-manifest.json is missing downstream publication/parity ticket references',
);
ensure(
  connectorManifestExample?.source_mode === 'credential_api'
    && connectorManifestExample?.integration_family === 'calendar',
  'connector manifest example is missing canonical family or source mode',
);
ensure(
  Array.isArray(selfModelExample?.write_scopes)
    && typeof selfModelExample?.review_gate === 'string',
  'self-model envelope example is missing write scopes or review gate',
);

if (missing.length > 0) {
  console.error('verify-repo-truth: failed');
  for (const item of missing) {
    console.error(`- ${item}`);
  }
  process.exit(1);
}

console.log('verify-repo-truth: ok');

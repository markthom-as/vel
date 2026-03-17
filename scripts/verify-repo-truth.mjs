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

const retiredDocumentationPaths = new Set([
  'docs/status.md',
  'docs/architecture.md',
]);

const fallbackJsonFixtureSchemaMap = new Map([
  ['config/examples/connector-manifest.example.json', 'config/schemas/connector-manifest.schema.json'],
  ['config/examples/self-model-envelope.example.json', 'config/schemas/self-model-envelope.schema.json'],
]);

function describeValueType(value) {
  if (Array.isArray(value)) {
    return 'array';
  }
  if (value === null) {
    return 'null';
  }
  return typeof value;
}

function isPlainObject(value) {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function escapeJsonPointer(segment) {
  return String(segment).replace(/~/g, '~0').replace(/\//g, '~1');
}

function joinJsonPointer(base, segment) {
  if (base === '$') {
    return `${base}/${escapeJsonPointer(segment)}`;
  }
  return `${base}/${escapeJsonPointer(segment)}`;
}

function resolveSchemaRef(rootSchema, ref) {
  if (typeof ref !== 'string' || !ref.startsWith('#/')) {
    throw new Error(`Unsupported schema reference: ${String(ref)}`);
  }
  const pathParts = ref.slice(2).split('/').map((part) => part.replace(/~1/g, '/').replace(/~0/g, '~'));
  let current = rootSchema;
  for (const part of pathParts) {
    if (!isPlainObject(current) || !(part in current)) {
      throw new Error(`Unresolvable schema reference: ${ref}`);
    }
    current = current[part];
  }
  return current;
}

function validateAgainstSchema({
  schemaNode,
  rootSchema,
  value,
  pointer,
  errors,
}) {
  if (!isPlainObject(schemaNode)) {
    return;
  }

  if (schemaNode.$ref) {
    try {
      const resolved = resolveSchemaRef(rootSchema, schemaNode.$ref);
      validateAgainstSchema({ schemaNode: resolved, rootSchema, value, pointer, errors });
    } catch (error) {
      errors.push(`${pointer}: ${error.message}`);
    }
    return;
  }

  if (Array.isArray(schemaNode.oneOf)) {
    const branchErrors = schemaNode.oneOf.map((branch) => {
      const candidateErrors = [];
      validateAgainstSchema({ schemaNode: branch, rootSchema, value, pointer, errors: candidateErrors });
      return candidateErrors;
    });
    const matchCount = branchErrors.filter((candidateErrors) => candidateErrors.length === 0).length;
    if (matchCount !== 1) {
      errors.push(`${pointer}: expected exactly one schema match, matched ${matchCount}`);
      if (matchCount === 0) {
        for (const [index, candidateErrors] of branchErrors.entries()) {
          for (const candidateError of candidateErrors) {
            errors.push(`${pointer}: oneOf[${index}] ${candidateError.replace(`${pointer}: `, '')}`);
          }
        }
      }
    }
    return;
  }

  if (schemaNode.type) {
    const allowedTypes = Array.isArray(schemaNode.type) ? schemaNode.type : [schemaNode.type];
    const valueType = describeValueType(value);
    const typeMatches = allowedTypes.some((type) => {
      if (type === 'integer') {
        return Number.isInteger(value);
      }
      if (type === 'number') {
        return typeof value === 'number' && Number.isFinite(value);
      }
      if (type === 'object') {
        return isPlainObject(value);
      }
      if (type === 'array') {
        return Array.isArray(value);
      }
      if (type === 'null') {
        return value === null;
      }
      return typeof value === type;
    });
    if (!typeMatches) {
      errors.push(`${pointer}: expected type ${allowedTypes.join('|')}, got ${valueType}`);
      return;
    }
  }

  if (Array.isArray(schemaNode.enum) && !schemaNode.enum.some((entry) => Object.is(entry, value))) {
    errors.push(`${pointer}: expected one of ${schemaNode.enum.map((entry) => JSON.stringify(entry)).join(', ')}`);
  }

  if (typeof schemaNode.pattern === 'string' && typeof value === 'string') {
    const regex = new RegExp(schemaNode.pattern);
    if (!regex.test(value)) {
      errors.push(`${pointer}: "${value}" does not match pattern ${schemaNode.pattern}`);
    }
  }

  if (typeof schemaNode.minLength === 'number' && typeof value === 'string' && value.length < schemaNode.minLength) {
    errors.push(`${pointer}: expected minLength ${schemaNode.minLength}, got ${value.length}`);
  }

  if (typeof schemaNode.minimum === 'number' && typeof value === 'number' && value < schemaNode.minimum) {
    errors.push(`${pointer}: expected >= ${schemaNode.minimum}, got ${value}`);
  }

  if (typeof schemaNode.maximum === 'number' && typeof value === 'number' && value > schemaNode.maximum) {
    errors.push(`${pointer}: expected <= ${schemaNode.maximum}, got ${value}`);
  }

  if (typeof schemaNode.minItems === 'number' && Array.isArray(value) && value.length < schemaNode.minItems) {
    errors.push(`${pointer}: expected minItems ${schemaNode.minItems}, got ${value.length}`);
  }

  if (Array.isArray(value) && schemaNode.items) {
    for (const [index, item] of value.entries()) {
      validateAgainstSchema({
        schemaNode: schemaNode.items,
        rootSchema,
        value: item,
        pointer: joinJsonPointer(pointer, index),
        errors,
      });
    }
  }

  if (isPlainObject(value)) {
    if (Array.isArray(schemaNode.required)) {
      for (const key of schemaNode.required) {
        if (!(key in value)) {
          errors.push(`${pointer}: missing required property "${key}"`);
        }
      }
    }

    if (isPlainObject(schemaNode.properties)) {
      for (const [key, propertySchema] of Object.entries(schemaNode.properties)) {
        if (key in value) {
          validateAgainstSchema({
            schemaNode: propertySchema,
            rootSchema,
            value: value[key],
            pointer: joinJsonPointer(pointer, key),
            errors,
          });
        }
      }
    }

    if (schemaNode.propertyNames) {
      for (const key of Object.keys(value)) {
        validateAgainstSchema({
          schemaNode: schemaNode.propertyNames,
          rootSchema,
          value: key,
          pointer: `${pointer} (property name "${key}")`,
          errors,
        });
      }
    }

    if (schemaNode.additionalProperties !== undefined) {
      const definedProperties = isPlainObject(schemaNode.properties)
        ? new Set(Object.keys(schemaNode.properties))
        : new Set();
      for (const [key, propertyValue] of Object.entries(value)) {
        if (definedProperties.has(key)) {
          continue;
        }
        if (schemaNode.additionalProperties === false) {
          errors.push(`${pointer}: unexpected property "${key}"`);
          continue;
        }
        if (isPlainObject(schemaNode.additionalProperties)) {
          validateAgainstSchema({
            schemaNode: schemaNode.additionalProperties,
            rootSchema,
            value: propertyValue,
            pointer: joinJsonPointer(pointer, key),
            errors,
          });
        }
      }
    }
  }
}

function collectJsonFixtureSchemaMappings(manifest) {
  const mappings = [];
  if (Array.isArray(manifest?.contract_examples)) {
    for (const entry of manifest.contract_examples) {
      if (!entry || typeof entry.path !== 'string' || typeof entry.schema !== 'string') {
        ensure(false, 'config/contracts-manifest.json has contract_examples entry missing path/schema');
        continue;
      }
      if (entry.path.endsWith('.json')) {
        mappings.push({ path: entry.path, schema: entry.schema, source: 'contract_examples' });
      }
    }
  }

  if (mappings.length > 0) {
    return mappings;
  }

  for (const [fixturePath, schemaPath] of fallbackJsonFixtureSchemaMap.entries()) {
    mappings.push({ path: fixturePath, schema: schemaPath, source: 'fallback-map' });
  }
  return mappings;
}

function validateManifestJsonFixtures(manifest) {
  const mappings = collectJsonFixtureSchemaMappings(manifest);
  ensure(mappings.length > 0, 'No JSON fixture/schema mappings found in config/contracts-manifest.json');

  for (const mapping of mappings) {
    const fixture = readJson(mapping.path);
    const schema = readJson(mapping.schema);
    if (!fixture || !schema) {
      continue;
    }

    const validationErrors = [];
    validateAgainstSchema({
      schemaNode: schema,
      rootSchema: schema,
      value: fixture,
      pointer: '$',
      errors: validationErrors,
    });

    ensure(
      validationErrors.length === 0,
      `Fixture schema validation failed (${mapping.source}): ${mapping.path} against ${mapping.schema}\n  ${validationErrors.join('\n  ')}`,
    );
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
for (const entry of docsCatalog?.entries ?? []) {
  ensure(
    !retiredDocumentationPaths.has(entry.path),
    `canonical documentation catalog contains retired path: ${entry.path}`,
  );
}
const expectedDocsBySurface = (surface) =>
  (docsCatalog?.entries ?? [])
    .filter((entry) => Array.isArray(entry.surfaces) && entry.surfaces.includes(surface))
    .map(({ category, title, path, description }) => ({ category, title, path, description }));
const expectedCatalogPaths = [
  ...expectedDocsBySurface('cli').map((entry) => entry.path),
  ...expectedDocsBySurface('web').map((entry) => entry.path),
  ...expectedDocsBySurface('apple').map((entry) => entry.path),
];
for (const docPath of expectedCatalogPaths) {
  ensure(
    !retiredDocumentationPaths.has(docPath),
    `surfaced documentation catalog contains retired path: ${docPath}`,
  );
}
ensure(
  expectedCatalogPaths.includes('docs/MASTER_PLAN.md'),
  'surfaced documentation catalogs do not include docs/MASTER_PLAN.md',
);
ensure(
  JSON.stringify(cliDocsCatalog) === JSON.stringify(expectedDocsBySurface('cli')),
  'CLI documentation catalog is not synchronized with docs/documentation-catalog.json',
);
for (const retiredPath of retiredDocumentationPaths) {
  ensure(
    !JSON.stringify(cliDocsCatalog).includes(retiredPath),
    `CLI documentation catalog contains retired path: ${retiredPath}`,
  );
}
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
for (const retiredPath of retiredDocumentationPaths) {
  ensure(
    !webDocsCatalog.includes(retiredPath),
    `web documentation catalog contains retired path: ${retiredPath}`,
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
for (const retiredPath of retiredDocumentationPaths) {
  ensure(
    !appleDocsCatalog.includes(retiredPath),
    `Apple documentation catalog contains retired path: ${retiredPath}`,
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
validateManifestJsonFixtures(contractManifest);
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

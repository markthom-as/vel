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

function normalizeRepoFeedbackStatus(statusValue) {
  const normalized = statusValue.trim().toLowerCase().replaceAll(' ', '_');
  if (normalized.includes('deferred')) return 'deferred';
  if (normalized.includes('in_progress')) return 'in_progress';
  if (normalized === 'done') return 'done';
  return normalized;
}

const requiredFiles = [
  '.github/workflows/ci.yml',
  'Makefile',
  'README.md',
  'AGENTS.md',
  'docs/README.md',
  'docs/reviews/README.md',
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
const reviewsReadme = readFile('docs/reviews/README.md');
const makefile = readFile('Makefile');
const status = readFile('docs/status.md');
const repoFeedbackReadme = readFile('docs/tickets/repo-feedback/README.md');
const ticketsReadme = readFile('docs/tickets/README.md');
const apiChat = readFile('docs/api/chat.md');
const repoFeedbackDir = path.join(repoRoot, 'docs', 'tickets', 'repo-feedback');

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
  docsReadme.includes('[tickets/README.md](tickets/README.md)'),
  'docs/README.md does not point to the ticket index as the active-plan entry point',
);
ensure(
  docsReadme.includes('[tickets/repo-feedback/README.md](tickets/repo-feedback/README.md)'),
  'docs/README.md does not mention the repo-feedback packet',
);
ensure(
  docsReadme.includes('[tickets/repo-audit-hardening/README.md](tickets/repo-audit-hardening/README.md)'),
  'docs/README.md does not mention the repo-audit-hardening packet',
);
ensure(
  ticketsReadme.includes('ticket inventory and triage index'),
  'docs/tickets/README.md does not describe itself as the ticket inventory and triage index',
);
ensure(
  ticketsReadme.includes('repo-audit-hardening/README.md'),
  'docs/tickets/README.md does not list the repo-audit-hardening packet',
);
ensure(
  ticketsReadme.includes('037-chat-remote-fallback-for-assistant-generation.md'),
  'docs/tickets/README.md does not include chat ticket 037 in the convenience list',
);
ensure(
  apiChat.includes('### `GET /ws`'),
  'docs/api/chat.md does not document the live /ws websocket entrypoint',
);
ensure(
  ticketsReadme.includes('[docs/status.md](../status.md)'),
  'docs/tickets/README.md does not defer implementation truth to docs/status.md',
);
ensure(
  /### 1\. Active convergence work|## Active convergence work/.test(ticketsReadme),
  'docs/tickets/README.md does not expose maturity classes for active work',
);
ensure(
  /### 2\. Near-term design \/ expansion|## Near-term design \/ expansion/.test(ticketsReadme),
  'docs/tickets/README.md does not expose maturity classes for near-term expansion',
);
ensure(
  /### 3\. Speculative \/ future architecture|## Speculative \/ future architecture/.test(ticketsReadme),
  'docs/tickets/README.md does not expose maturity classes for speculative architecture',
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
if (fs.existsSync(repoFeedbackDir)) {
  const repoFeedbackStatuses = new Map();
  for (const entry of fs.readdirSync(repoFeedbackDir).filter((name) => name.endsWith('.md'))) {
    if (entry === 'README.md') continue;
    const content = readFile(path.join('docs/tickets/repo-feedback', entry));
    const statusMatch = content.match(/^status:\s*([a-z_]+)$/m);
    ensure(statusMatch, `repo-feedback ticket missing status frontmatter: ${entry}`);
    if (!statusMatch) continue;
    const statusValue = statusMatch[1];
    ensure(
      ['done', 'in_progress', 'deferred'].includes(statusValue),
      `repo-feedback ticket has ambiguous status (${statusValue}): ${entry}`,
    );
    repoFeedbackStatuses.set(entry, statusValue);
  }
  const repoFeedbackSummaryStatuses = new Map();
  for (const match of repoFeedbackReadme.matchAll(/^- `(\d{3})` [^:]+: ([a-z_ ]+)/gm)) {
    repoFeedbackSummaryStatuses.set(match[1], normalizeRepoFeedbackStatus(match[2]));
  }
  for (const [entry, statusValue] of repoFeedbackStatuses.entries()) {
    const ticketNumber = entry.match(/^(\d{3})-/)?.[1];
    if (!ticketNumber) continue;
    const summaryStatus = repoFeedbackSummaryStatuses.get(ticketNumber);
    ensure(
      summaryStatus,
      `repo-feedback README missing convergence summary entry for ticket ${ticketNumber}`,
    );
    if (!summaryStatus) continue;
    ensure(
      summaryStatus === statusValue,
      `repo-feedback README summary status (${summaryStatus}) disagrees with frontmatter (${statusValue}) for ticket ${ticketNumber}`,
    );
  }
  ensure(
    [...repoFeedbackStatuses.values()].some((statusValue) => statusValue === 'done'),
    'repo-feedback packet does not record any completed tickets',
  );
  ensure(
    [...repoFeedbackStatuses.values()].some((statusValue) => statusValue === 'in_progress'),
    'repo-feedback packet does not record any in-progress tickets',
  );
}
ensure(
  status.includes('Repo-feedback follow-through'),
  'docs/status.md does not record repo-feedback follow-through',
);
ensure(
  reviewsReadme.includes('historical review'),
  'docs/reviews/README.md does not mark the reviews directory as historical review',
);
ensure(
  reviewsReadme.includes('../status.md'),
  'docs/reviews/README.md does not point back to docs/status.md',
);
ensure(
  reviewsReadme.includes('../tickets/README.md'),
  'docs/reviews/README.md does not point to the ticket index for active planning context',
);
ensure(
  reviewsReadme.includes('../README.md'),
  'docs/reviews/README.md does not point back to the docs taxonomy guide',
);

if (missing.length > 0) {
  console.error('verify-repo-truth: failed');
  for (const item of missing) {
    console.error(`- ${item}`);
  }
  process.exit(1);
}

console.log('verify-repo-truth: ok');

#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';

const repoRoot = process.cwd();
const summaryPath = path.join(repoRoot, 'target/coverage/vel-cli/llvm-cov-summary.json');
const thresholdsPath = path.join(repoRoot, 'config/coverage/vel-cli-line-thresholds.json');
const args = new Set(process.argv.slice(2));
const checkMode = args.has('--check');
const jsonMode = args.has('--json');

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function rel(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}

if (!fs.existsSync(summaryPath)) {
  throw new Error(`coverage summary not found: ${summaryPath}. Run scripts/run-cli-coverage.mjs first.`);
}

const report = readJson(summaryPath);
const thresholds = readJson(thresholdsPath);
const entry = report.data?.[0];
if (!entry?.totals || !Array.isArray(entry.files)) {
  throw new Error('unexpected llvm-cov JSON shape: missing data[0].totals/files');
}

const overallLinePercent = entry.totals.lines.percent;
const fileThresholds = Array.isArray(thresholds.file_line_percent_floors)
  ? thresholds.file_line_percent_floors
  : [];
const fileCoverage = new Map(entry.files.map((file) => [rel(file.filename), file.summary.lines.percent]));

const results = fileThresholds.map((threshold) => {
  const actual = fileCoverage.get(threshold.path);
  return {
    path: threshold.path,
    floor: threshold.percent,
    actual: actual ?? null,
    pass: typeof actual === 'number' && actual >= threshold.percent,
  };
});

const output = {
  overall: {
    floor: thresholds.overall_line_percent_floor,
    actual: overallLinePercent,
    pass: overallLinePercent >= thresholds.overall_line_percent_floor,
  },
  files: results,
  metadata: {
    threshold_stage: thresholds.threshold_stage ?? null,
    measured_at: new Date().toISOString(),
  },
};

if (jsonMode) {
  console.log(JSON.stringify(output, null, 2));
} else {
  console.log('check-cli-line-coverage: summary');
  console.log(`  overall line coverage: ${overallLinePercent.toFixed(2)}%`);
  console.log(`  overall floor: ${Number(thresholds.overall_line_percent_floor).toFixed(2)}%`);
  for (const result of results) {
    const actual = result.actual === null ? 'missing' : `${result.actual.toFixed(2)}%`;
    const verdict = result.pass ? 'pass' : 'fail';
    console.log(`  ${verdict}: ${result.path} ${actual} (floor ${result.floor.toFixed(2)}%)`);
  }
}

if (checkMode) {
  const failures = [];
  if (!output.overall.pass) {
    failures.push(
      `overall vel-cli line coverage ${overallLinePercent.toFixed(2)}% is below floor ${Number(
        thresholds.overall_line_percent_floor
      ).toFixed(2)}%`
    );
  }
  for (const result of results) {
    if (result.actual === null) {
      failures.push(`coverage summary missing file: ${result.path}`);
    } else if (!result.pass) {
      failures.push(
        `${result.path} line coverage ${result.actual.toFixed(2)}% is below floor ${result.floor.toFixed(2)}%`
      );
    }
  }
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(`check-cli-line-coverage: ${failure}`);
    }
    process.exit(1);
  }
}

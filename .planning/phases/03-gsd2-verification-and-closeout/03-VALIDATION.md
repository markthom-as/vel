# Phase 03 Validation

## Status

Complete on 2026-04-15.

## Commands Exercised

- `node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs progress bar --raw`
- `node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs state-snapshot`
- `node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs roadmap analyze`
- `node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs validate health`
- `node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs init progress`
- `node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs init new-milestone`
- `node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs init cleanup`
- `command -v gsd`
- `gsd --version`
- `gsd --help`
- `PATH="/opt/homebrew/opt/node@22/bin:$PATH" gsd headless --help`
- `PATH="/opt/homebrew/opt/node@22/bin:$PATH" gsd list`
- `PATH="/opt/homebrew/opt/node@22/bin:$PATH" gsd headless status --timeout 60000 --output-format json`
- `PATH="/opt/homebrew/opt/node@22/bin:$PATH" gsd graph status`
- `find .planning/phases -maxdepth 1 -type d | sort`
- `ls -d .planning/milestones/v*-phases`

## Results

- Progress routing correctly reports `2/3 plans (67%)` before Phase 03 summary creation.
- Phase indexes correctly report Phase 01 and Phase 02 complete, with Phase 03 as the only incomplete plan before closeout.
- Health is `healthy` with no warnings; before Phase 03 summary creation, the only info entry is the expected missing `03-01-SUMMARY.md`.
- Active phase discovery is milestone-local: `.planning/phases/` contains only `01`, `02`, and `03` for the `0.5.8` packet.
- Cleanup has no structured `init cleanup` helper in the current v1 tool, so cleanup proof used the documented workflow inputs: milestone registry, existing `v*-phases` archive dirs, and current `.planning/phases/` listing.
- New-milestone initialization is still usable enough to find the project, roadmap, and state files, but it reports `current_milestone: "v0.1"` instead of `0.5.8`.
- `gsd-pi@2.75.0` is installed and exposes `/opt/homebrew/bin/gsd` and `/opt/homebrew/bin/gsd-cli`.
- The default shell resolves Node to `v20.20.1`; `gsd-pi@2.75.0` requires Node `>=22.0.0`.
- With `/opt/homebrew/opt/node@22/bin` first in `PATH`, Node resolves to `v25.8.1`; `gsd headless --help` and `gsd list` work.
- `gsd headless status --timeout 60000 --output-format json` reported the M001 milestone complete but timed out after three 60-second restart attempts and returned `status: timeout`.
- `gsd graph status` failed because the installed package could not resolve `@gsd-build/mcp-server`.

## Residual Debt

- v1 `init progress` and `init new-milestone` still expose a stale milestone label (`v0.1` / `milestone`) even though phase routing and summaries are correct.
- There is no `init cleanup` structured helper; the cleanup workflow remains markdown-protocol driven.
- The GSD 2 command surface is installed, but it is not yet wired as the default runtime path and cannot yet replace the v1 workflow because `headless status` times out and `graph status` has a missing dependency.

These residuals are acceptable for closing `0.5.8` as a compatibility bridge, not as a full GSD 2 migration.

## Post-Closeout Runtime Launcher Follow-Up

Follow-up on 2026-04-16 added `scripts/gsd2.sh` and `npm run gsd2 -- <args>` so repo-local checks can run the installed `gsd-pi@2.75.0` surface with a selected Node `>=22` runtime instead of ad hoc shell `PATH` edits.

Additional follow-up on 2026-04-16 made the launcher repair the missing internal `@gsd-build/mcp-server` package link when the installed `gsd-pi` bundle already includes `packages/mcp-server`.

Validated:

- `bash -n scripts/gsd2.sh`
- `npm run gsd2 -- --version` -> `2.75.0`
- `npm run gsd2 -- list` -> `No packages installed.`
- `npm run gsd2 -- headless --help`
- `npm run gsd2 -- --cli --version` -> `2.75.0`
- `npm run gsd2 -- headless query --output-format json` -> reports `All milestones complete.`
- `npm run gsd2 -- graph status` -> initially reports graph not built, not a package-resolution failure
- `npm run gsd2 -- graph build` -> `Graph built: 8 nodes, 6 edges`
- `npm run gsd2 -- graph status` -> reports graph exists with 8 nodes, 6 edges, `stale: false`

Still limited:

- v1 `init progress` and `init new-milestone` still report `v0.1` because the v1 helper only uses ROADMAP heuristics and does not parse the current patch-level `0.5.8` milestone label.
- `gsd headless status --timeout 60000 --output-format json` remains less stable than `headless query`; the verified repo-local GSD 2 route for non-mutating state is `npm run gsd2 -- headless query --output-format json`.

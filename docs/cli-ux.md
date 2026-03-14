# Vel — CLI UX Specification

## Purpose

This document defines the **initial command-line experience** for Vel.

The CLI should feel like:

- a reliable operator shell
- a thin client to `veld`
- a natural daily interface
- something fast enough to use habitually

The CLI is not just a debug tool.  
It is one of the primary ways Vel will be used in v0.

---

## Design Goals

The CLI should be:

- simple
- memorable
- composable
- readable
- low-friction
- consistent with the product vision

It should support:

- quick capture
- recall
- daily orientation
- debugging / introspection
- behavior tuning

---

## CLI Philosophy

Vel's CLI should balance three stances:

### 1. Tool mode
For direct operations.

Examples:
- search
- capture
- show config
- inspect health

### 2. Assistant mode
For summaries and guidance.

Examples:
- morning
- today
- reflect

### 3. Partner mode
For context-rich collaboration.

Examples:
- resume a project
- ask what matters
- tune Vel's own behavior

The CLI should not force the user to remember a huge noun-verb labyrinth.

Prefer a **small, stable command set**.

---

## Naming Principles

Command names should be:

- short
- obvious
- hard to confuse
- easy to say out loud

Prefer:

- `vel today`
- `vel capture`
- `vel search`
- `vel morning`

Avoid:
- overly nested command trees
- jargon-heavy verbs
- redundant aliases unless they reduce real friction

---

## Top-Level Command Shape

Recommended top-level commands for early Vel:

```text
vel health
vel config
vel capture
vel search
vel today
vel morning
vel reflect
vel suggestions
vel project
vel goal
vel jobs
vel behavior
```

Not all need to exist in bootstrap, but this should be the intended surface.

---

## Command Priorities

### Bootstrap commands
These should exist first:

- `vel health`
- `vel capture`
- `vel config show`

### Early v0 commands
These should come next:

- `vel search`
- `vel today`
- `vel morning`
- `vel reflect`
- `vel suggestions`

### Later commands
After the core loop works:

- `vel project`
- `vel goal`
- `vel timeline`
- `vel jobs`
- `vel behavior`

---

## Core Commands

## 1. `vel health`

### Purpose
Quickly verify daemon and dependency health.

### Examples
```bash
vel health
vel health --json
```

### Human output
Should show:
- veld status
- DB status
- artifact store status
- degraded mode if relevant

### JSON mode
Should return machine-friendly payload.

---

## 2. `vel capture`

### Purpose
Fast intake of ideas, notes, reminders, and quick thoughts.

### Examples
```bash
vel capture "remember lidar budget estimate"
vel capture "memoir idea: chapter about utah winters"
vel capture --type quick_idea "performance prosthetic using floor pressure"
vel capture --project data-chorus "follow up with Cornelius"
```

### Behavior
- sends capture to `veld`
- prints capture ID or short confirmation
- should be fast enough to feel nearly frictionless

### Future options
- `--voice`
- `--privacy`
- `--container`
- `--goal`

---

## 3. `vel config show`

### Purpose
Display resolved configuration.

### Examples
```bash
vel config show
vel config show --json
```

### Output
Should include:
- daemon URL
- DB path
- artifact root
- log level

This is both useful and a sanity-preserving debugging tool.

---

## 4. `vel search`

### Purpose
Search captures, artifacts, and memory.

### Examples
```bash
vel search lidar
vel search "memoir chapter"
vel search --project data-chorus "budget"
vel search --type artifact "cornelius"
```

### Output
Should prefer:
- ranked concise results
- object type labels
- snippets
- IDs or references only when useful

### JSON mode
Must exist for scripting.

---

## 5. `vel today`

### Purpose
The "what should I focus on?" command.

### Examples
```bash
vel today
vel today --json
```

### Output should include
- active threads
- pending commitments
- suggested focus
- important reminders

This is one of Vel's most important commands.

---

## 6. `vel morning`

### Purpose
A more intentional daily briefing.

### Examples
```bash
vel morning
vel morning --refresh
```

### Output should include
- priorities
- open loops
- dormant-but-important threads
- suggested first moves

This should feel like a competent executive assistant, not a productivity cult leaflet.

---

## 7. `vel reflect`

### Purpose
Review recent activity.

### Examples
```bash
vel reflect
vel reflect --day
vel reflect --week
```

### Output should include
- what happened
- what was completed
- what remains open
- possibly one or two behavioral insights

---

## 8. `vel suggestions`

### Purpose
Inspect and interact with Vel's current suggestion queue.

### Examples
```bash
vel suggestions
vel suggestions --priority high
vel suggestions --project memoir
```

### Follow-up actions
Possible subcommands later:
```bash
vel suggestions dismiss <id>
vel suggestions accept <id>
vel suggestions train <id> --note "only remind me in mornings"
```

For v0 it is okay if these are implemented as flags or separate commands.

---

## 9. `vel project`

### Purpose
Project-specific context.

### Examples
```bash
vel project show data-chorus
vel project context data-chorus
vel project resume data-chorus
```

### Notes
This command can come later, but it should be part of the long-term CLI grammar.

---

## 10. `vel goal`

### Purpose
Goal-level alignment and progress.

### Examples
```bash
vel goal show memoir
vel goal alignment memoir
```

Useful later once goals are modeled.

---

## 11. `vel jobs`

### Purpose
Operational introspection.

### Examples
```bash
vel jobs
vel jobs show job_123
```

This is more for debugging and trust than everyday use.

---

## 12. `vel behavior`

### Purpose
Tune Vel itself.

### Examples
```bash
vel behavior show
vel behavior set suggestion_mode balanced
vel behavior set quiet_hours "22:00-08:00"
vel behavior set nag.health high
```

This is critical because Vel needs a visible control surface for its own behavior.

---

## Global Flags

Recommended global flags:

```text
--json
--url <base_url>
--timeout <seconds>
--verbose
--quiet
```

### `--json`
All user-facing commands that expose data should support JSON mode.

This keeps the CLI scriptable and testable.

### `--url`
Lets the CLI point to:
- localhost
- NAS
- remote node
- test environments

---

## Output Style

Human-readable output should be:

- compact
- structured
- skimmable
- not overdesigned

Prefer:

```text
Today
─────
Active threads
• Mimesis Institute launch
• memoir draft

Suggested focus
1. revise residency proposal
2. write memoir for 20 minutes

Pending commitments
• send Cornelius notes
```

Avoid:
- giant walls of text
- excessive emoji
- fake conversational fluff

Vel can be warm without sounding like a motivational app with a head injury.

---

## Exit Codes

CLI should use sane exit codes.

Suggested approach:

- `0` success
- `1` general error
- `2` daemon unavailable / connectivity issue
- `3` validation error
- `4` degraded mode warning if used in strict automation contexts

Keep this simple.

---

## Error UX

Errors should be:

- concise
- actionable
- specific

Example:
```text
Error: Could not reach veld at http://127.0.0.1:8080
Hint: Start the daemon with `cargo run -p veld`
```

Do not dump raw stack traces to normal users by default.

`--verbose` can expose more detail.

---

## Command Completion

Not required for bootstrap, but worth supporting later:

- zsh completion
- fish completion
- bash completion

This helps daily use.

---

## Aliases

Use aliases sparingly.

Possible useful aliases later:
- `vel t` → `vel today`
- `vel m` → `vel morning`

But do not add aliases until the base command set is stable.

---

## CLI UX Principles for Suggestions

When Vel surfaces suggestions in CLI output, it should clearly distinguish:

- fact
- inference
- recommendation

Example:

```text
Fact
• You have not logged memoir work in 18 days.

Recommendation
• Write for 20 minutes today?
```

This helps maintain trust.

---

## CLI UX Principles for Recall

When recalling memory, outputs should show:

- what object was matched
- why it seems relevant
- when it occurred

Example:

```text
Artifact: Meeting Summary
Date: 2026-03-12
Why it matched: contains "lidar budget"
Snippet: Need to estimate lidar cost compared to IR cameras.
```

---

## CLI UX Principles for Behavior Tuning

Vel's self-tuning commands should feel like configuration, not occult ritual.

Good:
```bash
vel behavior set nag.memoir medium
```

Bad:
```bash
vel train partner resonance memoir --mode elder-sage
```

Resist the temptation to make the terminal a grimoire.

---

## Bootstrap CLI Scope

For the initial repository bootstrap, only implement:

- `vel health`
- `vel capture <text>`
- `vel config show`

Do not build the full command surface yet.

This document defines the intended trajectory, not the day-one implementation burden.

---

## Future CLI Expansion

Later, Vel may support richer commands such as:

```bash
vel ask "what did I say about lidar sensors?"
vel resume
vel timeline --day today
vel diary add "today felt scattered but productive"
vel week
```

These should only be added once the underlying runtime is stable enough to support them honestly.

---

## Summary

Vel's CLI should feel like:

- a trustworthy operator shell
- a daily executive-function interface
- a partner that can be interrogated, tuned, and relied upon

It should be:
- boring where boring is good
- elegant where elegance reduces friction
- never more clever than it is useful

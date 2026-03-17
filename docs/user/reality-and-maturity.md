# Vel Reality And Maturity

This guide exists to prevent confusion between:

- what Vel ships now,
- what is bootstrap-level,
- what is partial,
- what is still planned.

For canonical implementation truth, always defer to [status.md](../status.md).

## What Vel is good at today

Today, Vel is strongest at:

- local daemon plus CLI operation
- capture and recall
- commitments and nudges
- current-context generation
- run/artifact inspectability
- local snapshot and file-based integration flows
- operator-facing web surfaces for current state, suggestions, settings, and explain-oriented context inspection

This is enough for real local use, especially by a technical operator.

## What is bootstrap-level

These areas exist and are useful, but should not be mistaken for product-complete surfaces:

- Apple clients
- Apple-linked local export
- some integration setup paths
- some higher-level product ergonomics

Bootstrap means:

- the path is real,
- the path is useful,
- the path is not yet fully polished, hardened, or broadly simplified.

## What is partial

Some features are implemented but still partial in breadth or maturity.

Common examples:

- integrations that depend on local exports or permissions
- user-facing docs that are still expanding
- surfaces where inspection is stronger than polished UX
- distributed or multi-client stories that are specified more deeply than they are shipped

## What is planned rather than shipped

The repo contains many detailed specs.

Those specs are valuable, but they are not automatic proof that the behavior already exists.

When in doubt:

- trust `docs/status.md`
- treat `docs/specs/` as design and planning unless status says it shipped

## What to expect as a user today

Reasonable expectations:

- you can run Vel locally
- you can capture, review, sync, evaluate, and inspect
- you can use local integrations and some client surfaces effectively
- you may still need to understand files, permissions, and settings more than a polished consumer product would require

Unreasonable expectations:

- fully invisible integration setup
- completely automatic Apple background ingestion
- product-finished multi-client behavior everywhere
- every spec in the repo being live

## The right user posture

Use Vel today as:

- a local-first cognition runtime,
- an inspectable operator system,
- a practical personal tool that is already useful,
- but not yet a fully smoothed mass-market product.

That framing matches the actual maturity of the repo much better than either “just a prototype” or “fully finished product.”

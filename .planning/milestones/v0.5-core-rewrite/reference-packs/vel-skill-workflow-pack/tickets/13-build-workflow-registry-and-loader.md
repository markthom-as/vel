# Build workflow registry and loader

## Scope
- Discover workflows from bundled, user, and workspace paths
- Resolve namespace/name/version
- Load workflow package assets relative to manifest path
- Support `vel workflow list` and `inspect`

## Acceptance criteria
- registry lists workflow packages deterministically
- inspect shows triggers, steps, permissions, and surfaces

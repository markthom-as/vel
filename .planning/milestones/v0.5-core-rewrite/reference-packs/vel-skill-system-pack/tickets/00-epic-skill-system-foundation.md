# Epic: Vel Skill System Foundation

## Objective

Establish a Vel-native, typed, permissioned skill runtime that supports internal pluggable behavior and CLI integration.

## Why

Vel needs a coherent substrate for toggleable and composable features. The skill system is that substrate.

## Scope

- manifest design and validation
- registry discovery
- CLI integration
- context mounting
- basic policy mediation
- example first-party skills

## Success criteria

- at least 3 internal skills run through one shared runtime path
- skill manifests validate against schema
- CLI can list, inspect, validate, and run skills
- read-only permissions are gated and logged
- output schemas can be validated

# Ticket: Build manifest and schema layer

## Objective

Create the manifest parser, validator, and file reference resolver for `skill.yaml` and related schemas.

## Deliverables

- Rust models for manifest
- JSON Schema validation against `skill.schema.json`
- relative file path resolution from skill root
- meaningful validation errors
- unit tests for valid and invalid manifests

## Acceptance criteria

- runtime rejects malformed manifests with actionable messages
- file references resolve correctly from package root
- schema validator works in CLI `vel skill validate`

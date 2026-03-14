# Vel — Repo Bootstrap Specification

## Purpose

This document tells Codex how to create the initial Vel repository.

The goal is to create a clean, buildable skeleton that matches the architecture and supports the first useful implementation slices.

## Bootstrap Goals

After the initial scaffold, the repository should support:

1. a Rust workspace
2. a `veld` daemon crate
3. a `vel-cli` crate
4. shared core crates for config, models, storage, and API types
5. one working health endpoint
6. one working CLI command
7. one SQLite migration system
8. room for later Python and Swift integration

## Repository Layout

```text
vel/
├── Cargo.toml
├── README.md
├── docs/
├── crates/
├── migrations/
└── var/
```

## Initial Database Schema

Bootstrap should create:

- `captures`
- `artifacts`
- `processing_jobs`

## Initial API Surface

Implement:

- `GET /v1/health`
- `POST /v1/captures`

## Initial CLI Surface

Implement:

- `vel health`
- `vel capture "some text"`
- `vel config show`


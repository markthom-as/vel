---
id: NOW-011
status: proposed
title: Add regression coverage for Now-page awareness and freshness
owner: qa+engineering
priority: P1
---

## Goal

Lock in the fixes so the page does not quietly rot again.

## Why

This feature crosses ingestion, inference, integration metadata, caching, and UI rendering. That is prime regression soil.

## Files likely touched

- `clients/web/src/components/NowView.test.tsx`
- `clients/web/src/data/query.test.tsx`
- backend tests in `crates/veld/src/app.rs`
- any new service-specific test modules

## Required coverage

### Backend

- evaluate runs after all awareness-affecting sync routes
- local timezone affects morning/today logic
- `/v1/now` returns coherent labels + freshness + prioritized tasks
- event filtering only returns upcoming/current items

### Frontend

- focus refresh works
- interval refresh works
- stale/fresh/error badges render correctly
- human labels render instead of raw keys
- debug drawer reveals raw fields when opened

## Acceptance criteria

- The ticket is not done until the previous screenshot’s class of failures is pinned by tests.

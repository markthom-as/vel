---
id: APPLE-002
title: Define shared Apple contract layer mapped from core schemas
status: proposed
owner: agent
priority: p0
area: packages/core-sync-domain
depends_on: [APPLE-001]
---

# Goal

Create a canonical Apple-side contract layer derived from existing Vel domain and sync contracts, without inventing an alternate ontology in Swift because that road ends in tears.

# Requirements

- Define shared DTOs / domain facades in `Packages/VelAppleCore`
- Source these from existing repo contracts where possible:
  - reminder / task item
  - medication event
  - meeting / context window
  - risk state / severity band
  - suggestion / nudge
  - event log item
  - sync cursor / mutation envelope
- Document mapping from backend/core model -> Apple model
- Separate:
  - wire formats
  - persisted local models
  - view models

# Implementation notes

Recommended layering:

- `Contracts/` for Codable wire objects
- `Domain/` for app-safe semantic types
- `Mappers/` for conversion
- `Formatting/` for derived display helpers

Avoid:

- direct UI dependence in contracts
- watch-only forks of core models
- duplicated enums with slightly different spellings because that is how drift breeds

# Deliverables

- Swift package with tests
- markdown doc: `clients/apple/docs/model-mapping.md`
- fixture JSON samples pulled from real or representative backend outputs

# Acceptance criteria

- mapping is explicit and test-covered
- at least 3 fixture-driven decode tests pass
- a backend field rename causes compile/test failures in the right place rather than silent weirdness

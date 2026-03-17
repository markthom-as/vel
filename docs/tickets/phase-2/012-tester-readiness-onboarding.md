---
title: Tester-Readiness Onboarding & Node Discovery
status: planned
owner: staff-eng
type: onboarding
priority: medium
created: 2026-03-17
labels:
  - onboarding
  - distributed
  - phase-2
---

Develop a frictionless onboarding flow for early testers, focusing on automated discovery and linking of multiple nodes (Phone, Laptop, Authority Node).

## Technical Details
- **Local Source Discovery**: Automate the detection of common signal sources (e.g., Apple Health, local Git repos, Obsidian vaults) during first-run.
- **Node Link CLI**: Create `vel node link` which generates a short-lived QR code or pairing code for linking devices via Tailscale or LAN.
- **Scoped Pairing**: Pairing tokens should be short-lived, purpose-scoped, and revocable.
- **Onboarding Wizard**: A simple Web/CLI wizard that guides the user through setting up their primary authority node.
- **Freshness Diagnostics**: A diagnostic surface to show the sync state and "freshness" of newly connected nodes.
- **Trust Visibility**: Onboarding should make it clear what a linked device can read, write, or execute.
- **Accessibility Baseline**: Onboarding flows should remain usable through keyboard-first web interactions, readable CLI paths, and platform accessibility affordances on Apple surfaces.
- **Configurability**: Effective node configuration and pairing state should be inspectable after setup, not hidden behind one-shot setup flows.

## Acceptance Criteria
- A non-technical tester can connect their iPhone to a local authority node in < 2 minutes.
- Common local data sources are automatically suggested for ingestion.
- The system correctly identifies and displays the newly linked node in the `vel node list` output.
- Linked-device onboarding does not hand out long-lived broad credentials by default.
- Onboarding paths remain operable through accessible web or CLI flows even when the ideal surface is unavailable.

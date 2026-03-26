---
title: Distributed Node Choices
doc_type: guide
status: complete
owner: staff-eng
created: 2026-03-25
updated: 2026-03-25
keywords:
  - distributed
  - nodes
  - sync
  - local-first
summary: Concise helper guide for how to think about linked nodes, local authority, and distributed-state choices in Vel.
---

# Purpose

Give agents a short decision guide for distributed or multi-node work.

# Current Stance

Vel is local-first and authority-first.

That means:

- start with one authoritative runtime per user environment
- support multiple surfaces and linked clients around that authority
- widen to multi-node behavior only through explicit sync and conflict contracts

Do not assume generic distributed systems are the default architecture.

# What Counts As A Node

A node is a durable runtime authority or linked runtime participant with its own identity, storage, and sync posture.

Examples:

- the primary local daemon
- a linked device runtime
- a future user-scoped hosted runtime

A browser tab or simple HTTP client is not automatically a node in the same sense.

# Preferred Order Of Complexity

1. one local authority node
2. linked clients over explicit transport
3. bounded linked-node continuity
4. later deterministic sync and conflict handling

Do not jump straight to multi-primary assumptions.

# Canonical Rules

- keep node identity explicit
- keep sync ordering explicit
- keep provenance and ownership explicit
- treat offline and replay behavior as first-class
- fail closed on ambiguous external access or write authority

# When To Add Distributed Logic

Add distributed or node-aware logic only when the change truly requires:

- state continuity across authorities
- explicit conflict handling
- offline mutation and later reconciliation
- node-scoped capability or trust decisions

If the feature only needs another shell over the same authority, do not model it as distributed state.

# What To Avoid

- hidden multi-primary writes
- provider-specific sync shortcuts that bypass canonical ownership rules
- treating every connected surface as an equal authority
- using distributed language to justify vague state semantics
- inventing node behavior without stable IDs, run evidence, and conflict posture

# Agent Questions

Before adding node or sync behavior, ask:

1. is there still one authority, or are there now multiple authorities
2. what is the stable node identity
3. what is the ordering or replay rule
4. where do conflicts surface
5. which layer owns reconciliation
6. what evidence explains the final state

# Practical Heuristic

If the answer is “the web client needs this state too,” that is usually not a distributed-systems problem.

If the answer is “two durable runtimes may diverge and later reconcile,” that is a distributed node problem.

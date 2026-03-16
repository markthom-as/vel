---
title: Implement task voice intent bridge
status: ready
owner: agent
priority: P2
area: vel-task-hud
---

# Goal
Map voice commands onto task actions cleanly.

## Scope
Support intents such as:
- mark that done
- snooze for an hour
- what should I do next
- show tasks
- hide this for now

## Requirements
- voice layer should call task actions, not reimplement them
- ambiguous commands should degrade gracefully
- do not make voice the only path for any critical action

## Tests
- intent mapping tests
- parameter extraction tests
- no-op / unknown command handling

## Done when
- voice intents can trigger task actions through shared services


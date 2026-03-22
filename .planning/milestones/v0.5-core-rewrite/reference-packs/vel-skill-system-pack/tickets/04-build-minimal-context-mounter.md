# Ticket: Build minimal context mounting layer

## Objective

Create the initial typed context-mounting system used by skill execution.

## Scope

For MVP, support:

- time
- user/workspace metadata
- thread/session metadata
- tasks summary
- calendar summary
- projects summary

## Acceptance criteria

- skills receive stable JSON context envelope
- mounted buckets can be omitted when unavailable
- runtime records which buckets were included

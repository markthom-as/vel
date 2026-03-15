
# vel_architecture_for_agents.md

Purpose: clear architecture reference for coding agents.

---
# Repository Structure

crates/
  vel-core
  vel-storage
  vel-api
  vel-cli
  vel-signals
  vel-context
  vel-risk
  vel-nudges
  vel-threads
  vel-synthesis

---
# Data Flow

sources
→ adapters
→ signals
→ context reducer
→ commitments + dependencies
→ risk engine
→ nudge policies
→ suggestions
→ artifacts

---
# Event Lifecycle

1 signal arrives
2 signal stored
3 context recomputed
4 commitment risk updated
5 policies evaluated
6 nudges created or escalated
7 artifacts optionally produced

---
# Core Entities

signals
commitments
dependencies
threads
nudges
suggestions
artifacts
context_state

---
# Module Responsibilities

vel-signals
handles ingestion and normalization of events

vel-context
maintains current_context and state transitions

vel-risk
computes risk scores for commitments

vel-nudges
evaluates policies and produces nudges

vel-threads
links commitments, captures, and conversations

vel-synthesis
produces reflective artifacts

---
# Key Invariants

signals are append-only

context is recomputed from signals

risk derives from commitments + context

nudges derive from risk + policies

LLM synthesis never determines real-time operational state

---
# Debug / Inspection Commands

vel context
vel signals
vel risk
vel nudges
vel explain
vel threads

---
# Design Philosophy

deterministic operational layer
+
reflective synthesis layer

Signals → Context → Risk → Nudges → Reflection

---
# Implementation Principle

Always build the smallest end-to-end slice first.

Example slice

calendar event
→ prep dependency
→ risk calculation
→ nudge
→ snooze
→ escalation
→ explain

Once this works reliably, expand the system.

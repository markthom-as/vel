---
id: CAL-001
title: Connect instance registry and capability manifest
status: todo
priority: P0
dependencies:
  - INTG-001
  - INTG-003
---

# Goal

Add a durable Vel-native registry for Connect-capable instances and a capability manifest contract that declares whether an instance can host external agent runtimes.

# Scope

- define `connect_instance` identity and status model
- add capability manifest types
- persist or project instance capability snapshots
- represent reachability, environment, and policy constraints
- expose read APIs for instance discovery

# Deliverables

- domain types for Connect instance and capability manifest
- storage schema/query support
- read API for listing and inspecting instances
- fixtures for at least host, laptop, and remote-executor style instances

# Acceptance criteria

- Vel can list known Connect instances independently of project/session state.
- An instance can advertise supported runtimes and launch-related constraints without UI-specific booleans.
- Capability shape is suitable for both human operator surfaces and host-agent selection logic.

# Notes

This ticket owns the instance substrate, not launch execution itself.

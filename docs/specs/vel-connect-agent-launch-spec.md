# Vel Connect Agent Launch Spec

Status: planned
Audience: runtime, integration, web, CLI, and client implementers
Purpose: define how Vel launches and supervises external coding-agent runtimes on compatible Connect instances while keeping Vel as the canonical planner and operator surface

## 1. Summary

Vel should support launching external coding-agent runtimes such as:

- Codex
- GitHub Copilot agent modes
- Cursor agent surfaces
- Claude Code
- OpenCode
- Gemini CLI
- future compatible runtimes

The requirement is not "hardcode six brands into the UI."

The requirement is:

- any Connect instance that advertises agent-launch support can be targeted,
- the available runtimes and their capabilities are discoverable,
- launched sessions are visible and interactable in Vel UI and CLI surfaces,
- Vel's main host agent can launch, inspect, steer, and integrate those sessions,
- external agents remain bounded workers rather than alternate sources of truth.

## 2. Why This Exists

Vel already has adjacent planning for:

- provider/family/connection-aware integrations,
- project-linked agent sessions,
- multi-client swarm execution,
- bounded worker orchestration.

What is still missing is an explicit contract for agent launch across reachable compute targets.

In practice, the user should be able to:

- see which Connect instances can run which agent runtimes,
- launch an agent on the right machine for the task,
- watch status and outputs from Vel,
- send follow-up steering from Vel,
- let Vel's host agent coordinate or hand off work to those sessions.

## 3. Terminology

### 3.1 Connect instance

A **Connect instance** is a Vel-known connection or node that exposes a machine-local or service-local control surface for one or more agent runtimes.

Examples:

- the main Vel host machine
- a laptop in the Vel cluster
- a workstation reachable over Tailscale
- a remote executor node

This term should map onto the existing family/provider/connection model rather than bypass it.

### 3.2 Agent runtime

An **agent runtime** is a concrete executable or service surface that can run an interactive coding or research session.

Examples:

- `codex`
- `copilot_agent`
- `cursor_agent`
- `claude_code`
- `opencode`
- `gemini_cli`

The runtime registry must be extensible. Core code should not assume the list is closed.

### 3.3 Host agent

The **host agent** is Vel's main planner/integrator agent on the current authority node.

It owns:

- target selection
- launch requests
- session supervision
- message routing
- artifact intake
- final integration into Vel state

It does not abdicate truth to the launched runtime.

## 4. Goals

- Launch supported agent runtimes on any compatible Connect instance.
- Make compatibility discoverable through capability manifests rather than UI hardcoding.
- Show launched sessions in first-class Vel operator surfaces.
- Allow the operator and Vel host agent to interact with those sessions after launch.
- Preserve project association, auditability, and replayable event history.
- Support both direct human interaction and host-agent-mediated interaction with the same session model.

## 5. Non-Goals

- Making every external runtime a canonical source of truth for tasks, risk, or context.
- Letting arbitrary runtimes mutate Vel state without approved action contracts.
- Requiring uniform deep integration for every vendor on day one.
- Forcing writeback support where a runtime is only observable or launchable.

## 6. Core Requirements

### 6.1 Capability discovery

Each Connect instance should publish a machine-readable capability manifest that includes at least:

- `instance_id`
- `display_name`
- `reachability`
- `execution_environment`
- `supported_agent_runtimes`
- `launch_modes`
- `interactive_transport`
- `workspace_access`
- `tool_capabilities`
- `artifact_capabilities`
- `policy_constraints`

Example capability questions:

- can this instance launch `claude_code`?
- can it attach to a repo/workspace path?
- can it stream stdout/events?
- can it accept follow-up messages?
- can it return structured artifacts?
- can it be driven by the host agent or only directly by a human operator?

### 6.2 Launch semantics

Vel should treat launch as an explicit operation with a durable request/response record.

Minimum launch inputs:

- target `connect_instance_id`
- requested `agent_runtime`
- project slug
- optional workspace/repo path
- task or prompt payload
- interaction mode
- autonomy/trust profile
- initiating actor (`user`, `host_agent`, `automation`)

Launch outcome must produce or update a durable `agent_session`-like record with:

- instance identity
- runtime identity
- launch state
- external session ref
- project association
- operator/host-agent control affordances

### 6.3 Session interaction

After launch, a session must be interactable through Vel even when the underlying runtime has its own native UI.

Minimum post-launch actions:

- inspect session status
- read recent output/events
- send steering or follow-up input
- pause/cancel/stop when supported
- open the native surface when a deep link or local path exists
- attach outputs/artifacts back into Vel provenance

### 6.4 Host-agent interoperability

Vel's host agent must be able to:

- discover compatible Connect instances,
- choose a runtime based on capability and policy,
- launch a remote session,
- send follow-up instructions,
- consume intermediate and terminal outputs,
- summarize or integrate returned artifacts,
- escalate to the user when the runtime or instance is blocked.

The host agent remains the supervisor. External sessions are workers.

### 6.5 Operator surfaces

This feature must be visible in both:

- web UI
- CLI/operator surfaces

Minimum UI expectations:

- list Connect instances that support agent launch
- show which runtimes each instance supports
- let the user choose target instance + runtime + project
- show active and historical launched sessions
- surface session health, recency, state, and queue/attention indicators
- provide controls for message, steer, stop, and open-native-surface actions

This should integrate with the Projects workspace rather than living as an orphan settings panel.

## 7. Data Model Direction

This spec should extend, not replace, the planned project/session model.

### 7.1 Connect instance registry

Vel should add a durable registry or projection for Connect-capable instances with fields such as:

- `connect_instance_id`
- `connection_id`
- `node_id`
- `display_name`
- `status`
- `capabilities_json`
- `last_seen_at`
- `last_error`

### 7.2 Agent runtime catalog

Vel should model runtimes as identifiers plus capability metadata, not as hardcoded UI labels alone.

Candidate fields:

- `runtime_id`
- `display_name`
- `kind`
- `vendor`
- `supports_launch`
- `supports_interactive_followup`
- `supports_native_open`
- `supports_structured_artifacts`
- `supports_host_agent_control`

### 7.3 Session extensions

The planned `agent_sessions` model should gain fields or related tables for:

- `connect_instance_id`
- `runtime_id`
- `launch_origin`
- `external_session_ref`
- `native_open_target`
- `supervisor_session_id`
- `launch_payload_json`
- `capability_snapshot_json`

This is how Vel distinguishes "a session imported from transcript evidence" from "a live launched session on a compatible instance."

## 8. API Direction

Candidate surfaces:

- `GET /v1/connect/instances`
- `GET /v1/connect/instances/:id`
- `POST /v1/connect/instances/:id/agent-launch`
- `GET /v1/agent-runtimes`
- `GET /v1/agent-sessions/:id`
- `POST /v1/agent-sessions/:id/message`
- `POST /v1/agent-sessions/:id/steer`
- `POST /v1/agent-sessions/:id/stop`
- `POST /v1/agent-sessions/:id/open`

The final path shape can move, but the contract needs these capabilities.

## 9. UI / CLI Behavior

### 9.1 Projects page

The Projects page should support:

- `Launch agent` action
- Connect instance picker
- runtime picker filtered by instance capability
- project-linked launch composer
- live session cards for launched sessions

### 9.2 Main host-agent surface

Vel's main host-agent surface should be able to:

- recommend an instance/runtime pair,
- launch an external session on behalf of the user,
- show the spawned session as linked work,
- let the user continue either through the host agent or directly against the session.

### 9.3 CLI

Representative commands:

```bash
vel connect instances
vel agent runtimes
vel project launch-agent vel --instance laptop-west --runtime codex --prompt "refactor run event mapper"
vel agent send <session-id> "stay inside existing crate boundaries"
vel agent stop <session-id>
```

## 10. Guardrails

- Capability manifests, not brand-specific conditionals, should drive availability.
- A launched runtime must not become a hidden write authority over Vel state.
- Session actions must be auditable as events and visible in operator history.
- If an instance is read-only or launch-only, the UI must say so plainly.
- If native deep-link/open support exists, Vel should expose it, but native surfaces do not replace Vel visibility.

## 11. Relationship To Existing Specs

This spec extends and should align with:

- [vel-projects-page-spec.md](vel-projects-page-spec.md)
- [vel-multi-client-swarm-spec.md](vel-multi-client-swarm-spec.md)
- [vel-multi-vendor-integration-and-person-identity-spec.md](vel-multi-vendor-integration-and-person-identity-spec.md)
- [vel-cluster-sync-spec.md](vel-cluster-sync-spec.md)
- [vel-agent-runtime-spec.md](vel-agent-runtime-spec.md)

Interpretation rule:

- multi-vendor integration explains connection/capability modeling,
- projects explains operator session surfaces,
- swarm explains supervision,
- this spec explains launch and interaction for external agent runtimes on compatible Connect instances.

## 12. Acceptance Criteria

- There is a documented concept of Connect instances that can advertise agent-launch support.
- The spec explicitly covers runtimes such as Codex, Copilot, Cursor, Claude Code, OpenCode, Gemini CLI, and future runtimes.
- The operator can discover, launch, view, and interact with launched sessions from Vel surfaces.
- Vel's main host agent can supervise and interact with those sessions.
- The design keeps Vel as canonical planner/integrator rather than forking authority to external agents.

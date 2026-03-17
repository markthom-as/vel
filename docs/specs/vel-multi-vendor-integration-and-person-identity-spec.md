# Vel Multi-Vendor Integration and Person Identity Spec

Status: planned
Audience: architecture, adapter, storage, API, and client implementers
Purpose: define how Vel should support multiple providers per integration family, make people first-class across those providers, and prepare robust ingestion for Apple, Steam, and standards-based sources

## 1. Why This Exists

Vel already has the beginning of an integration layer, but the current model is still mostly:

- one adapter per family
- one path or credential set per family
- family-specific status cards

That is not enough for real personal use.

In practice:

- messaging means iMessage, Signal, WhatsApp, and possibly email-like or Slack-like threads
- notes means Apple Notes, Obsidian, filesystem markdown, and exported docs
- transcripts means Zoom, Google Meet, Apple voice recordings, manual transcripts, and assistant logs
- tasks means Todoist, Apple Reminders, and later other task stores
- wellbeing means Apple Health, Apple Mindfulness / meditation sessions, sleep, workouts, and medication-adjacent signals
- activity means workstation activity, git, Steam activity, and later other device or platform activity

Vel also cannot treat these as disconnected feeds. A single person may appear as:

- a meeting attendee in calendar
- a message participant in Signal
- a contact in Apple Contacts
- a collaborator in Google Workspace
- a speaker in a Zoom transcript
- a note mention in Apple Notes or Obsidian

That should converge into one Vel-native person model with traceable external identities.

## 2. Goals

- Support multiple providers within a single integration family.
- Support multiple concurrent connections per provider and per family.
- Treat person identity as a first-class Vel domain, not incidental metadata.
- Preserve vendor-specific detail without leaking vendor semantics into core inference.
- Make Apple adapters implementable without forcing Apple-specific logic into core types.
- Make Steam activity implementable as a standard activity/game-session input.
- Prefer open or widely-used interchange formats where available.
- Keep route handlers thin and keep application logic in services.
- Preserve local-first operation, inspectability, and exportability.

## 3. Non-Goals

- Building every adapter now.
- Full bidirectional sync for every source family.
- Generic CRM/contact management as a product in itself.
- Perfect global identity resolution across all providers on day one.
- Treating every vendor object as a first-class Vel object before the canonical model exists.

## 4. Core Design Principles

### 4.1 Family, Provider, Connection

Vel must distinguish three layers:

- `integration_family`: the domain family, such as `messaging`, `notes`, `transcripts`, `tasks`, `calendar`, `wellbeing`, `activity`, `contacts`, `files`
- `integration_provider`: the vendor or protocol, such as `imessage`, `signal`, `whatsapp`, `apple_notes`, `obsidian`, `zoom`, `google_meet`, `apple_reminders`, `todoist`, `steam`, `google_workspace`
- `integration_connection`: one configured account or source instance, such as:
  - personal iCloud Reminders
  - work Google Calendar
  - one Obsidian vault
  - one WhatsApp export source
  - one Steam account

This is the key boundary missing from the current one-card-per-family model.

### 4.2 Canonical Core, Vendor Edge

Vel core should own:

- family enums
- provider enums
- connection identity
- canonical source object references
- person identity and external identity linkage
- canonical signal/event/capture/commitment/transcript/message payload contracts

Adapters should own:

- auth
- API calls or local file parsing
- provider-specific field normalization
- replay-safe source identity generation
- provider capability declaration

Adapters must not own:

- person merge policy
- risk
- nudge decisions
- final context interpretation

### 4.3 Person-Native, Not String-Native

Anywhere a human shows up repeatedly, Vel should have a route toward a native person record.

Examples:

- message participants
- meeting attendees
- transcript speakers
- reminder assignees or shared-list collaborators
- note mentions or note owners where source supports it
- Google Workspace collaborators

Not every adapter must create a fully-resolved person immediately, but every adapter with person-bearing data must emit enough structured identity material for resolution later.

### 4.4 Standards First Where Useful

When there is a durable, inspectable standard, prefer that over a fragile vendor-only shape.

Highest-value standards and exchange formats for Vel:

- `iCalendar / ICS` and `CalDAV` for calendar data
- `vCard / CardDAV` for contacts and people bootstrap
- `Markdown + frontmatter` and plaintext for notes/documents
- `JSON` transcript envelopes for imported transcripts
- `WebVTT`, `SRT`, and plain text transcript imports where structured JSON is absent
- `Google Workspace APIs and export formats` for calendar/docs/drive/meet-adjacent data
- `CSV` import/export where it meaningfully lowers lock-in for tasks, contacts, or activity ledgers

Standards are useful when they improve:

- portability
- local-first exports
- fixture generation
- replay-safe ingest
- inspectability

They are not useful when they erase critical provider semantics Vel still needs.

## 5. Canonical Domain Model

### 5.1 Integration Families

Initial canonical families:

- `calendar`
- `tasks`
- `messaging`
- `notes`
- `transcripts`
- `contacts`
- `wellbeing`
- `activity`
- `files`

These families define product meaning. Providers plug into them.

### 5.2 Providers

Initial provider registry should support at least:

- `google_calendar`
- `apple_calendar`
- `ics`
- `todoist`
- `apple_reminders`
- `imessage`
- `signal`
- `whatsapp`
- `apple_notes`
- `obsidian`
- `filesystem_notes`
- `zoom`
- `google_meet`
- `apple_voice_memos`
- `google_voice`
- `assistant_transcript_import`
- `apple_contacts`
- `google_contacts`
- `carddav`
- `apple_health`
- `apple_mindfulness`
- `steam`
- `google_drive`
- `google_docs`

The provider list must be extensible without schema churn.

### 5.3 Integration Connection

Each connection should have:

- `connection_id`
- `family`
- `provider`
- `display_name`
- `account_ref`
- `scope_ref`
  - vault id
  - phone number
  - email address
  - calendar id
  - drive id
  - device id
- `connection_kind`
  - local_path
  - oauth
  - api_token
  - export_import
  - os_bridge
  - protocol_sync
- `sync_mode`
  - manual
  - scheduled_pull
  - webhook_push
  - watcher
  - bridge_push
- `enabled`
- `status`
- `capabilities`
- `last_sync_at`
- `last_sync_status`
- `last_error`

This record is the operator-facing and system-facing control plane unit.

### 5.4 Canonical Source Object Reference

Every ingested item should carry a structured origin ref:

```json
{
  "family": "messaging",
  "provider": "signal",
  "connection_id": "conn_signal_personal",
  "object_type": "message_thread",
  "external_id": "signal-thread-123",
  "external_parent_id": null,
  "source_ref": "signal:thread:signal-thread-123"
}
```

This becomes the durable bridge between vendor records and Vel-native objects.

### 5.5 Canonical Person Model

Vel should add a first-class people subsystem with at least:

- `people`
- `person_aliases`
- `person_external_identities`
- `person_contact_methods`
- `person_relationship_edges`
- `person_merge_events`

Minimum `people` fields:

- `person_id`
- `display_name`
- `sort_name`
- `kind`
  - human
  - org
  - unknown_speaker
  - service_account
- `notes`
- `metadata_json`
- `created_at`
- `updated_at`

Minimum `person_external_identities` fields:

- `person_id`
- `provider`
- `connection_id`
- `identity_type`
  - phone
  - email
  - provider_user_id
  - handle
  - speaker_label
  - contact_card_id
- `identity_value`
- `confidence`
- `verification_state`

This must support:

- one person with many provider identities
- one provider identity resolving to one person
- unresolved identities that can later be merged

### 5.6 Person Resolution Contract

Adapters that expose person-like entities must emit participant identity candidates, not just display strings.

Examples:

- iMessage / Signal / WhatsApp:
  - phone numbers
  - handles
  - participant display names
- Apple Reminders:
  - shared list owners or assignees when available
- Zoom / Google Meet:
  - speaker labels
  - participant emails
  - display names
- Google Workspace:
  - document collaborators
  - meeting attendees
  - drive owners

Resolution pipeline:

1. ingest provider object with raw identity candidates
2. normalize provider identity records
3. attempt deterministic or policy-approved person match
4. store unresolved identities when confidence is insufficient
5. make explain surfaces show why a person match exists

## 6. Family-Specific Canonical Shapes

### 6.1 Messaging

Providers:

- `imessage`
- `signal`
- `whatsapp`
- later `telegram`, `slack`, `email_thread`

Canonical messaging objects:

- `message_thread`
- `message`
- `participant`

Required thread fields:

- provider
- connection_id
- external thread id
- participant refs
- latest timestamp
- waiting state
- urgency
- scheduling relevance

Vel-native person rule:

- participants should resolve to `person_id` where possible
- unresolved participants must still preserve raw handles

### 6.2 Notes

Providers:

- `apple_notes`
- `obsidian`
- `filesystem_notes`
- later `google_docs_export`

Canonical note fields:

- title
- body or content pointer
- modified_at
- created_at if available
- notebook / vault / folder
- author or owner identity when available
- linked people mentions when available
- source object ref

Vel-native person rule:

- note ownership and explicit mentions should have a route to person linkage
- notes remain captures/artifacts, but people references should not be flattened into strings only

### 6.3 Transcripts

Providers:

- `zoom`
- `google_meet`
- `apple_voice_memos`
- `google_voice`
- `assistant_transcript_import`

Canonical transcript hierarchy:

- `transcript_session`
- `transcript_segment`
- `speaker_identity`
- optional linked recording artifact

Required fields:

- session title
- occurred_at
- provider
- source object ref
- optional recording artifact
- ordered segments
- speaker labels
- participant identity candidates

Vel-native person rule:

- each speaker should resolve to `person_id` when possible
- if not, keep stable `unknown_speaker` records instead of dropping identity structure

### 6.4 Tasks

Providers:

- `todoist`
- `apple_reminders`

Canonical task payload must support:

- title
- notes
- due time
- completion state
- priority
- labels
- project/list
- recurrence
- assignee / owner identity

Vel-native person rule:

- shared reminder/task participants should be person-linkable

### 6.5 Wellbeing

Providers:

- `apple_health`
- `apple_mindfulness`

Canonical wellbeing objects:

- `health_metric`
- `meditation_session`
- `workout_session`
- `sleep_sample`
- later medication adherence or symptom entries where appropriate

Apple-specific requirement:

- mindfulness / meditation sessions must be ingestible without overloading generic `health_metric`
- the canonical contract should allow duration, modality, start/end time, source app/device, and optional tags

### 6.6 Activity

Providers:

- workstation activity
- `git`
- `steam`

Canonical activity objects:

- `computer_activity`
- `git_activity`
- `game_activity`

Steam-specific requirement:

- Steam should map to `game_activity` with fields such as:
  - game title
  - app id
  - started_at
  - ended_at or duration
  - platform/device if known
  - social or multiplayer hints if available

This must feed attention, time-allocation, and drift-aware surfaces without pretending game activity is the same thing as shell activity.

### 6.7 Files and Workspace Data

Providers:

- `google_drive`
- `google_docs`

Canonical file/document fields:

- document id
- title
- mime type
- owner identity
- collaborator identities
- modified_at
- url or export pointer
- folder / workspace context

Google Workspace requirement:

- design for shared identity linkage across Calendar, Drive, Docs, and Meet-adjacent artifacts
- do not treat each Google product as unrelated identity space

## 7. Standards and Exchange Strategy

### 7.1 Preferred Standards

Prefer these standards and formats when available:

- calendar: `ICS`, `iCalendar`, `CalDAV`
- contacts: `vCard`, `CardDAV`
- notes/documents: `Markdown`, plaintext, frontmatter, HTML export when needed
- transcripts: structured `JSON`, `WebVTT`, `SRT`, plaintext
- tasks/export: `CSV` and JSON import/export where vendor APIs are weak

### 7.2 Google Workspace Strategy

For Google-integrated data, Vel should support both:

- direct API sync for live operational use
- export/import-friendly shapes for local-first backup and rehydration

Useful Google Workspace surfaces:

- Calendar
- Contacts
- Drive
- Docs
- Meet transcript / recording metadata when available through accessible APIs or exports

### 7.3 Apple Strategy

Apple sources often require OS-native bridges rather than public cross-platform APIs.

Design requirement:

- Apple adapters must be implementable via a local bridge layer on Apple clients
- Rust core must consume normalized contracts, not EventKit- or HealthKit-specific types

This is especially important for:

- Apple Reminders
- Apple Calendar
- Apple Notes
- Apple Contacts
- Apple Health
- Apple Mindfulness / meditation
- Apple voice recordings

## 8. Settings and Operator Surface

The operator integrations surface must evolve from one card per family to:

- family overview
- provider grouping
- multiple connection rows per provider
- explicit connection health and sync history
- source-specific guidance
- identity-resolution visibility where relevant

Minimum operator actions:

- add connection
- edit connection
- disable connection
- run sync for one connection
- run sync for a provider
- inspect sync history
- inspect sample payloads
- inspect person resolution results

## 9. API and Runtime Implications

### 9.1 Settings / Control Plane

Introduce connection-oriented APIs instead of only family-oriented fields.

Needed read model concepts:

- list families
- list providers
- list connections
- get connection detail
- get connection sync history
- get provider capability manifest

### 9.2 Runtime Looping

Runtime loops should eventually schedule by connection, not only by family kind.

Examples:

- sync personal Signal export
- sync work WhatsApp export
- sync one Obsidian vault
- sync one Steam account
- sync one Apple Reminders account

### 9.3 Explainability

Explain surfaces must be able to answer:

- which provider and connection produced this object?
- which external identities mapped to this person?
- why did Vel think this Zoom speaker and this Signal contact are the same person?
- which connection last failed?

## 10. Foundational Prep Work

Before writing vendor adapters broadly, Vel should implement:

1. canonical family/provider/connection domain types
2. storage for connections and connection events
3. capability registry for providers and standards
4. first-class people schema
5. external identity + person resolution pipeline
6. connection-aware API surfaces
7. fixture packs for multi-provider, same-person scenarios
8. explain/debug UI for person linkage and provider provenance

Without this prep, each new adapter will hardcode its own worldview and create migration debt.

## 11. Rollout Order

Recommended order:

### Phase 1: Foundations

- connection model
- provider registry
- person schema
- source object refs
- connection-aware settings API

### Phase 2: Highest-value provider expansion

- Apple Reminders
- Apple Health + Mindfulness
- iMessage / Signal / WhatsApp model
- Obsidian + Apple Notes model
- Zoom + Google Meet transcript model

### Phase 3: Broader ecosystem

- Google Contacts / Workspace identity convergence
- Steam activity
- Drive / Docs linkage
- deeper shared-person explainability

## 12. Risks

- Overfitting core types to one vendor.
- Creating person merge behavior that is too eager and hard to audit.
- Treating transcript speakers as throwaway labels instead of identity-bearing evidence.
- Confusing “provider” with “connection”.
- Making Apple support impossible by leaking Apple framework types into Rust core.
- Treating Steam as a novelty source instead of a meaningful activity signal.

## 13. Hard Rules

- `vel-core` owns canonical family/provider/connection/person semantics.
- `vel-storage` must not depend on transport DTO crates.
- Adapters emit normalized data and provenance, not inference.
- Any source with stable human participants must have a path into Vel-native person identity.
- Any new provider must declare whether it supports:
  - multiple connections
  - stable external ids
  - person identities
  - export/import
  - local-first operation
  - bidirectional writeback

## 14. Deliverables for the Ticket Pack

This spec should be executed through a dedicated integration-expansion ticket pack covering:

- foundations
- people identity
- provider capability manifests
- Apple adapter prep
- messaging vendors
- note vendors
- transcript vendors
- Steam activity
- Google Workspace data convergence
- operator surfaces
- fixtures, tests, and rollout

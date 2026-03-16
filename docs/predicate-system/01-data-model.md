
# Data Model

## Entities

Vel stores entities as typed objects.

Common entity types:

- user
- agent
- task
- document
- project
- commitment
- calendar_event

Example:

entity
------
id
type
created_at

---

## Assertions

Assertions are the central cognition primitive.

Fields:

assertion
---------
id
predicate
subject_entity
object_value
confidence
source
observed_at
valid_from
valid_until
status

Example assertion:

predicate: depends_on
subject: task_42
object: migration_17

---

## Assertion Sources

Sources track where beliefs came from.

source types:

- user_message
- tool_output
- document_scan
- rule_derivation

table:

assertion_source
----------------
id
assertion_id
source_type
source_ref

---

## Conflicts

When two assertions disagree.

assertion_conflict
------------------
id
assertion_a
assertion_b
detected_at
status

---

## State Transitions

Track changes over time.

state_transition
----------------
entity_id
previous_state
new_state
timestamp
trigger


# Rule Engine

Vel derives higher‑level reasoning using rules over predicates.

Example rules:

## Deadline Risk

IF

task_due(X, soon)
AND
task_not_started(X)

THEN

risk_of_missing_deadline(X)

---

## User Overload

IF

high_commitment_density(today)
AND
user_energy(low)

THEN

user_overloaded(today)

---

## Clarification Needed

IF

confidence(assertion) < 0.6

THEN

needs_confirmation(assertion)

---

Rules should be implemented using a Datalog‑style engine or SQL rule system.

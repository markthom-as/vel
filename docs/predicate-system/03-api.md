
# Assertion API

Vel services must interact with the predicate layer through a stable API.

## Create Observation

POST /observations

payload:

{
"type": "user_message",
"content": "...",
"timestamp": "..."
}

---

## Create Assertion

POST /assertions

payload:

{
"predicate": "depends_on",
"subject": "task_42",
"object": "migration_17",
"confidence": 0.82
}

---

## Query Assertions

GET /assertions?predicate=depends_on&subject=task_42

---

## Mark Assertion Stale

PATCH /assertions/{id}

{
"status": "stale"
}

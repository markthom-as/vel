# vel_thread_graph_spec.md

Status: Canonical thread graph specification  
Audience: coding agent implementing Vel  
Purpose: define how Vel represents unresolved threads across projects, people, conversations, commitments, and artifacts so that context, reflection, and follow-through can remain connected over time

---

# 1. Purpose

The thread graph is Vel’s model of **unfinished continuity**.

Its job is to represent things that are not well captured by isolated tasks or captures alone:

- ongoing projects
- unresolved conversations
- latent follow-ups
- recurring obligations
- thematic strands of thought
- clusters of related commitments and captures

Threads help Vel answer questions like:

- what is still open here?
- what keeps recurring?
- which conversations or projects are dormant but unresolved?
- what commitments, signals, and artifacts belong to the same underlying thread?
- what am I repeatedly circling without closure?

Vel should treat threads as first-class objects.

---

# 2. Design Principles

## 2.1 Threads are not just tasks
A thread may contain many commitments, but it is not reducible to any one of them.

## 2.2 Threads are continuity objects
They preserve the link between:
- people
- projects
- discussions
- commitments
- artifacts
- captures
- agent conversations

## 2.3 Threads should be inspectable and steerable
Vel may infer threads, but users should be able to:
- inspect
- rename
- merge
- split
- close / reopen
- link or unlink entities

## 2.4 Threads should support both operational and reflective use
Operational:
- next relevant follow-up
- unresolved project obligations

Reflective:
- recurring ideation themes
- dormant but unresolved strands
- repeated return to same problems

---

# 3. Thread Types

Use a small initial set.

## 3.1 project
Examples:
- Vel
- Onassis proposal
- Mimesis budgets
- Scylla/Charybdis system build

## 3.2 person
Examples:
- Dimitri
- Cornelius
- CPA / lawyer / collaborator follow-up threads

## 3.3 conversation
Examples:
- unresolved email exchange
- ongoing decision thread
- back-and-forth planning issue

## 3.4 theme
Examples:
- medication adherence
- overcommitment
- commute logistics
- pattern of avoiding follow-up

## 3.5 logistics
Examples:
- travel to Salt Lake
- pre-meeting routines
- recurring prep mechanics

Agents should not invent a giant taxonomy.  
These five types are enough to start.

---

# 4. Core Use Cases

The thread graph must support:

## 4.1 Open threads view
Example:
- unresolved Dimitri follow-up
- active Vel architecture thread
- dormant grant budget thread

## 4.2 Thread-aware current context
Current context may eventually include:
- open thread ids
- dominant thread
- thread risk relevance

## 4.3 Thread-aware synthesis
Weekly synthesis should be able to say:
- which threads dominated attention
- which threads remained unresolved
- which dormant threads resurfaced
- which threads absorbed time without explicit commitments

## 4.4 Thread-aware planning
Vel may eventually use threads to suggest:
- follow-up actions
- project block scheduling
- closure opportunities

---

# 5. Data Model

## 5.1 threads table

```sql
CREATE TABLE threads (
  id TEXT PRIMARY KEY,
  thread_type TEXT NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL,
  metadata_json TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);
```

### status values
- `open`
- `dormant`
- `closed`

### metadata_json examples
```json
{
  "project_hint": "vel",
  "priority_hint": "high",
  "inferred": true,
  "notes": "Created from repeated Vel architecture conversations"
}
```

## 5.2 thread_links table

```sql
CREATE TABLE thread_links (
  id TEXT PRIMARY KEY,
  thread_id TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  relation_type TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (thread_id) REFERENCES threads(id)
);
```

### entity_type values
- `commitment`
- `capture`
- `signal`
- `artifact`
- `suggestion`
- `transcript_message`
- `conversation`

### relation_type values
- `concerns`
- `originated_from`
- `blocks`
- `follows_from`
- `mentions`
- `belongs_to`

### uniqueness
```sql
CREATE UNIQUE INDEX idx_thread_link_unique
ON thread_links(thread_id, entity_type, entity_id, relation_type);
```

---

# 6. Thread Lifecycle

Threads should support these lifecycle states:

## 6.1 open
Currently relevant or unresolved.

## 6.2 dormant
No recent activity, but not resolved.

## 6.3 closed
Explicitly resolved or no longer relevant.

### transitions
- open → dormant: inactivity threshold
- dormant → open: new linked signal/capture/commitment
- open/dormant → closed: explicit closure or very high confidence completion
- closed → open: reopened by new evidence or user action

The coding agent should keep lifecycle simple at first and prefer user-visible state changes over hidden auto-closing.

---

# 7. Thread Creation Rules

Threads may be created by:

## 7.1 explicit user action
Examples:
- create thread
- promote capture to thread
- tag project/person explicitly

## 7.2 adapter/import signals
Examples:
- Todoist project import
- transcript mentioning known project repeatedly
- calendar event with recurring collaborator

## 7.3 inference
Examples:
- repeated Vel-related captures + commitments + transcript messages
- repeated person-name mentions tied to unresolved commitments
- recurring logistics cluster around Salt Lake meetings

### important rule
Inferred thread creation is allowed, but it should be explainable and steerable.

---

# 8. Thread Linking Rules

Vel should link entities to threads through deterministic or explainable heuristics.

Examples:

## 8.1 project threads
Link:
- commitments tagged/project-scoped to Vel
- transcript messages mentioning Vel design
- weekly synthesis artifacts about Vel
- suggestions regarding Vel policy tuning

## 8.2 person threads
Link:
- commitments involving a person
- conversations mentioning a collaborator
- calendar meetings with that person
- follow-up tasks

## 8.3 theme threads
Link:
- repeated meds nudges
- overcommitment suggestions
- drift-related synthesis artifacts

Do not force perfect clustering initially.  
Start simple and allow manual correction.

---

# 9. Thread Graph Operations

Support these operations eventually:

- create thread
- inspect thread
- list open threads
- link entity to thread
- unlink entity from thread
- rename thread
- merge threads
- split thread (later)
- close / reopen thread

### initial CLI
```bash
vel threads
vel thread inspect <id>
vel thread close <id>
vel thread reopen <id>
```

### later CLI
```bash
vel thread merge <id1> <id2>
vel thread link <thread_id> <entity_type> <entity_id>
```

---

# 10. Derived Thread Views

Vel should be able to generate:

## 10.1 Open threads
Threads with unresolved commitments or recent relevant activity.

## 10.2 Dormant unresolved threads
Threads with no recent signals but not closed.

## 10.3 Hot threads
Threads with:
- high risk commitments
- repeated nudges
- repeated captures
- active suggestions

## 10.4 Hidden threads
Threads with repeated mentions but no explicit commitments yet.

This is especially useful for:
- ideation from chat transcripts
- projects receiving thought but not action

---

# 11. Thread Relevance Scoring

Threads should eventually have a relevance score for current context and synthesis.

Possible inputs:
- recent activity count
- linked commitment risk
- linked nudge frequency
- linked transcript/capture frequency
- explicit project priority
- current day/calendar relevance

A simple first heuristic is fine.

Example:
```text
thread_relevance =
  recent_activity
+ linked_open_commitments
+ linked_high_risk_commitments
+ current_context_overlap
```

This can later help current context expose:
- dominant thread
- open thread ids
- thread summary artifacts

---

# 12. Thread Graph and Current Context

Current context may later include:

- `open_thread_ids`
- `dominant_thread_id`
- `thread_relevance_summary`

But do not block first implementation on this.

The thread graph should first exist independently and be inspectable.

---

# 13. Thread Graph and Synthesis

The thread graph is a major input to reflective synthesis.

Weekly synthesis should eventually be able to say:
- which threads dominated
- which threads drifted
- which threads are unresolved but inactive
- which threads generate repeated commitments/nudges

The “Vel on Vel” backlog synthesis should heavily use project thread data.

---

# 14. Thread Graph and Assistant Continuity

Chat transcripts and assistant conversations should feed the thread graph.

This allows Vel to remember:
- what you have been ideating about
- which projects recur in conversation
- which unresolved questions keep surfacing
- where ideation has not yet turned into commitments

This is one of the most important uses of threads.

---

# 15. Explainability Requirements

Vel must be able to explain:
- why a thread exists
- why an entity is linked to a thread
- why a thread was marked hot/dormant/open
- why two items appear connected

Potential CLI:
```bash
vel explain thread <id>
```

Example structured output:
```json
{
  "thread_id": "thr_vel",
  "thread_type": "project",
  "title": "Vel",
  "linked_entities": [
    {"entity_type": "commitment", "entity_id": "com_1"},
    {"entity_type": "transcript_message", "entity_id": "msg_9"}
  ],
  "reasons": [
    "Repeated project references in transcripts",
    "Multiple open commitments tagged to Vel"
  ]
}
```

---

# 16. Testing Requirements

## 16.1 Unit tests
- thread creation
- thread linking uniqueness
- lifecycle transitions
- relevance scoring basics

## 16.2 Replay tests
Given a sequence of captures/transcripts/commitments:
- thread emerges
- links accumulate
- dormant → open reactivation works

## 16.3 Explainability tests
- inferred thread has reasons
- linked entities can be explained

---

# 17. Minimal First Slice

The first end-to-end slice should be:

1. create a `Vel` project thread
2. link:
   - one commitment
   - one capture
   - one transcript message
3. inspect open threads
4. inspect thread details
5. close and reopen thread

This proves:
- first-class thread object
- cross-entity linking
- assistant continuity path

After that, implement inferred thread creation from repeated project mentions.

---

# 18. Practical Engineering Rules

1. Threads are first-class objects.
2. Links are explicit and inspectable.
3. Start with a small type system.
4. Prefer explainable heuristics over clever clustering.
5. Let users steer/rename/close threads.
6. Do not collapse threads into tags.
7. Do not require perfect inference to be useful.

---

# 19. Success Criteria

The thread graph is successful when:

- Vel can list open unresolved threads
- commitments/captures/transcripts can be linked to threads
- project continuity survives across days and conversations
- dormant but unresolved threads can be surfaced
- weekly synthesis can use thread structure
- “Vel on Vel” work can be represented as a project thread

---

# 20. Final Summary

The thread graph is Vel’s model of **unfinished continuity**.

Commitments capture obligations.
Signals capture events.
Artifacts capture outputs.
Threads capture the persistent strands that tie them together.

In short:

> threads let Vel remember not just what happened, but what remains alive across time.
# vel — Runtime Design Notes

## Framing

If you want Vel to become a real runtime and not merely a sharp local utility, the trick is simple to say and annoyingly hard to execute:

> make state explicit, make effects durable, and make generated behavior inspectable

That is the whole game.

Most "agent frameworks" fail because they optimize for apparent cleverness instead of operational legibility. The result is a gooey mess of prompts, callbacks, retries, and side effects.

Vel has a chance to avoid that.

---

# The design target

Vel should evolve toward:

> a local-first execution runtime for cognition-oriented workflows

That means it can run:
- capture processing
- retrieval tasks
- context generation
- synthesis jobs
- eventually agents

But the important word is **runtime**, not "agent."

Agents are just one kind of workload.

---

# Core runtime model

A good minimal runtime model is:

```text
Input
  ↓
Run
  ↓
Steps
  ↓
Events
  ↓
Artifacts / Outputs
  ↓
Final state
```

## First-class objects

Vel should probably end up with these domain objects:

- `Run`
- `Step`
- `Event`
- `Artifact`
- `ToolInvocation`
- `Job`
- `ContextOutput`

You do not need all of them fully fleshed out on day one, but the mental model matters.

---

# State machine before agent loop

Do not start with "the agent thinks, then maybe calls a tool."

Start with a state machine.

Example run states:
- `queued`
- `running`
- `blocked`
- `succeeded`
- `failed`
- `cancelled`

Example step states:
- `pending`
- `in_progress`
- `succeeded`
- `failed`
- `retryable_failed`
- `cancelled`

### Why

A visible state machine prevents the runtime from becoming metaphysical sludge.

---

# Event-sourced-ish, not event-theater

You do not need to go full cathedral and become an event-sourcing cult.

But you **do** want append-only runtime events for important transitions.

Examples:
- `RUN_CREATED`
- `RUN_STARTED`
- `STEP_STARTED`
- `TOOL_INVOKED`
- `TOOL_SUCCEEDED`
- `TOOL_FAILED`
- `ARTIFACT_WRITTEN`
- `RUN_CANCELLED`
- `RUN_SUCCEEDED`

### Why

This gives you:
- replayability
- inspection
- better tests
- debugging
- future UI support

The runtime can still maintain current state tables for convenience. The event log just makes the system less forgetful than its operator.

---

# Tool boundary design

If you introduce tools, make the interface strict early.

## Desired types

- `ToolSpec`
- `ToolInput`
- `ToolOutput`
- `ToolError`
- `ToolTimeout`
- `RetryPolicy`

## Important properties

1. **Structured inputs**

   Avoid stringly-typed soup.

2. **Structured outputs**

   Outputs should be machine-usable and artifact-friendly.

3. **Timeouts**

   Every tool invocation should have a time budget.

4. **Cancellation hooks**

   Long-running tasks need cooperative cancellation.

5. **Idempotency support**

   Replays and retries should not duplicate side effects when avoidable.

---

# Artifact-oriented execution

This is one of the sharpest paths forward for Vel.

Instead of thinking:

> the runtime primarily produces chat turns

think:

> the runtime primarily produces artifacts

Artifacts may be:
- markdown briefings
- extracted URLs
- structured JSON
- search result bundles
- context summaries
- transcripts
- generated plans

### Why it matters

Artifacts are:
- durable
- inspectable
- replay-friendly
- composable
- easy to debug

Chat transcripts alone are not.

---

# Proposed runtime loop

A minimal run execution loop could look like this:

1. create run
2. emit `RUN_CREATED`
3. transition to `running`
4. determine next step
5. execute step
6. emit step/tool/artifact events
7. update run state
8. continue until terminal state

Pseudocode:

```rust
loop {
    if cancellation_requested(run_id) {
        emit(RUN_CANCELLED);
        mark_run_cancelled(run_id);
        break;
    }

    let step = scheduler.next_step(run_id)?;

    match execute_step(step).await {
        Ok(outcome) => {
            persist_outcome(&outcome)?;
            emit_step_success(&outcome)?;
        }
        Err(err) if err.is_retryable() => {
            schedule_retry(step, err)?;
            emit_step_retryable_failure(step, err)?;
        }
        Err(err) => {
            mark_run_failed(run_id, &err)?;
            emit_run_failed(run_id, &err)?;
            break;
        }
    }

    if run_complete(run_id)? {
        mark_run_succeeded(run_id)?;
        emit_run_succeeded(run_id)?;
        break;
    }
}
```

Not glamorous. Very healthy.

---

# Scheduling model

Do not overbuild this early.

You probably want a tiny scheduler abstraction that can answer:

- what run should execute next?
- what step is ready?
- is retry due?
- is cancellation pending?

Initially this can all live in SQLite and a single worker process.

### Good enough early model

- one daemon
- one worker loop
- SQLite-backed jobs/runs
- cooperative cancellation
- bounded concurrency

This is plenty.

---

# Job queue model

Add a minimal queue abstraction early.

Suggested job statuses:
- `queued`
- `leased`
- `running`
- `retry_scheduled`
- `succeeded`
- `failed`
- `cancelled`

Suggested job kinds:
- `artifact_extraction`
- `search_index`
- `context_generation`
- `daily_synthesis`
- `agent_run`

### Why

Once background work begins, lack of a queue model creates procedural spaghetti almost immediately.

---

# Retry semantics

This is where many runtimes become chaos goblins.

Retries should be:
- explicit
- bounded
- classified

Error types should distinguish:
- retryable transient failure
- terminal failure
- validation error
- operator cancellation
- timeout

### Important

Retries without idempotency create duplicated side effects. That is how systems become haunted.

Good mitigation:
- operation ids
- unique invocation ids
- artifact checksums
- dedupe for known side-effecting tools

---

# Cancellation semantics

Cancellation should not be performative.

A real runtime should let you:
- cancel queued work
- cancel running work cooperatively
- inspect what was partially produced
- optionally resume

### Recommended pattern

- mark run as `cancel_requested`
- worker checks between steps / during long operations
- tool invocations support timeout/cancel where possible
- emit cancellation events

CLI ideas:

```bash
vel run cancel <id>
vel run inspect <id>
vel run resume <id>
```

---

# Observability and operator ergonomics

Every runtime eventually needs an operator story, even if the operator is just Future You at 1:17 AM.

Add:

## 1. `vel doctor`
checks health/config/storage

## 2. `vel runs`
shows current/recent runs

## 3. `vel run inspect <id>`
shows:
- status
- steps
- events
- artifacts
- error details

## 4. `vel timeline`
shows recent runtime activity

## 5. structured logs
not just vibes sprayed to stdout

---

# Provenance everywhere

Every generated output should be able to answer:
- what sources were used?
- what query produced them?
- what model or generator produced this?
- when?
- can I regenerate it?

This matters especially for:
- context views
- daily briefings
- synthesis
- future agent outputs

Without provenance, the runtime becomes epistemically mushy very quickly.

---

# LLM integration: add late, add thin

When you add model-backed behavior, keep it behind a narrow interface.

Domain concepts:
- `SynthesisRequest`
- `ContextGenerationRequest`
- `SummaryOutput`

Infrastructure concepts:
- provider
- model
- token usage
- latency
- cost

Keep the second set out of core domain types as much as possible.

### Why

The runtime should survive provider churn. Those companies rotate identities faster than mythic gods on a bender.

---

# What not to build yet

## 1. Generic DAG editor
No.

## 2. Multi-host distributed orchestration
Absolutely not yet.

## 3. Fine-grained plugin marketplace abstractions
Also no.

## 4. Chat-first agent UX as the primary interface
Tempting, but dangerous.

The strongest thing in Vel is that it can become **artifact-first and orientation-first**. Protect that.

---

# A plausible evolution path

## Stage 1
- runs
- events
- artifacts
- doctor
- inspect

## Stage 2
- job queue
- context generation as jobs
- provenance
- replay

## Stage 3
- synthesis commands
- dossier/topic views
- agent runs over the same runtime substrate

## Stage 4
- richer tool system
- resumable long-running jobs
- optional remote UI / clients

---

# A runtime north star

A good test for Vel's runtime architecture is this:

> If a run fails halfway through, can I see what happened, what was produced, what is missing, and whether retrying will do the wrong thing?

If the answer is yes, the runtime is maturing.

If the answer is "well, there's a log line somewhere," it is not.

---

# Final recommendation

Build Vel's runtime around:
- explicit runs
- explicit steps
- append-only events
- durable artifacts
- structured tool boundaries
- provenance
- inspection commands

That gives you a strong substrate for everything else:
- retrieval
- context
- synthesis
- agents

Skip that discipline, and you do not get a runtime. You get a haunted bundle of side effects with a cool name.

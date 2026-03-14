# Context runs — spec (implemented)

Narrow, practical spec for making context generation (today / morning / end-of-day) run-backed.

## Implemented shape

- **Service:** `veld` `services/context_runs` — `generate_today`, `generate_morning`, `generate_end_of_day` each create a run, transition to running, load snapshot, compute via `context_generation::build_*`, write JSON to `artifact_root/context/<kind>/<run_id>.json`, create managed artifact row, create run → artifact ref, append run events, transition to succeeded (or failed on error).
- **Run detail:** `GET /v1/runs/:id` and `vel run inspect <id>` include `artifacts: Vec<ArtifactSummaryData>` (from refs from run to artifact).
- **Integration test:** `context_today_creates_run_artifact_and_ref` asserts one run, status succeeded, event sequence, one ref, and managed artifact.

---

## Original spec (current state → target)

- Context endpoints (`GET /v1/context/today`, `morning`, `end-of-day`) are implemented.
- They are computed synchronously in `veld` by the `context_generation` service from `orientation_snapshot()`.
- No run is created; no output artifact is persisted; no refs or run_events are written.
- The run/event/ref substrate exists (schema, storage, API, CLI inspection).

## Target state

- Each context request creates a **run** (kind `context_generation`).
- Run transitions: `queued` → `running` → `succeeded` (or `failed`).
- Output is persisted as an **artifact** (e.g. JSON summary); artifact is **managed** (path, checksum, size when we write it).
- **Refs** link: run → artifact; optionally artifact → source captures.
- **Run events** are appended: `run_created`, `run_started`, `context_generated`, `artifact_written`, `run_succeeded` (or `run_failed`).
- API/CLI response is unchanged from the caller’s perspective; in addition, the run is inspectable via `vel runs` / `vel run inspect <id>`.

## Event sequence (success)

1. `run_created` (seq 1)
2. `run_started` (seq 2)
3. `context_generated` (seq 3) — payload may include summary shape
4. `artifact_written` (seq 4) — payload includes artifact_id
5. `run_succeeded` (seq 5)

## Event sequence (failure)

1. `run_created` (seq 1)
2. `run_started` (seq 2)
3. `run_failed` (seq 3) — error payload in run.error_json

## Artifact behavior

- Context output (e.g. today/morning/end-of-day JSON) is written to a file under the configured artifact root.
- Artifact row: `storage_kind = managed`, canonical relative path, checksum and size populated when written.
- One artifact per successful context run.

## Provenance

- Ref: `from_type=run`, `from_id=<run_id>`, `to_type=artifact`, `to_id=<artifact_id>`, `relation_type=attached_to`.
- Optional: refs from artifact to capture IDs that were used as sources (e.g. `derived_from`).

## Acceptance tests

1. Request to `GET /v1/context/today` creates a run; run status moves queued → running → succeeded.
2. Run has input_json (e.g. `{"context_kind":"today"}`) and output_json (or artifact reference).
3. An artifact row exists; ref links run → artifact.
4. `vel run inspect <run_id>` shows the event sequence above and the artifact ref.
5. On failure (e.g. DB error during generation), run status is `failed`, run has error_json, and no artifact is created.

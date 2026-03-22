# Phase 57 Rust Backend Contract Matrix

## Purpose

Make the Rust backend contract actionable instead of philosophical.

This matrix maps backend concern areas to:

- required trait or interface families
- owning Phase 57 chunk
- first chunk that must lock the concern
- feature-gating expectations
- target assumptions
- test strategy

## Matrix

| Concern | Required Trait / Interface Family | Owned Primarily By | Must Lock By | Feature-Gated? | Target Assumptions | Test Strategy |
| --- | --- | --- | --- | --- | --- | --- |
| Type system and IDs | typed ID newtypes, envelope serialization contracts | `57-01`, `57-05` | `57-01` | no | portable Rust core | type/serde unit tests |
| Canonical object storage | `ObjectStore`, `TransactionManager` | `57-01`, `57-05` | `57-01` | backend feature only | embedded-first local persistence | in-memory + embedded storage tests |
| Relations | `RelationStore` | `57-01`, `57-05` | `57-01` | no | storage-agnostic | relation invariant tests |
| Sync links | `SyncLinkStore` | `57-01`, `57-05` | `57-01` | provider features consume it | portable linkage layer | sync-link lifecycle tests |
| Membrane actions | `PolicyEvaluator`, action/query interfaces | `57-02`, `57-05` | `57-02` | no | executor-neutral contracts | contract/golden tests |
| Ownership and policy | `PolicyEvaluator` | `57-02`, `57-05` | `57-02` | optional profiles, not optional model | portable rules | policy explain golden tests |
| Runtime records | `RuntimeStore`, `IntentProcessor` | `57-02`, `57-05` | `57-02` | no | async-safe or equivalent | WriteIntent lifecycle tests |
| Registry loading | `ManifestSource`, `RegistryLoader`, `RegistryReconciler`, `RegistryStore` | `57-03`, `57-05` | `57-03` | yes for provider packages | compiled-core first, package later | registry seeding tests |
| Workflow runtime | `RuntimeExecutor`, `RuntimeStore`, policy + registry interfaces | `57-03`, `57-05` | `57-03` | optional by workflow-related features | no raw platform handles in core | workflow run contract tests |
| Todoist adapter | adapter trait seam over core + sync + policy stores | `57-04`, `57-05` | `57-04` | `todoist` | network and secret provider optional by target | fake + real smoke tests |
| Google Calendar adapter | adapter trait seam over core + sync + policy stores | `57-04`, `57-05` | `57-04` | `google-calendar` | bounded sync defaults, network optional by target | fake + real smoke tests |
| Secrets | `CredentialProvider`, `SecretStore` | `57-05` | `57-05` | `native-secrets` or alternative provider | no raw secret storage in core | fake secret-provider tests |
| Queries | query AST / query-struct abstraction | `57-01`, `57-05` | `57-05` | no | storage-neutral semantics | in-memory query contract tests |
| Projections | `ProjectionStore` | `57-01`, `57-05` | `57-05` | optional by read-model features | rebuildable, non-authoritative | projection rebuild tests |
| Scheduler and jobs | `JobScheduler`, `RuntimeExecutor`, `IntentProcessor` | `57-05` | `57-05` | `background-sync` and related flags | target-specific executors behind traits | retry/recovery tests |
| Migrations | migration trait/interface families + artifact import API | `57-04`, `57-05` | `57-05` | backend-dependent | deterministic/idempotent | replay migration tests |
| Audit | `AuditStore` | `57-02`, `57-05` | `57-02` | `full-audit` optional expansion only | explainable and stable | audit diff/golden tests |

## Use

This matrix should be read together with:

- [57-DEPENDENCY-AND-INVARIANTS.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/57-DEPENDENCY-AND-INVARIANTS.md)
- [57-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/57-CONTEXT.md)
- [0.5-rust-backend-implementation-constraints.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/0.5-rust-backend-implementation-constraints.md)

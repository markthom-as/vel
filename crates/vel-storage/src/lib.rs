mod db;
mod infra;
mod mapping;
mod repositories;
mod storage_backend;

pub use db::{
    ArtifactInsert, ArtifactRecord, AssistantTranscriptInsert, AssistantTranscriptRecord,
    BackupRunRecord, BrokerEventRecord, CaptureInsert, ClusterWorkerRecord, ClusterWorkerUpsert,
    CommitmentInsert, ConnectRunRecord, ConversationInsert, ConversationRecord, DailySessionRecord,
    EventLogInsert, EventLogRecord, InferredStateInsert, IntegrationConnectionFilters,
    IntegrationConnectionInsert, InterventionInsert, InterventionRecord, MessageInsert,
    MessageRecord, NudgeInsert, NudgeRecord, PendingJob, RetryReadyRun, RuntimeLoopRecord,
    SearchFilters, SignalInsert, SignalRecord, Storage, StorageError, SuggestionEvidenceInsert,
    SuggestionEvidenceRecord, SuggestionFeedbackInsert, SuggestionFeedbackRecord,
    SuggestionFeedbackSummary, SuggestionInsertV2, SuggestionRecord, UncertaintyRecord,
    UncertaintyRecordInsert, UpstreamObjectRefRecord, WorkAssignmentInsert, WorkAssignmentRecord,
    WorkAssignmentUpdate,
};
pub use repositories::{
    canonical_objects_repo::{
        get_canonical_object, insert_canonical_object, update_canonical_object,
        CanonicalObjectRecord,
    },
    integration_accounts_repo::{
        get_integration_account, upsert_integration_account, IntegrationAccountRecord,
    },
    projections_repo::{get_projection, rebuild_projection, upsert_projection, ProjectionRecord},
    registry_repo::{
        get_registry_object, list_registry_objects, upsert_registry_object,
        CanonicalRegistryRecord,
    },
    relations_repo::{list_relations_from, upsert_relation, CanonicalRelationRecord},
    runtime_records_repo::{insert_runtime_record, list_runtime_records, RuntimeRecord},
    sync_links_repo::{list_sync_links_for_object, update_sync_link_state, upsert_sync_link, SyncLinkRecord},
};
pub use storage_backend::{
    AuditStore, ObjectStore, ProjectionStore, RegistryStore, RelationStore, RevisionToken,
    RuntimeStore, StorageContractError, StorageTransaction, StoreQuery, SyncLinkStore,
    TransactionManager,
};
pub use vel_core::{
    ConflictCaseRecord, ContextCapture, DurableRoutineBlock, LinkScope, LinkStatus,
    LinkedNodeRecord, ModuleId, OrderingStamp, OrientationSnapshot, PairingTokenRecord,
    PersonAlias, PersonId, PersonLinkRef, PersonRecord, PlanningConstraint,
    PlanningConstraintKind, ProjectFamily, ProjectId, ProjectProvisionRequest, ProjectRecord,
    ProjectRootRef, ProjectStatus, RoutinePlanningProfile, SearchResult, SemanticHit,
    SemanticMemoryRecord, SemanticQuery, SyncLinkId, TaskId, WorkAssignmentStatus, WorkflowId,
    WritebackOperationRecord, WritebackStatus, WriteIntentId,
};

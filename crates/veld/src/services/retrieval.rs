use vel_core::{SemanticHit, SemanticQuery, SemanticSourceKind};

use crate::{errors::AppError, state::AppState};

pub fn phase6_source_kinds() -> Vec<SemanticSourceKind> {
    vec![
        SemanticSourceKind::Project,
        SemanticSourceKind::Note,
        SemanticSourceKind::TranscriptNote,
        SemanticSourceKind::Thread,
        SemanticSourceKind::Person,
    ]
}

pub fn context_source_kinds() -> Vec<SemanticSourceKind> {
    let mut kinds = vec![SemanticSourceKind::Capture];
    kinds.extend(phase6_source_kinds());
    kinds
}

pub async fn semantic_query(
    state: &AppState,
    query: &SemanticQuery,
) -> Result<Vec<SemanticHit>, AppError> {
    Ok(state.storage.semantic_query(query).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use time::OffsetDateTime;
    use tokio::sync::broadcast;
    use vel_config::AppConfig;
    use vel_core::{
        HybridRetrievalPolicy, IntegrationConnectionId, IntegrationFamily, IntegrationSourceRef,
        PersonAlias, PersonId, PersonRecord, ProjectFamily, ProjectId, ProjectProvisionRequest,
        ProjectRecord, ProjectRootRef, ProjectStatus, RetrievalStrategy, SemanticQueryFilters,
    };
    use vel_storage::AssistantTranscriptInsert;

    fn test_state(storage: vel_storage::Storage) -> AppState {
        let (broadcast_tx, _) = broadcast::channel(8);
        AppState::new(
            storage,
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        )
    }

    #[tokio::test]
    async fn retrieval_preserves_project_person_and_thread_hits() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = OffsetDateTime::now_utc();

        storage
            .create_project(ProjectRecord {
                id: ProjectId::from("proj_retrieval".to_string()),
                slug: "accountant-ops".to_string(),
                name: "Accountant Ops".to_string(),
                family: ProjectFamily::Work,
                status: ProjectStatus::Active,
                primary_repo: ProjectRootRef {
                    path: "/tmp/tax-ops".to_string(),
                    label: "tax-ops".to_string(),
                    kind: "repo".to_string(),
                },
                primary_notes_root: ProjectRootRef {
                    path: "/tmp/notes/tax-ops".to_string(),
                    label: "tax-ops".to_string(),
                    kind: "notes_root".to_string(),
                },
                secondary_repos: vec![],
                secondary_notes_roots: vec![],
                upstream_ids: BTreeMap::new(),
                pending_provision: ProjectProvisionRequest {
                    create_repo: false,
                    create_notes_root: false,
                },
                created_at: now,
                updated_at: now,
                archived_at: None,
            })
            .await
            .unwrap();
        storage
            .create_person(PersonRecord {
                id: PersonId::from("per_retrieval".to_string()),
                display_name: "Annie Accountant".to_string(),
                given_name: Some("Annie".to_string()),
                family_name: None,
                relationship_context: Some("tax accountant".to_string()),
                birthday: None,
                last_contacted_at: Some(now),
                aliases: vec![PersonAlias {
                    platform: "email".to_string(),
                    handle: "annie@example.com".to_string(),
                    display: "Annie".to_string(),
                    source_ref: Some(IntegrationSourceRef {
                        family: IntegrationFamily::Messaging,
                        provider_key: "gmail".to_string(),
                        connection_id: IntegrationConnectionId::from("icn_people".to_string()),
                        external_id: "msg_annie".to_string(),
                    }),
                }],
                links: vec![],
            })
            .await
            .unwrap();
        storage
            .insert_thread(
                "thr_retrieval",
                "planning_execution",
                "Accountant follow-up thread",
                "open",
                "{}",
            )
            .await
            .unwrap();
        storage
            .upsert_note_semantic_record(
                "projects/tax-ops/accountant.md",
                "Accountant follow-up",
                "Need accountant follow up on quarterly estimate.",
                "cap_note_retrieval",
                now.unix_timestamp(),
            )
            .await
            .unwrap();
        storage
            .insert_assistant_transcript(AssistantTranscriptInsert {
                id: "tr_retrieval".to_string(),
                source: "chat".to_string(),
                conversation_id: "conv_retrieval".to_string(),
                message_id: Some("msg_retrieval".to_string()),
                timestamp: now.unix_timestamp(),
                role: "assistant".to_string(),
                content: "Draft the accountant follow up note.".to_string(),
                metadata_json: serde_json::json!({
                    "notes": {
                        "source_subtype": "transcript",
                    }
                }),
            })
            .await
            .unwrap();

        let state = test_state(storage);
        let hits = semantic_query(
            &state,
            &SemanticQuery {
                query_text: "accountant follow up ops".to_string(),
                top_k: 8,
                strategy: RetrievalStrategy::Hybrid,
                include_provenance: true,
                filters: SemanticQueryFilters {
                    source_kinds: context_source_kinds(),
                    ..Default::default()
                },
                policy: Some(HybridRetrievalPolicy {
                    lexical_weight: 0.25,
                    semantic_weight: 0.75,
                    rerank_window: 16,
                    min_combined_score: 0.01,
                }),
            },
        )
        .await
        .unwrap();

        assert!(hits
            .iter()
            .any(|hit| hit.source_kind == SemanticSourceKind::Project));
        assert!(hits
            .iter()
            .any(|hit| hit.source_kind == SemanticSourceKind::Note));
        assert!(hits
            .iter()
            .any(|hit| hit.source_kind == SemanticSourceKind::TranscriptNote));
        assert!(hits
            .iter()
            .any(|hit| hit.source_kind == SemanticSourceKind::Thread));
        assert!(hits
            .iter()
            .any(|hit| hit.source_kind == SemanticSourceKind::Person));
        assert!(hits.iter().all(|hit| {
            matches!(
                hit.source_kind,
                SemanticSourceKind::Capture
                    | SemanticSourceKind::Project
                    | SemanticSourceKind::Note
                    | SemanticSourceKind::TranscriptNote
                    | SemanticSourceKind::Thread
                    | SemanticSourceKind::Person
            )
        }));
    }
}

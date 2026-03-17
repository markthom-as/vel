use vel_storage::{NudgeInsert, Storage, SuggestionInsertV2};

fn test_policy_config() -> veld::policy_config::PolicyConfig {
    veld::policy_config::PolicyConfig::default()
}

async fn test_storage() -> Storage {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
}

async fn insert_nudges(storage: &Storage, nudges: &[(&str, &str)]) {
    let recent = time::OffsetDateTime::now_utc().unix_timestamp() - 3600;
    for (nudge_type, level) in nudges {
        storage
            .insert_nudge(NudgeInsert {
                nudge_type: (*nudge_type).to_string(),
                level: (*level).to_string(),
                state: "resolved".to_string(),
                related_commitment_id: None,
                message: format!("{} happened", nudge_type),
                snoozed_until: None,
                resolved_at: Some(recent),
                signals_snapshot_json: None,
                inference_snapshot_json: None,
                metadata_json: None,
            })
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn repeated_commute_danger_creates_one_suggestion() {
    let storage = test_storage().await;
    insert_nudges(
        &storage,
        &[
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
        ],
    )
    .await;

    let created =
        veld::services::suggestions::evaluate_after_nudges(&storage, &test_policy_config())
            .await
            .unwrap();

    assert_eq!(created, 1);
    let suggestions = storage.list_suggestions(Some("pending"), 10).await.unwrap();
    assert_eq!(suggestions.len(), 1);
    assert_eq!(suggestions[0].suggestion_type, "increase_commute_buffer");
}

#[tokio::test]
async fn same_evidence_does_not_duplicate_pending_suggestions() {
    let storage = test_storage().await;
    insert_nudges(
        &storage,
        &[
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
        ],
    )
    .await;

    let policy = test_policy_config();
    veld::services::suggestions::evaluate_after_nudges(&storage, &policy)
        .await
        .unwrap();
    let created = veld::services::suggestions::evaluate_after_nudges(&storage, &policy)
        .await
        .unwrap();

    assert_eq!(created, 0);
    let suggestions = storage.list_suggestions(Some("pending"), 10).await.unwrap();
    assert_eq!(suggestions.len(), 1);
}

#[tokio::test]
async fn rejected_recent_suggestion_suppresses_recreation() {
    let storage = test_storage().await;
    insert_nudges(
        &storage,
        &[
            ("meeting_prep_window", "warning"),
            ("meeting_prep_window", "warning"),
            ("meeting_prep_window", "warning"),
        ],
    )
    .await;
    let recent = time::OffsetDateTime::now_utc().unix_timestamp() - 3600;

    let suggestion_id = storage
        .insert_suggestion_v2(SuggestionInsertV2 {
            suggestion_type: "increase_prep_window".to_string(),
            state: "rejected".to_string(),
            title: Some("Increase prep window".to_string()),
            summary: None,
            priority: 60,
            confidence: Some("medium".to_string()),
            dedupe_key: Some("increase_prep_window".to_string()),
            payload_json: serde_json::json!({
                "type": "increase_prep_window",
                "suggested_minutes": 45
            }),
            decision_context_json: None,
        })
        .await
        .unwrap();
    storage
        .update_suggestion_state(&suggestion_id, "rejected", Some(recent), None)
        .await
        .unwrap();

    let created =
        veld::services::suggestions::evaluate_after_nudges(&storage, &test_policy_config())
            .await
            .unwrap();

    assert_eq!(created, 0);
}

#[tokio::test]
async fn repeated_not_useful_feedback_suppresses_family_without_recent_rejection() {
    let storage = test_storage().await;
    let stale = time::OffsetDateTime::now_utc().unix_timestamp() - (30 * 86_400);

    for _ in 0..2 {
        let suggestion_id = storage
            .insert_suggestion_v2(SuggestionInsertV2 {
                suggestion_type: "increase_prep_window".to_string(),
                state: "rejected".to_string(),
                title: Some("Increase prep window".to_string()),
                summary: None,
                priority: 60,
                confidence: Some("medium".to_string()),
                dedupe_key: Some("increase_prep_window".to_string()),
                payload_json: serde_json::json!({
                    "type": "increase_prep_window",
                    "suggested_minutes": 45
                }),
                decision_context_json: None,
            })
            .await
            .unwrap();
        storage
            .update_suggestion_state(&suggestion_id, "rejected", Some(stale), None)
            .await
            .unwrap();
        storage
            .insert_suggestion_feedback(vel_storage::SuggestionFeedbackInsert {
                suggestion_id,
                outcome_type: "rejected_not_useful".to_string(),
                notes: Some("not useful right now".to_string()),
                observed_at: stale,
                payload_json: None,
            })
            .await
            .unwrap();
    }

    insert_nudges(
        &storage,
        &[
            ("meeting_prep_window", "warning"),
            ("meeting_prep_window", "warning"),
            ("meeting_prep_window", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
        ],
    )
    .await;

    let mut policy = test_policy_config();
    policy.suggestions.response_debt.threshold = 2;
    policy.suggestions.max_new_per_evaluate = 4;

    let created = veld::services::suggestions::evaluate_after_nudges(&storage, &policy)
        .await
        .unwrap();

    assert_eq!(created, 1);
    let suggestions = storage.list_suggestions(Some("pending"), 10).await.unwrap();
    assert!(suggestions
        .iter()
        .all(|suggestion| suggestion.suggestion_type != "increase_prep_window"));
    assert!(suggestions
        .iter()
        .any(|suggestion| suggestion.suggestion_type == "add_followup_block"));
}

#[tokio::test]
async fn config_thresholds_change_creation_behavior() {
    let storage = test_storage().await;
    insert_nudges(
        &storage,
        &[
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
        ],
    )
    .await;

    let mut strict_policy = test_policy_config();
    strict_policy.suggestions.commute.threshold = 4;
    let strict_created =
        veld::services::suggestions::evaluate_after_nudges(&storage, &strict_policy)
            .await
            .unwrap();
    assert_eq!(strict_created, 0);

    let mut relaxed_policy = test_policy_config();
    relaxed_policy.suggestions.commute.threshold = 2;
    let relaxed_created =
        veld::services::suggestions::evaluate_after_nudges(&storage, &relaxed_policy)
            .await
            .unwrap();
    assert_eq!(relaxed_created, 1);
}

#[tokio::test]
async fn evidence_rows_are_written_and_inspectable() {
    let storage = test_storage().await;
    insert_nudges(
        &storage,
        &[
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
        ],
    )
    .await;

    veld::services::suggestions::evaluate_after_nudges(&storage, &test_policy_config())
        .await
        .unwrap();

    let suggestion = storage
        .list_suggestions(Some("pending"), 10)
        .await
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let evidence = storage
        .list_suggestion_evidence(&suggestion.id)
        .await
        .unwrap();
    assert_eq!(evidence.len(), 3);
    assert!(evidence.iter().all(|item| item.evidence_type == "nudge"));
}

#[tokio::test]
async fn suggestions_are_ranked_deterministically() {
    let storage = test_storage().await;
    storage
        .set_current_context(
            time::OffsetDateTime::now_utc().unix_timestamp(),
            &serde_json::json!({
                "global_risk_score": 0.8,
            })
            .to_string(),
        )
        .await
        .unwrap();
    insert_nudges(
        &storage,
        &[
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("meeting_prep_window", "warning"),
            ("meeting_prep_window", "warning"),
            ("meeting_prep_window", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("morning_drift", "warning"),
            ("morning_drift", "warning"),
            ("morning_drift", "warning"),
        ],
    )
    .await;

    let mut policy = test_policy_config();
    policy.suggestions.response_debt.threshold = 2;
    policy.suggestions.morning_drift.threshold = 2;
    policy.suggestions.max_new_per_evaluate = 4;

    let created = veld::services::suggestions::evaluate_after_nudges(&storage, &policy)
        .await
        .unwrap();
    assert_eq!(created, 4);

    let suggestions = storage.list_suggestions(Some("pending"), 10).await.unwrap();
    let ordered_types: Vec<_> = suggestions
        .iter()
        .map(|suggestion| suggestion.suggestion_type.as_str())
        .collect();
    assert_eq!(
        ordered_types,
        vec![
            "increase_commute_buffer",
            "increase_prep_window",
            "add_followup_block",
            "add_start_routine",
        ]
    );
}

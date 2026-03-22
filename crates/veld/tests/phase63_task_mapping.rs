use time::OffsetDateTime;
use vel_adapters_todoist::{
    map_todoist_comment, map_todoist_project, map_todoist_task, task_facets, AttachedCommentRecord,
    TodoistCommentAuthorStub, TodoistCommentPayload, TodoistProjectPayload, TodoistSectionFacet,
    TodoistTaskPayload,
};

#[test]
fn todoist_task_project_and_comment_mapping_stays_canonical_first() {
    let imported_at = OffsetDateTime::UNIX_EPOCH;
    let project = map_todoist_project(
        "integration_account_primary",
        &TodoistProjectPayload {
            remote_id: "proj_personal".to_string(),
            name: "Personal".to_string(),
            color: Some("berry_red".to_string()),
            is_inbox_project: false,
            sections: vec![TodoistSectionFacet {
                remote_id: "sec_morning".to_string(),
                name: "Morning".to_string(),
            }],
        },
        imported_at,
    );
    let task_payload = TodoistTaskPayload {
        remote_id: "todo_123".to_string(),
        title: "Morning review".to_string(),
        description: Some("Review inbox and routines".to_string()),
        completed: false,
        priority: Some("p1".to_string()),
        due: Some(serde_json::json!({"kind":"date","value":"2026-03-23"})),
        labels: vec![
            "maintain".to_string(),
            "time:morning".to_string(),
            "duration:15m".to_string(),
            "context:desk".to_string(),
        ],
        project_remote_id: Some("proj_personal".to_string()),
        parent_remote_id: None,
        section_remote_id: Some("sec_morning".to_string()),
    };
    let task = map_todoist_task(
        "task_01canonical",
        "integration_account_primary",
        &task_payload,
        imported_at,
    );
    let comment = map_todoist_comment(
        "integration_account_primary",
        "task_01canonical",
        &TodoistCommentPayload {
            remote_id: "comment_123".to_string(),
            parent_remote_task_id: "todo_123".to_string(),
            body: "Need to reschedule this if morning slips".to_string(),
            author: TodoistCommentAuthorStub {
                remote_id: Some("user_123".to_string()),
                display_name: Some("Jove".to_string()),
            },
            created_at: imported_at,
            updated_at: imported_at,
        },
    );

    assert_eq!(project.object_type, "project");
    assert_eq!(
        project.facets_json["provider_facets"]["todoist"]["section_posture"],
        "non-first-class"
    );
    assert_eq!(task.object_type, "task");
    assert_eq!(task.facets_json["status"], "ready");
    assert_eq!(task.facets_json["priority"], "high");
    assert_eq!(task.facets_json["task_type"], "maintain");
    assert_eq!(task.facets_json["tags"][0], "maintain");
    assert_eq!(
        task.facets_json["task_semantics"]["time_of_day_hint"],
        "morning"
    );
    assert_eq!(
        task.facets_json["task_semantics"]["estimated_duration_minutes"],
        15
    );
    assert_eq!(
        task.facets_json["task_semantics"]["interpretation_provenance"],
        "inferred_from_tag"
    );
    assert_eq!(
        task.facets_json["provider_facets"]["todoist"]["section_id"],
        "sec_morning"
    );
    assert!(
        task.facets_json["project_ref"]
            .as_str()
            .unwrap()
            .starts_with("proj_"),
        "one primary project/container relation should be preserved canonically"
    );

    let facets = task_facets("integration_account_primary", &task_payload);
    assert_eq!(facets["tags"][1], "time:morning");
    assert_eq!(
        facets["provider_facets"]["todoist"]["project_id"],
        "proj_personal"
    );

    assert_eq!(comment.parent_object_ref, "task_01canonical");
    assert_eq!(
        comment.provider_facets["todoist"]["comment_id"],
        "comment_123"
    );
    let _: AttachedCommentRecord = comment;
}

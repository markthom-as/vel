use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;
use vel_storage::CanonicalObjectRecord;

use crate::{
    project_mapping::todoist_project_id,
    todoist_ids::{todoist_provider_object_ref, TODOIST_MODULE_ID},
};

#[derive(Debug, Clone, PartialEq)]
pub struct TodoistTaskPayload {
    pub remote_id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub priority: Option<String>,
    pub due: Option<JsonValue>,
    pub labels: Vec<String>,
    pub project_remote_id: Option<String>,
    pub parent_remote_id: Option<String>,
    pub section_remote_id: Option<String>,
}

pub fn map_todoist_task(
    task_id: &str,
    integration_account_id: &str,
    payload: &TodoistTaskPayload,
    imported_at: OffsetDateTime,
) -> CanonicalObjectRecord {
    CanonicalObjectRecord {
        id: task_id.to_string(),
        object_type: "task".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision: 1,
        status: "active".to_string(),
        provenance_json: json!({
            "origin": "imported",
            "basis": "todoist_task_mapping",
            "source_refs": [
                TODOIST_MODULE_ID,
                todoist_provider_object_ref("task", &payload.remote_id),
            ],
        }),
        facets_json: task_facets(integration_account_id, payload),
        source_summary_json: None,
        deleted_at: None,
        archived_at: None,
        created_at: imported_at,
        updated_at: imported_at,
    }
}

pub fn task_facets(integration_account_id: &str, payload: &TodoistTaskPayload) -> JsonValue {
    let interpretation = interpret_tags(&payload.labels);

    json!({
        "title": payload.title,
        "description": payload.description,
        "status": if payload.completed { "done" } else { "ready" },
        "priority": map_priority(payload.priority.as_deref()),
        "due": payload.due,
        "task_type": interpretation.task_type,
        "project_ref": payload.project_remote_id.as_ref().map(|remote_id| {
            todoist_project_id(integration_account_id, remote_id).to_string()
        }),
        "parent_task_ref": JsonValue::Null,
        "tags": payload.labels,
        "task_semantics": interpretation.task_semantics,
        "provider_facets": {
            "todoist": {
                "project_id": payload.project_remote_id,
                "section_id": payload.section_remote_id,
                "section_name_snapshot": JsonValue::Null,
                "parent_task_id": payload.parent_remote_id,
                "labels": payload.labels,
                "priority": payload.priority,
                "due": payload.due,
                "is_deleted_upstream": false,
            }
        }
    })
}

fn map_priority(priority: Option<&str>) -> &'static str {
    match priority {
        Some("p0") => "critical",
        Some("p1") => "high",
        Some("p2") => "medium",
        Some("p3") => "low",
        Some("p4") => "lowest",
        _ => "medium",
    }
}

struct TagInterpretation {
    task_type: String,
    task_semantics: JsonValue,
}

fn interpret_tags(labels: &[String]) -> TagInterpretation {
    let mut task_type = "generic".to_string();
    let mut task_semantics = json!({});
    let JsonValue::Object(ref mut semantics) = task_semantics else {
        return TagInterpretation {
            task_type,
            task_semantics,
        };
    };

    for label in labels {
        match label.as_str() {
            "maintain" | "practice" | "ritual" | "chore" => {
                task_type = label.clone();
                semantics.insert(
                    "interpretation_provenance".to_string(),
                    JsonValue::String("inferred_from_tag".to_string()),
                );
            }
            _ if label.starts_with("time:") => {
                semantics.insert(
                    "time_of_day_hint".to_string(),
                    JsonValue::String(label.trim_start_matches("time:").to_string()),
                );
                semantics.insert(
                    "interpretation_provenance".to_string(),
                    JsonValue::String("inferred_from_tag".to_string()),
                );
            }
            _ if label.starts_with("duration:") => {
                if let Some(minutes) = parse_duration_minutes(label.trim_start_matches("duration:"))
                {
                    semantics.insert(
                        "estimated_duration_minutes".to_string(),
                        JsonValue::Number(minutes.into()),
                    );
                    semantics.insert(
                        "interpretation_provenance".to_string(),
                        JsonValue::String("inferred_from_tag".to_string()),
                    );
                }
            }
            _ => {}
        }
    }

    TagInterpretation {
        task_type,
        task_semantics,
    }
}

fn parse_duration_minutes(value: &str) -> Option<u64> {
    value.strip_suffix('m')?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::{map_todoist_task, task_facets, TodoistTaskPayload};
    use time::OffsetDateTime;

    #[test]
    fn todoist_task_mapping_preserves_raw_tags_and_interprets_semantics() {
        let payload = TodoistTaskPayload {
            remote_id: "todo_123".to_string(),
            title: "Morning review".to_string(),
            description: Some("Review inbox and routines".to_string()),
            completed: false,
            priority: Some("p1".to_string()),
            due: None,
            labels: vec![
                "maintain".to_string(),
                "time:morning".to_string(),
                "duration:15m".to_string(),
            ],
            project_remote_id: Some("proj_personal".to_string()),
            parent_remote_id: None,
            section_remote_id: Some("sec_morning".to_string()),
        };

        let mapped = map_todoist_task(
            "task_01mapped",
            "integration_account_primary",
            &payload,
            OffsetDateTime::UNIX_EPOCH,
        );
        let facets = task_facets("integration_account_primary", &payload);

        assert_eq!(mapped.object_type, "task");
        assert_eq!(mapped.facets_json["priority"], "high");
        assert_eq!(mapped.facets_json["task_type"], "maintain");
        assert_eq!(mapped.facets_json["tags"][0], "maintain");
        assert_eq!(
            mapped.facets_json["task_semantics"]["time_of_day_hint"],
            "morning"
        );
        assert_eq!(
            mapped.facets_json["task_semantics"]["estimated_duration_minutes"],
            15
        );
        assert_eq!(
            facets["provider_facets"]["todoist"]["section_id"],
            "sec_morning"
        );
    }
}

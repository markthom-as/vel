use serde_json::{json, Value as JsonValue};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use vel_core::ProjectId;
use vel_storage::CanonicalObjectRecord;

use crate::todoist_ids::{todoist_provider_object_ref, TODOIST_MODULE_ID};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoistSectionFacet {
    pub remote_id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoistProjectPayload {
    pub remote_id: String,
    pub name: String,
    pub color: Option<String>,
    pub is_inbox_project: bool,
    pub sections: Vec<TodoistSectionFacet>,
}

pub fn todoist_project_id(integration_account_id: &str, remote_id: &str) -> ProjectId {
    let mut hasher = Sha256::new();
    hasher.update(format!("{integration_account_id}:todoist:project:{remote_id}").as_bytes());
    let digest = hasher.finalize();
    ProjectId::from(format!("proj_{}", hex::encode(&digest[..12])))
}

pub fn map_todoist_project(
    integration_account_id: &str,
    project: &TodoistProjectPayload,
    imported_at: OffsetDateTime,
) -> CanonicalObjectRecord {
    let project_id = todoist_project_id(integration_account_id, &project.remote_id);

    CanonicalObjectRecord {
        id: project_id.to_string(),
        object_type: "project".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision: 1,
        status: "active".to_string(),
        provenance_json: json!({
            "origin": "imported",
            "basis": "todoist_project_mapping",
            "source_refs": [
                TODOIST_MODULE_ID,
                todoist_provider_object_ref("project", &project.remote_id),
            ],
        }),
        facets_json: project_facets(project),
        source_summary_json: None,
        deleted_at: None,
        archived_at: None,
        created_at: imported_at,
        updated_at: imported_at,
    }
}

pub fn project_facets(project: &TodoistProjectPayload) -> JsonValue {
    json!({
        "name": project.name,
        "container_kind": "project",
        "provider_facets": {
            "todoist": {
                "project_id": project.remote_id,
                "color": project.color,
                "is_inbox_project": project.is_inbox_project,
                // Sections remain non-first-class provider facet metadata in 0.5.
                "sections": project.sections.iter().map(|section| {
                    json!({
                        "section_id": section.remote_id,
                        "name": section.name,
                    })
                }).collect::<Vec<_>>(),
                "section_posture": "non-first-class",
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{
        map_todoist_project, project_facets, todoist_project_id, TodoistProjectPayload,
        TodoistSectionFacet,
    };
    use time::OffsetDateTime;

    #[test]
    fn todoist_projects_map_sections_as_provider_facets() {
        let project = TodoistProjectPayload {
            remote_id: "proj_personal".to_string(),
            name: "Personal".to_string(),
            color: Some("blue".to_string()),
            is_inbox_project: false,
            sections: vec![TodoistSectionFacet {
                remote_id: "sec_morning".to_string(),
                name: "Morning".to_string(),
            }],
        };

        let mapped = map_todoist_project(
            "integration_account_primary",
            &project,
            OffsetDateTime::UNIX_EPOCH,
        );

        assert_eq!(mapped.object_type, "project");
        assert_eq!(
            mapped.facets_json["provider_facets"]["todoist"]["section_posture"],
            "non-first-class"
        );
        assert_eq!(
            mapped.facets_json["provider_facets"]["todoist"]["sections"][0]["section_id"],
            "sec_morning"
        );
        assert_eq!(
            todoist_project_id("integration_account_primary", "proj_personal"),
            todoist_project_id("integration_account_primary", "proj_personal")
        );
        assert_eq!(project_facets(&project)["container_kind"], "project");
    }
}

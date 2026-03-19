//! `vel review` — daily and weekly review views.

use std::collections::HashMap;

use crate::client::ApiClient;
use vel_api_types::{
    ActionItemData, CommitmentData, ProjectFamilyData, ProjectRecordData, ReviewSnapshotData,
};

const TRUNCATE: usize = 50;
const TOP_ACTION_TITLES_LIMIT: usize = 3;

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
struct ProjectReviewCandidate {
    project_id: vel_core::ProjectId,
    slug: String,
    family: ProjectFamilyData,
    open_commitment_count: u32,
}

pub async fn run_today(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let captures_resp = client.list_captures_recent(20, true).await?;
    let captures = captures_resp
        .data
        .expect("list_captures_recent missing data");
    let now = client.get_now().await?.data.expect("get_now missing data");
    let latest_ctx = client
        .get_artifact_latest("context_brief")
        .await
        .ok()
        .and_then(|r| r.data);

    if json {
        let review_counts = summarize_review_snapshot(&now.review_snapshot);
        let out = serde_json::json!({
            "captures_today": captures.len(),
            "captures": captures,
            "latest_context_artifact": latest_ctx,
            "open_action_count": review_counts.0,
            "triage_count": review_counts.1,
            "top_action_titles": top_action_titles(&now.action_items),
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!("=== Review: today ===\n");
    println!("Captures today: {}", captures.len());
    if !captures.is_empty() {
        for c in &captures {
            let content = if c.content_text.len() > TRUNCATE {
                format!("{}...", &c.content_text[..TRUNCATE])
            } else {
                c.content_text.clone()
            };
            println!("  {}  {}  {}", c.capture_id, c.occurred_at, content);
        }
    }
    println!();
    if let Some(Some(ref a)) = latest_ctx {
        println!(
            "Latest context artifact: {}  ({})",
            a.artifact_id, a.storage_uri
        );
    } else {
        println!("Latest context artifact: (none)");
    }
    Ok(())
}

pub async fn run_week(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let captures_resp = client.list_captures_recent(50, false).await?;
    let captures = captures_resp
        .data
        .expect("list_captures_recent missing data");
    let projects = client
        .list_projects()
        .await?
        .data
        .expect("list_projects missing data")
        .projects;
    let open_commitments = client
        .list_commitments(Some("open"), None, None, 500)
        .await?
        .data
        .expect("list_commitments missing data");
    let latest_ctx = client
        .get_artifact_latest("context_brief")
        .await
        .ok()
        .and_then(|r| r.data);

    if json {
        let project_review_candidates =
            build_project_review_candidates(&projects, &open_commitments);
        let out = serde_json::json!({
            "captures_recent": captures.len(),
            "captures": captures,
            "latest_context_artifact": latest_ctx,
            "project_review_candidates": project_review_candidates,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!("=== Review: week (recent {}) ===\n", captures.len());
    println!("Recent captures: {}", captures.len());
    if !captures.is_empty() {
        for c in &captures {
            let content = if c.content_text.len() > TRUNCATE {
                format!("{}...", &c.content_text[..TRUNCATE])
            } else {
                c.content_text.clone()
            };
            println!("  {}  {}  {}", c.capture_id, c.occurred_at, content);
        }
    }
    println!();
    if let Some(Some(ref a)) = latest_ctx {
        println!(
            "Latest context artifact: {}  ({})",
            a.artifact_id, a.storage_uri
        );
    } else {
        println!("Latest context artifact: (none)");
    }
    Ok(())
}

fn summarize_review_snapshot(snapshot: &ReviewSnapshotData) -> (u32, u32) {
    (snapshot.open_action_count, snapshot.triage_count)
}

fn top_action_titles(action_items: &[ActionItemData]) -> Vec<String> {
    action_items
        .iter()
        .take(TOP_ACTION_TITLES_LIMIT)
        .map(|item| item.title.clone())
        .collect()
}

fn build_project_review_candidates(
    projects: &[ProjectRecordData],
    commitments: &[CommitmentData],
) -> Vec<ProjectReviewCandidate> {
    let mut slug_lookup = HashMap::new();
    let mut alias_lookup = HashMap::new();
    let mut counts = vec![0_u32; projects.len()];

    for (index, project) in projects.iter().enumerate() {
        slug_lookup.insert(project.slug.to_lowercase(), index);
    }

    for (index, project) in projects.iter().enumerate() {
        let alias_key = project.name.to_lowercase();
        if !slug_lookup.contains_key(&alias_key) {
            alias_lookup.entry(alias_key).or_insert(index);
        }
    }

    for commitment in commitments {
        let Some(project_key) = commitment
            .project
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            continue;
        };
        let normalized = project_key.to_lowercase();
        let project_index = slug_lookup
            .get(&normalized)
            .copied()
            .or_else(|| alias_lookup.get(&normalized).copied());
        if let Some(index) = project_index {
            counts[index] += 1;
        }
    }

    let mut candidates: Vec<ProjectReviewCandidate> = projects
        .iter()
        .enumerate()
        .filter_map(|(index, project)| {
            let open_commitment_count = counts[index];
            (open_commitment_count > 0).then(|| ProjectReviewCandidate {
                project_id: project.id.clone(),
                slug: project.slug.clone(),
                family: project.family,
                open_commitment_count,
            })
        })
        .collect();

    candidates.sort_by(|left, right| {
        right
            .open_commitment_count
            .cmp(&left.open_commitment_count)
            .then_with(|| left.slug.cmp(&right.slug))
    });
    candidates
}

#[cfg(test)]
mod tests {
    use super::{build_project_review_candidates, summarize_review_snapshot, top_action_titles};
    use serde_json::json;
    use time::OffsetDateTime;
    use vel_api_types::{
        ActionItemData, ActionKindData, ActionStateData, ActionSurfaceData, CommitmentData,
        ProjectFamilyData, ProjectProvisionRequestData, ProjectRecordData, ProjectRootRefData,
        ProjectStatusData, ReviewSnapshotData,
    };
    use vel_core::{ActionItemId, CommitmentId, ProjectId};

    fn sample_project(project_id: &str, slug: &str, name: &str) -> ProjectRecordData {
        ProjectRecordData {
            id: ProjectId::from(project_id.to_string()),
            slug: slug.to_string(),
            name: name.to_string(),
            family: ProjectFamilyData::Work,
            status: ProjectStatusData::Active,
            primary_repo: ProjectRootRefData {
                path: "/tmp/vel".to_string(),
                label: "vel".to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRefData {
                path: "/tmp/notes/vel".to_string(),
                label: "vel".to_string(),
                kind: "notes_root".to_string(),
            },
            secondary_repos: vec![],
            secondary_notes_roots: vec![],
            upstream_ids: Default::default(),
            pending_provision: ProjectProvisionRequestData::default(),
            created_at: OffsetDateTime::UNIX_EPOCH,
            updated_at: OffsetDateTime::UNIX_EPOCH,
            archived_at: None,
        }
    }

    fn sample_commitment(id: &str, project: Option<&str>) -> CommitmentData {
        CommitmentData {
            id: CommitmentId::from(id.to_string()),
            text: format!("commitment-{id}"),
            source_type: "manual".to_string(),
            source_id: None,
            status: "open".to_string(),
            due_at: None,
            project: project.map(ToString::to_string),
            commitment_kind: None,
            created_at: OffsetDateTime::UNIX_EPOCH,
            resolved_at: None,
            metadata: json!({}),
        }
    }

    #[test]
    fn review_today_helpers_surface_phase5_counts() {
        let counts = summarize_review_snapshot(&ReviewSnapshotData {
            open_action_count: 4,
            triage_count: 2,
            projects_needing_review: 1,
        });
        let titles = top_action_titles(&[
            ActionItemData {
                id: ActionItemId::from("act_1".to_string()),
                surface: ActionSurfaceData::Now,
                kind: ActionKindData::NextStep,
                title: "Ship Phase 05".to_string(),
                summary: "Finish the slice".to_string(),
                project_id: None,
                state: ActionStateData::Active,
                rank: 90,
                surfaced_at: OffsetDateTime::UNIX_EPOCH,
                snoozed_until: None,
                evidence: vec![],
            },
            ActionItemData {
                id: ActionItemId::from("act_2".to_string()),
                surface: ActionSurfaceData::Inbox,
                kind: ActionKindData::Review,
                title: "Triage stale thread".to_string(),
                summary: "Open the thread".to_string(),
                project_id: None,
                state: ActionStateData::Active,
                rank: 80,
                surfaced_at: OffsetDateTime::UNIX_EPOCH,
                snoozed_until: None,
                evidence: vec![],
            },
        ]);

        assert_eq!(counts, (4, 2));
        assert_eq!(titles, vec!["Ship Phase 05", "Triage stale thread"]);
    }

    #[test]
    fn review_week_candidates_prefer_typed_project_slug_before_alias() {
        let projects = vec![
            sample_project("proj_vel", "vel", "Vel Runtime"),
            sample_project("proj_runtime", "runtime-core", "Runtime Core"),
        ];
        let commitments = vec![
            sample_commitment("c1", Some("vel")),
            sample_commitment("c2", Some("Vel Runtime")),
            sample_commitment("c3", Some("runtime-core")),
            sample_commitment("c4", Some("Runtime Core")),
            sample_commitment("c5", Some("unknown-project")),
        ];

        let candidates = build_project_review_candidates(&projects, &commitments);

        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].slug, "runtime-core");
        assert_eq!(candidates[0].open_commitment_count, 2);
        assert_eq!(candidates[1].slug, "vel");
        assert_eq!(candidates[1].open_commitment_count, 2);
    }
}

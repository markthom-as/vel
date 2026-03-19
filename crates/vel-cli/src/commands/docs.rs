use serde::{Deserialize, Serialize};
use vel_config::{load_repo_contracts_manifest, ContractsManifest};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct DocEntry {
    category: String,
    title: String,
    path: String,
    description: String,
}

const DOCS_CATALOG_JSON: &str = include_str!("docs_catalog.generated.json");

fn docs_catalog() -> anyhow::Result<Vec<DocEntry>> {
    serde_json::from_str(DOCS_CATALOG_JSON)
        .map_err(|error| anyhow::anyhow!("failed to parse docs catalog: {error}"))
}

fn contracts_manifest() -> anyhow::Result<ContractsManifest> {
    load_repo_contracts_manifest()
        .map_err(|error| anyhow::anyhow!("failed to parse contracts manifest: {error}"))
}

pub fn run(json: bool) -> anyhow::Result<()> {
    let docs = docs_catalog()?;
    let manifest = contracts_manifest()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&docs)?);
        return Ok(());
    }

    println!("Core and daily-use documentation");
    for doc in docs.iter().filter(|entry| entry.category == "core") {
        println!("- {}: {} — {}", doc.title, doc.path, doc.description);
    }
    println!();
    println!("User-specific and setup documentation");
    for doc in docs.iter().filter(|entry| entry.category == "user") {
        println!("- {}: {} — {}", doc.title, doc.path, doc.description);
    }
    println!();
    println!("Published contracts and deeper detail");
    println!(
        "- manifest version {} (live: {}, templates: {}, examples: {}, schemas: {})",
        manifest.version,
        manifest.live_configs.len(),
        manifest.templates.len(),
        manifest.contract_examples.len(),
        manifest.schema_count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{contracts_manifest, docs_catalog};

    #[test]
    fn docs_catalog_points_at_current_authority_docs() {
        let docs = docs_catalog().expect("docs catalog should parse");
        let paths: Vec<&str> = docs.iter().map(|entry| entry.path.as_str()).collect();
        assert!(paths.contains(&"docs/README.md"));
        assert!(paths.contains(&"docs/MASTER_PLAN.md"));
        assert!(paths.contains(
            &"docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md"
        ));
        assert!(
            paths.contains(&"docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md")
        );
        assert!(!paths.contains(&"docs/status.md"));
        assert!(!paths.contains(&"docs/architecture.md"));
    }

    #[test]
    fn contracts_manifest_includes_foundational_contract_examples() {
        let manifest = contracts_manifest().expect("contracts manifest should parse");
        assert!(manifest
            .contract_examples
            .iter()
            .any(|entry| entry.kind == "connector_manifest"));
        assert!(manifest
            .contract_examples
            .iter()
            .any(|entry| entry.kind == "self_model_envelope"));
    }
}

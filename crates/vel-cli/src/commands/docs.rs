use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct DocEntry {
    category: &'static str,
    title: &'static str,
    path: &'static str,
    description: &'static str,
}

const DOCS: &[DocEntry] = &[
    DocEntry {
        category: "core",
        title: "Docs Guide",
        path: "docs/README.md",
        description: "Top-level documentation authority and navigation guide.",
    },
    DocEntry {
        category: "core",
        title: "Status",
        path: "docs/status.md",
        description: "Canonical implementation truth for shipped behavior.",
    },
    DocEntry {
        category: "core",
        title: "Architecture",
        path: "docs/architecture.md",
        description: "High-level system structure and boundaries.",
    },
    DocEntry {
        category: "core",
        title: "Data Model",
        path: "docs/data-model.md",
        description: "Domain and storage model reference.",
    },
    DocEntry {
        category: "user",
        title: "User Docs",
        path: "docs/user/README.md",
        description: "Canonical user-facing entrypoint for operating Vel.",
    },
    DocEntry {
        category: "user",
        title: "Quickstart",
        path: "docs/user/quickstart.md",
        description: "Shortest path to first working local Vel use.",
    },
    DocEntry {
        category: "user",
        title: "Setup",
        path: "docs/user/setup.md",
        description: "Configuration, storage, integrations, and macOS setup.",
    },
    DocEntry {
        category: "user",
        title: "Daily Use",
        path: "docs/user/daily-use.md",
        description: "Repeated daily workflow once Vel is running.",
    },
    DocEntry {
        category: "user",
        title: "Privacy",
        path: "docs/user/privacy.md",
        description: "Local-first trust model and data ownership guide.",
    },
];

pub fn run(json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(DOCS)?);
        return Ok(());
    }

    println!("Core documentation");
    for doc in DOCS.iter().filter(|entry| entry.category == "core") {
        println!("- {}: {} — {}", doc.title, doc.path, doc.description);
    }
    println!();
    println!("User-specific Vel documentation");
    for doc in DOCS.iter().filter(|entry| entry.category == "user") {
        println!("- {}: {} — {}", doc.title, doc.path, doc.description);
    }

    Ok(())
}

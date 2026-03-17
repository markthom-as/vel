use crate::DomainKind;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GlossaryCategory {
    Domain,
    Workflow,
    Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct GlossaryEntry {
    pub slug: &'static str,
    pub term: &'static str,
    pub category: GlossaryCategory,
    pub summary: &'static str,
    pub aliases: &'static [&'static str],
    pub related_terms: &'static [&'static str],
    pub domain_kind: Option<DomainKind>,
    pub dsl_selectors: &'static [&'static str],
    pub dsl_operations: &'static [&'static str],
    pub command_examples: &'static [&'static str],
}

const GLOSSARY: &[GlossaryEntry] = &[
    GlossaryEntry {
        slug: "capture",
        term: "capture",
        category: GlossaryCategory::Domain,
        summary: "A durable user or system input such as a note, transcript-derived item, or imported document fragment.",
        aliases: &["captures", "note", "notes", "inbox item"],
        related_terms: &["artifact", "commitment", "signal"],
        domain_kind: Some(DomainKind::Capture),
        dsl_selectors: &["id", "latest", "recent"],
        dsl_operations: &["create", "inspect", "list", "link", "explain"],
        command_examples: &["vel command should capture remember this for later"],
    },
    GlossaryEntry {
        slug: "commitment",
        term: "commitment",
        category: GlossaryCategory::Domain,
        summary: "An actionable, reviewable item that matters over time and remains distinct from a raw capture.",
        aliases: &["commitments", "todo", "todos", "task", "tasks"],
        related_terms: &["capture", "thread", "nudge"],
        domain_kind: Some(DomainKind::Commitment),
        dsl_selectors: &["id", "open", "due_today", "latest"],
        dsl_operations: &["create", "inspect", "list", "update", "link", "explain"],
        command_examples: &["vel command should commit finish the risk audit"],
    },
    GlossaryEntry {
        slug: "run",
        term: "run",
        category: GlossaryCategory::Domain,
        summary: "A tracked execution record for a run-backed workflow, including lifecycle state and emitted events.",
        aliases: &["runs", "job", "jobs"],
        related_terms: &["artifact", "run event", "context"],
        domain_kind: Some(DomainKind::Run),
        dsl_selectors: &["id", "latest", "today"],
        dsl_operations: &["inspect", "list", "update", "explain"],
        command_examples: &[],
    },
    GlossaryEntry {
        slug: "artifact",
        term: "artifact",
        category: GlossaryCategory::Domain,
        summary: "A durable output or external reference produced by Vel, such as a context brief or synthesis file.",
        aliases: &["artifacts", "output", "outputs"],
        related_terms: &["capture", "run", "ref"],
        domain_kind: Some(DomainKind::Artifact),
        dsl_selectors: &["id", "latest", "type"],
        dsl_operations: &["create", "inspect", "list", "link", "explain"],
        command_examples: &[],
    },
    GlossaryEntry {
        slug: "thread",
        term: "thread",
        category: GlossaryCategory::Domain,
        summary: "A first-class continuity object for an active line of work, conversation, or theme.",
        aliases: &["threads"],
        related_terms: &["commitment", "context", "artifact"],
        domain_kind: Some(DomainKind::Thread),
        dsl_selectors: &["id", "open", "latest"],
        dsl_operations: &["create", "inspect", "list", "update", "link", "explain"],
        command_examples: &[],
    },
    GlossaryEntry {
        slug: "context",
        term: "context",
        category: GlossaryCategory::Workflow,
        summary: "The current orientation picture Vel computes from captures, commitments, integrations, and recent runs.",
        aliases: &["current context", "today context", "brief"],
        related_terms: &["run", "artifact", "review"],
        domain_kind: Some(DomainKind::Context),
        dsl_selectors: &["today", "morning", "end_of_day"],
        dsl_operations: &["inspect", "execute", "explain"],
        command_examples: &["vel command should review today"],
    },
    GlossaryEntry {
        slug: "spec-draft",
        term: "spec draft",
        category: GlossaryCategory::Workflow,
        summary: "A planning artifact for a proposed design or documentation slice that is not implementation truth by itself.",
        aliases: &["spec", "spec_draft", "design doc"],
        related_terms: &["execution plan", "artifact"],
        domain_kind: Some(DomainKind::SpecDraft),
        dsl_selectors: &["topic", "latest"],
        dsl_operations: &["create", "inspect", "list", "explain"],
        command_examples: &["vel command should spec command language glossary"],
    },
    GlossaryEntry {
        slug: "execution-plan",
        term: "execution plan",
        category: GlossaryCategory::Workflow,
        summary: "A planning artifact that breaks implementation work into an ordered slice, distinct from shipped behavior.",
        aliases: &["plan", "execution_plan", "implementation plan"],
        related_terms: &["spec draft", "artifact"],
        domain_kind: Some(DomainKind::ExecutionPlan),
        dsl_selectors: &["topic", "latest"],
        dsl_operations: &["create", "inspect", "list", "explain"],
        command_examples: &["vel command should plan repo glossary integration"],
    },
    GlossaryEntry {
        slug: "should",
        term: "should",
        category: GlossaryCategory::Command,
        summary: "The current structured command-language phrase family for explicit intent-oriented commands.",
        aliases: &["command should"],
        related_terms: &["capture", "commitment", "review"],
        domain_kind: None,
        dsl_selectors: &[],
        dsl_operations: &[],
        command_examples: &["vel command should capture remember the meeting buffer"],
    },
    GlossaryEntry {
        slug: "capture-verb",
        term: "capture",
        category: GlossaryCategory::Command,
        summary: "DSL verb for creating a capture from inline text.",
        aliases: &["note"],
        related_terms: &["capture", "should"],
        domain_kind: None,
        dsl_selectors: &[],
        dsl_operations: &[],
        command_examples: &["vel command should capture remember to review the latest run"],
    },
    GlossaryEntry {
        slug: "feature-verb",
        term: "feature",
        category: GlossaryCategory::Command,
        summary: "DSL verb for creating a typed feature-request capture.",
        aliases: &[],
        related_terms: &["capture", "should"],
        domain_kind: None,
        dsl_selectors: &[],
        dsl_operations: &[],
        command_examples: &["vel command should feature add glossary search"],
    },
    GlossaryEntry {
        slug: "commit-verb",
        term: "commit",
        category: GlossaryCategory::Command,
        summary: "DSL verb for creating an open commitment from inline text.",
        aliases: &["todo"],
        related_terms: &["commitment", "should"],
        domain_kind: None,
        dsl_selectors: &[],
        dsl_operations: &[],
        command_examples: &["vel command should commit write the glossary appendix"],
    },
    GlossaryEntry {
        slug: "review-verb",
        term: "review",
        category: GlossaryCategory::Command,
        summary: "DSL verb for read-oriented review flows such as today or week context review.",
        aliases: &[],
        related_terms: &["context", "should"],
        domain_kind: None,
        dsl_selectors: &[],
        dsl_operations: &[],
        command_examples: &["vel command should review week"],
    },
    GlossaryEntry {
        slug: "spec-verb",
        term: "spec",
        category: GlossaryCategory::Command,
        summary: "DSL verb for creating a planned spec-draft artifact intent.",
        aliases: &[],
        related_terms: &["spec draft", "should"],
        domain_kind: None,
        dsl_selectors: &[],
        dsl_operations: &[],
        command_examples: &["vel command should spec operator glossary API"],
    },
    GlossaryEntry {
        slug: "plan-verb",
        term: "plan",
        category: GlossaryCategory::Command,
        summary: "DSL verb for creating a planned execution-plan artifact intent.",
        aliases: &[],
        related_terms: &["execution plan", "should"],
        domain_kind: None,
        dsl_selectors: &[],
        dsl_operations: &[],
        command_examples: &["vel command should plan command language rollout"],
    },
    GlossaryEntry {
        slug: "delegate-verb",
        term: "delegate",
        category: GlossaryCategory::Command,
        summary: "Reserved DSL verb for delegation-oriented command flows; recognized but not yet executable.",
        aliases: &[],
        related_terms: &["execution plan", "should"],
        domain_kind: None,
        dsl_selectors: &[],
        dsl_operations: &[],
        command_examples: &["vel command should delegate review queue cleanup"],
    },
];

pub const SHOULD_COMMAND_VERBS: &[&str] = &[
    "capture", "feature", "commit", "review", "spec", "plan", "delegate",
];

pub fn glossary_entries() -> &'static [GlossaryEntry] {
    GLOSSARY
}

pub fn glossary_entry(slug: &str) -> Option<&'static GlossaryEntry> {
    GLOSSARY.iter().find(|entry| entry.slug == slug)
}

pub fn glossary_entry_for_kind(kind: DomainKind) -> Option<&'static GlossaryEntry> {
    GLOSSARY
        .iter()
        .find(|entry| entry.domain_kind == Some(kind))
}

pub fn dsl_registry_entries() -> impl Iterator<Item = &'static GlossaryEntry> {
    GLOSSARY.iter().filter(|entry| entry.domain_kind.is_some())
}

pub fn should_command_verb_entries() -> impl Iterator<Item = &'static GlossaryEntry> {
    GLOSSARY.iter().filter(|entry| {
        entry.category == GlossaryCategory::Command && entry.slug.ends_with("-verb")
    })
}

pub fn normalize_should_command_verb(token: &str) -> Option<&'static str> {
    should_command_verb_entries().find_map(|entry| {
        if entry.term == token || entry.aliases.contains(&token) {
            Some(entry.term)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{
        glossary_entries, glossary_entry_for_kind, normalize_should_command_verb,
        should_command_verb_entries, SHOULD_COMMAND_VERBS,
    };
    use crate::DomainKind;

    #[test]
    fn domain_kinds_have_glossary_entries() {
        assert_eq!(
            glossary_entry_for_kind(DomainKind::Capture).map(|entry| entry.term),
            Some("capture")
        );
        assert_eq!(
            glossary_entry_for_kind(DomainKind::Commitment).map(|entry| entry.term),
            Some("commitment")
        );
        assert_eq!(
            glossary_entry_for_kind(DomainKind::ExecutionPlan).map(|entry| entry.term),
            Some("execution plan")
        );
    }

    #[test]
    fn glossary_contains_command_examples() {
        assert!(glossary_entries()
            .iter()
            .any(|entry| !entry.command_examples.is_empty()));
        assert!(SHOULD_COMMAND_VERBS.contains(&"review"));
    }

    #[test]
    fn normalizes_should_command_verb_aliases() {
        let verbs = should_command_verb_entries()
            .map(|entry| entry.term)
            .collect::<Vec<_>>();
        assert!(verbs.contains(&"capture"));
        assert_eq!(normalize_should_command_verb("note"), Some("capture"));
        assert_eq!(normalize_should_command_verb("todo"), Some("commit"));
        assert_eq!(normalize_should_command_verb("unknown"), None);
    }
}

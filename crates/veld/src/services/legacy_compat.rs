use crate::errors::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyCompatDisposition {
    Retired,
    IsolatedReadOnly,
}

pub fn deprecated_write_path_error(
    path: &str,
    canonical_path: Option<&str>,
    note: &str,
) -> AppError {
    let canonical = canonical_path
        .map(|value| format!(" Use {value} instead."))
        .unwrap_or_default();
    AppError::gone(format!(
        "legacy write path {path} is deprecated and no longer active in the 0.5 canonical backend cutover.{canonical} {note}"
    ))
}

pub fn legacy_disposition_note(
    path: &str,
    disposition: LegacyCompatDisposition,
    note: &str,
) -> String {
    let state = match disposition {
        LegacyCompatDisposition::Retired => "retired",
        LegacyCompatDisposition::IsolatedReadOnly => "isolated_read_only",
    };
    format!("{path} => {state}: {note}")
}

#[cfg(test)]
mod tests {
    use super::{deprecated_write_path_error, legacy_disposition_note, LegacyCompatDisposition};

    #[test]
    fn deprecated_write_path_error_names_cutover_and_canonical_replacement() {
        let error = deprecated_write_path_error(
            "/api/integrations/todoist/create-task",
            Some("/api/integrations/todoist/write-intent"),
            "The legacy commitment/writeback shape is quarantined.",
        );

        assert!(error.to_string().contains("0.5 canonical backend cutover"));
        assert!(error
            .to_string()
            .contains("/api/integrations/todoist/write-intent"));
    }

    #[test]
    fn disposition_note_marks_retired_and_isolated_paths() {
        let retired = legacy_disposition_note(
            "/api/integrations/email/send-draft",
            LegacyCompatDisposition::Retired,
            "Not included in 0.5.",
        );
        let isolated = legacy_disposition_note(
            "/api/integrations",
            LegacyCompatDisposition::IsolatedReadOnly,
            "Read-only settings/status remains for operator continuity.",
        );

        assert!(retired.contains("retired"));
        assert!(isolated.contains("isolated_read_only"));
    }
}

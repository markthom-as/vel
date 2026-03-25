use anyhow::{bail, Context};
use serde::Serialize;
use serde_yaml::Value;
use std::collections::BTreeSet;
use std::fs;

const DEFAULT_POLICIES_PATH: &str = "config/policies.yaml";

#[derive(Debug, Clone, Serialize)]
struct PolicyCheckReport {
    ok: bool,
    path: String,
    issues: Vec<String>,
}

pub fn run_check(path: Option<&str>, json: bool) -> anyhow::Result<()> {
    let path = path.unwrap_or(DEFAULT_POLICIES_PATH);
    let report = check_policy_file(path)?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).context("serialize policy check report")?
        );
    } else if report.ok {
        println!("policy check: PASS");
        println!("path: {}", report.path);
    } else {
        println!("policy check: FAIL");
        println!("path: {}", report.path);
        for issue in &report.issues {
            println!("- {}", issue);
        }
    }

    if !report.ok {
        bail!("policy check failed");
    }
    Ok(())
}

fn check_policy_file(path: &str) -> anyhow::Result<PolicyCheckReport> {
    let content = fs::read_to_string(path).with_context(|| format!("read {path}"))?;
    let value: Value = serde_yaml::from_str(&content).with_context(|| format!("parse {path}"))?;

    let mut issues = Vec::new();
    let Some(root) = value.as_mapping() else {
        issues.push("policy root must be a YAML mapping/object".to_string());
        return Ok(PolicyCheckReport {
            ok: false,
            path: path.to_string(),
            issues,
        });
    };

    let allowed = BTreeSet::from([
        "loops".to_string(),
        "queued_work".to_string(),
        "suggestions".to_string(),
        "policies".to_string(),
    ]);

    let actual = root
        .keys()
        .filter_map(Value::as_str)
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();

    for missing in allowed.difference(&actual) {
        issues.push(format!("missing required root key `{missing}`"));
    }
    for unknown in actual.difference(&allowed) {
        issues.push(format!("unknown root key `{unknown}` (fail-closed)"));
    }

    for required_mapping in ["loops", "queued_work", "suggestions", "policies"] {
        match root.get(Value::String(required_mapping.to_string())) {
            Some(Value::Mapping(_)) => {}
            Some(_) => issues.push(format!(
                "root key `{required_mapping}` must be a mapping/object"
            )),
            None => {}
        }
    }

    Ok(PolicyCheckReport {
        ok: issues.is_empty(),
        path: path.to_string(),
        issues,
    })
}

#[cfg(test)]
mod tests {
    use super::check_policy_file;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_policy_file(contents: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        path.push(format!("vel_policy_check_{nonce}.yaml"));
        fs::write(&path, contents).expect("write temp policy file");
        path
    }

    #[test]
    fn policy_check_passes_for_valid_root_shape() {
        let path = temp_policy_file(
            r#"
loops: {}
queued_work: {}
suggestions: {}
policies: {}
"#,
        );
        let report = check_policy_file(path.to_str().expect("utf8 path")).expect("check");
        assert!(report.ok);
        fs::remove_file(path).ok();
    }

    #[test]
    fn policy_check_fails_closed_for_unknown_or_missing_keys() {
        let path = temp_policy_file(
            r#"
loops: {}
queued_work: {}
suggestions: {}
extra_key: {}
"#,
        );
        let report = check_policy_file(path.to_str().expect("utf8 path")).expect("check");
        assert!(!report.ok);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.contains("missing required root key `policies`")));
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.contains("unknown root key `extra_key`")));
        fs::remove_file(path).ok();
    }
}

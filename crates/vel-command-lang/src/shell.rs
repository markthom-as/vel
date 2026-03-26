use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellInputMode {
    Slash,
    Vel,
    SpokenSlash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellInputParse {
    pub mode: ShellInputMode,
    pub raw_text: String,
    pub canonical_text: String,
    pub tokens: Vec<String>,
    pub display_tokens: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellIntentHints {
    pub target_kind: String,
    pub mode: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellParsedCommand {
    pub family: String,
    pub verb: String,
    pub target_tokens: Vec<String>,
    pub source_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellRegistryEntry {
    pub kind: String,
    pub aliases: Vec<String>,
    pub selectors: Vec<String>,
    pub operations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellCompletion {
    pub input: Vec<String>,
    pub completion_hints: Vec<String>,
    pub registry: Vec<ShellRegistryEntry>,
    pub parsed: Option<ShellParsedCommand>,
    pub local_preview: Option<String>,
    pub local_explanation: Option<String>,
    pub intent_hints: Option<ShellIntentHints>,
    pub parse_error: Option<String>,
}

const ROOT_COMMAND_HINTS: [&str; 16] = [
    "morning",
    "standup",
    "checkin",
    "today",
    "capture",
    "recent",
    "search",
    "settings",
    "integrations",
    "components",
    "project",
    "run",
    "thread",
    "people",
    "signals",
    "should",
];

pub fn format_canonical_command(tokens: &[String]) -> String {
    if tokens.is_empty() {
        "vel".to_string()
    } else {
        format!("vel {}", tokens.join(" "))
    }
}

pub fn canonicalize_command_tokens(tokens: &[String]) -> Vec<String> {
    if tokens.len() >= 2 && tokens[0] == "check" && tokens[1] == "in" {
        let mut canonical = vec!["checkin".to_string()];
        canonical.extend(tokens[2..].iter().cloned());
        return canonical;
    }

    tokens
        .iter()
        .enumerate()
        .map(|(index, token)| {
            if index == 0 && matches!(token.as_str(), "check-in" | "check_in") {
                "checkin".to_string()
            } else {
                token.clone()
            }
        })
        .collect()
}

pub fn parse_explicit_command_input(text: &str) -> Option<ShellInputParse> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.starts_with('/') {
        let tokens = trimmed
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let first_token = tokens.first().map(String::as_str).unwrap_or_default();
        if first_token == "/" {
            return Some(ShellInputParse {
                mode: ShellInputMode::Slash,
                raw_text: trimmed.to_string(),
                canonical_text: "vel".to_string(),
                tokens: Vec::new(),
                display_tokens: Vec::new(),
            });
        }

        let command_token = first_token.trim_start_matches('/');
        if command_token.is_empty() || command_token.contains('/') {
            return None;
        }
        if !command_token.chars().enumerate().all(|(idx, ch)| {
            if idx == 0 {
                ch.is_ascii_alphabetic()
            } else {
                ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
            }
        }) {
            return None;
        }

        let display_tokens = std::iter::once(command_token.to_string())
            .chain(tokens.into_iter().skip(1))
            .collect::<Vec<_>>();
        let canonical_tokens = canonicalize_command_tokens(&display_tokens);
        return Some(ShellInputParse {
            mode: ShellInputMode::Slash,
            raw_text: trimmed.to_string(),
            canonical_text: format_canonical_command(&canonical_tokens),
            tokens: canonical_tokens,
            display_tokens,
        });
    }

    if trimmed == "slash" {
        return Some(ShellInputParse {
            mode: ShellInputMode::SpokenSlash,
            raw_text: trimmed.to_string(),
            canonical_text: "vel".to_string(),
            tokens: Vec::new(),
            display_tokens: Vec::new(),
        });
    }

    if let Some(stripped) = trimmed.strip_prefix("slash ") {
        let display_tokens = stripped
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>();
        if display_tokens.is_empty() {
            return None;
        }
        let command_token = display_tokens
            .first()
            .map(String::as_str)
            .unwrap_or_default();
        if !command_token.chars().enumerate().all(|(idx, ch)| {
            if idx == 0 {
                ch.is_ascii_alphabetic()
            } else {
                ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
            }
        }) {
            return None;
        }
        let canonical_tokens = canonicalize_command_tokens(&display_tokens);
        return Some(ShellInputParse {
            mode: ShellInputMode::SpokenSlash,
            raw_text: trimmed.to_string(),
            canonical_text: format_canonical_command(&canonical_tokens),
            tokens: canonical_tokens,
            display_tokens,
        });
    }

    if trimmed == "vel" {
        return Some(ShellInputParse {
            mode: ShellInputMode::Vel,
            raw_text: trimmed.to_string(),
            canonical_text: "vel".to_string(),
            tokens: Vec::new(),
            display_tokens: Vec::new(),
        });
    }

    let Some(stripped) = trimmed.strip_prefix("vel ") else {
        return None;
    };
    let display_tokens = stripped
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let canonical_tokens = canonicalize_command_tokens(&display_tokens);
    Some(ShellInputParse {
        mode: ShellInputMode::Vel,
        raw_text: trimmed.to_string(),
        canonical_text: format_canonical_command(&canonical_tokens),
        tokens: canonical_tokens,
        display_tokens,
    })
}

pub fn explicit_command_name(text: &str) -> Option<String> {
    let command = parse_explicit_command_input(text)?
        .tokens
        .into_iter()
        .next()
        .unwrap_or_default();

    if command.is_empty() || command.contains('/') {
        return None;
    }

    if command.chars().enumerate().all(|(idx, ch)| {
        if idx == 0 {
            ch.is_ascii_alphabetic()
        } else {
            ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
        }
    }) {
        Some(command)
    } else {
        None
    }
}

pub fn shell_completion_for_text(text: &str) -> Option<ShellCompletion> {
    parse_explicit_command_input(text).and_then(|parsed| shell_completion(&parsed.tokens))
}

fn shell_registry() -> Vec<ShellRegistryEntry> {
    vec![
        ShellRegistryEntry {
            kind: "daily_loop_session".to_string(),
            aliases: vec![
                "morning".to_string(),
                "standup".to_string(),
                "checkin".to_string(),
            ],
            selectors: vec!["today".to_string()],
            operations: vec!["execute".to_string(), "resume".to_string()],
        },
        ShellRegistryEntry {
            kind: "vel_command_group".to_string(),
            aliases: vec![
                "settings".to_string(),
                "integrations".to_string(),
                "components".to_string(),
                "project".to_string(),
                "run".to_string(),
                "thread".to_string(),
                "people".to_string(),
                "signals".to_string(),
                "explain".to_string(),
            ],
            selectors: vec!["subcommand".to_string()],
            operations: vec![
                "inspect".to_string(),
                "list".to_string(),
                "execute".to_string(),
            ],
        },
    ]
}

fn subcommand_hints(head: &str) -> Option<&'static [&'static str]> {
    match head {
        "settings" => Some(&["show"]),
        "integrations" => Some(&["show", "connections"]),
        "components" => Some(&["list"]),
        "project" => Some(&["list", "families", "inspect", "create"]),
        "run" => Some(&["intent", "dry-run", "list", "inspect", "status"]),
        "thread" => Some(&["list", "inspect", "close", "reopen", "follow", "reply"]),
        "people" => Some(&["list", "inspect"]),
        "signals" => Some(&["list", "create"]),
        "explain" => Some(&["context", "commitment", "run", "drift", "command"]),
        _ => None,
    }
}

pub fn shell_completion(tokens: &[String]) -> Option<ShellCompletion> {
    let registry = shell_registry();

    match tokens {
        [] => Some(ShellCompletion {
            input: Vec::new(),
            completion_hints: ROOT_COMMAND_HINTS.iter().map(|value| (*value).to_string()).collect(),
            registry,
            parsed: None,
            local_preview: Some("Vel command mode".to_string()),
            local_explanation: Some(
                "Start with morning, standup, checkin, or a typed `should ...` command."
                    .to_string(),
            ),
            intent_hints: None,
            parse_error: None,
        }),
        [head] if head == "morning" => Some(ShellCompletion {
            input: tokens.to_vec(),
            completion_hints: Vec::new(),
            registry,
            parsed: Some(ShellParsedCommand {
                family: "vel".to_string(),
                verb: "morning".to_string(),
                target_tokens: Vec::new(),
                source_text: "vel morning".to_string(),
            }),
            local_preview: Some("Launch or resume the morning daily-loop session.".to_string()),
            local_explanation: Some(
                "Equivalent to `vel morning`. Opens the morning routine for today.".to_string(),
            ),
            intent_hints: Some(ShellIntentHints {
                target_kind: "daily_loop_session".to_string(),
                mode: "execute".to_string(),
                suggestions: vec!["today startup".to_string(), "orientation".to_string()],
            }),
            parse_error: None,
        }),
        [head] if head == "standup" => Some(ShellCompletion {
            input: tokens.to_vec(),
            completion_hints: Vec::new(),
            registry,
            parsed: Some(ShellParsedCommand {
                family: "vel".to_string(),
                verb: "standup".to_string(),
                target_tokens: Vec::new(),
                source_text: "vel standup".to_string(),
            }),
            local_preview: Some("Launch or resume the standup daily-loop session.".to_string()),
            local_explanation: Some(
                "Equivalent to `vel standup`. Opens the standup check-in for today.".to_string(),
            ),
            intent_hints: Some(ShellIntentHints {
                target_kind: "daily_loop_session".to_string(),
                mode: "execute".to_string(),
                suggestions: vec!["status review".to_string(), "blockers".to_string()],
            }),
            parse_error: None,
        }),
        [head] if head == "checkin" => Some(ShellCompletion {
            input: tokens.to_vec(),
            completion_hints: Vec::new(),
            registry,
            parsed: Some(ShellParsedCommand {
                family: "vel".to_string(),
                verb: "checkin".to_string(),
                target_tokens: Vec::new(),
                source_text: "vel checkin".to_string(),
            }),
            local_preview: Some("Resume the active daily-loop session.".to_string()),
            local_explanation: Some(
                "Equivalent to `vel checkin`. Resumes today's active standup first, then morning if available."
                    .to_string(),
            ),
            intent_hints: Some(ShellIntentHints {
                target_kind: "daily_loop_session".to_string(),
                mode: "resume".to_string(),
                suggestions: vec!["active standup".to_string(), "resume morning".to_string()],
            }),
            parse_error: None,
        }),
        [head] if head == "should" => None,
        [head] => {
            if let Some(hints) = subcommand_hints(head) {
                Some(ShellCompletion {
                    input: tokens.to_vec(),
                    completion_hints: hints.iter().map(|value| (*value).to_string()).collect(),
                    registry,
                    parsed: Some(ShellParsedCommand {
                        family: "vel".to_string(),
                        verb: head.clone(),
                        target_tokens: Vec::new(),
                        source_text: format!("vel {head}"),
                    }),
                    local_preview: Some(format!("Vel command group `{head}`.")),
                    local_explanation: Some(format!(
                        "Complete `vel {head}` with one of the supported subcommands."
                    )),
                    intent_hints: None,
                    parse_error: None,
                })
            } else if ROOT_COMMAND_HINTS.contains(&head.as_str()) {
                None
            } else if ROOT_COMMAND_HINTS.iter().any(|value| value.starts_with(head)) {
                Some(ShellCompletion {
                    input: tokens.to_vec(),
                    completion_hints: ROOT_COMMAND_HINTS
                        .iter()
                        .copied()
                        .filter(|value| value.starts_with(head))
                        .map(str::to_string)
                        .collect(),
                    registry,
                    parsed: None,
                    local_preview: Some("Vel command mode".to_string()),
                    local_explanation: Some(
                        "Use a daily-loop command or continue with a typed `should ...` command."
                            .to_string(),
                    ),
                    intent_hints: None,
                    parse_error: None,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        canonicalize_command_tokens, explicit_command_name, parse_explicit_command_input,
        shell_completion, shell_completion_for_text, ShellInputMode,
    };

    #[test]
    fn canonicalizes_check_in_aliases() {
        assert_eq!(
            canonicalize_command_tokens(&["check".to_string(), "in".to_string()]),
            vec!["checkin".to_string()]
        );
        assert_eq!(
            canonicalize_command_tokens(&["check-in".to_string()]),
            vec!["checkin".to_string()]
        );
    }

    #[test]
    fn extracts_explicit_command_names_from_shell_aliases() {
        assert_eq!(
            explicit_command_name("/morning").as_deref(),
            Some("morning")
        );
        assert_eq!(
            explicit_command_name("vel standup").as_deref(),
            Some("standup")
        );
        assert_eq!(
            explicit_command_name("slash check in").as_deref(),
            Some("checkin")
        );
    }

    #[test]
    fn parses_explicit_command_input_into_canonical_vel_text() {
        let slash = parse_explicit_command_input("/check in now").expect("slash parse");
        assert_eq!(slash.mode, ShellInputMode::Slash);
        assert_eq!(slash.canonical_text, "vel checkin now");
        assert_eq!(slash.tokens, vec!["checkin".to_string(), "now".to_string()]);

        let spoken = parse_explicit_command_input("slash morning").expect("spoken parse");
        assert_eq!(spoken.mode, ShellInputMode::SpokenSlash);
        assert_eq!(spoken.canonical_text, "vel morning");

        let vel = parse_explicit_command_input("vel project inspect").expect("vel parse");
        assert_eq!(vel.mode, ShellInputMode::Vel);
        assert_eq!(vel.canonical_text, "vel project inspect");
    }

    #[test]
    fn completes_root_and_group_commands() {
        let root = shell_completion(&[]).expect("root");
        assert!(root.completion_hints.contains(&"morning".to_string()));

        let project = shell_completion(&["project".to_string()]).expect("project");
        assert!(project.completion_hints.contains(&"inspect".to_string()));
    }

    #[test]
    fn completes_shell_text_directly() {
        let completion = shell_completion_for_text("/morning").expect("completion");
        assert_eq!(
            completion.parsed.as_ref().expect("parsed").source_text,
            "vel morning"
        );
    }
}

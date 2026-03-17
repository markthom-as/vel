use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhraseFamily {
    Should,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Verb {
    Capture,
    Feature,
    Commit,
    Remind,
    Review,
    Synthesize,
    Import,
    Explain,
    Spec,
    Plan,
    Delegate,
}

impl Display for Verb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Capture => "capture",
            Self::Feature => "feature",
            Self::Commit => "commit",
            Self::Remind => "remind",
            Self::Review => "review",
            Self::Synthesize => "synthesize",
            Self::Import => "import",
            Self::Explain => "explain",
            Self::Spec => "spec",
            Self::Plan => "plan",
            Self::Delegate => "delegate",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ParsedCommand {
    pub family: PhraseFamily,
    pub verb: Verb,
    pub target_tokens: Vec<String>,
    pub source_text: String,
}

impl ParsedCommand {
    pub fn primary_target(&self) -> Option<&str> {
        self.target_tokens.first().map(String::as_str)
    }

    pub fn joined_target(&self) -> Option<String> {
        if self.target_tokens.is_empty() {
            None
        } else {
            Some(self.target_tokens.join(" "))
        }
    }
}

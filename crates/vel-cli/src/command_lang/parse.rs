use crate::command_lang::ast::{ParsedCommand, PhraseFamily, Verb};
use crate::command_lang::tokenize::tokenize;
use anyhow::{bail, Result};
use vel_core::normalize_should_command_verb;

pub fn parse(input: &[String]) -> Result<ParsedCommand> {
    let tokens = tokenize(input)?;
    let source_text = tokens.join(" ");

    let Some(head) = tokens.first().map(|value| value.as_str()) else {
        bail!("empty command input")
    };

    match head {
        "should" => parse_should(tokens, source_text),
        other => bail!("unsupported command family `{other}`"),
    }
}

fn parse_should(tokens: Vec<String>, source_text: String) -> Result<ParsedCommand> {
    if tokens.len() < 3 {
        bail!("`should` commands require a verb and a target");
    }

    let canonical_verb =
        normalize_should_command_verb(tokens[1].as_str()).unwrap_or(tokens[1].as_str());
    let verb = Verb::from_keyword(canonical_verb)
        .ok_or_else(|| anyhow::anyhow!("unsupported `should` verb `{}`", tokens[1]))?;

    let target_tokens = tokens[2..].to_vec();
    if target_tokens.is_empty() {
        bail!("`should {}` requires a target", verb);
    }

    Ok(ParsedCommand {
        family: PhraseFamily::Should,
        verb,
        target_tokens,
        source_text,
    })
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::command_lang::ast::{PhraseFamily, Verb};

    #[test]
    fn parses_should_capture() {
        let input = vec![
            "should".to_string(),
            "capture".to_string(),
            "remember".to_string(),
        ];
        let parsed = parse(&input).expect("parse");
        assert_eq!(parsed.family, PhraseFamily::Should);
        assert_eq!(parsed.verb, Verb::Capture);
        assert_eq!(parsed.target_tokens, vec!["remember"]);
    }

    #[test]
    fn parses_should_feature() {
        let input = vec![
            "should".to_string(),
            "feature".to_string(),
            "message".to_string(),
            "triage".to_string(),
        ];
        let parsed = parse(&input).expect("parse");
        assert_eq!(parsed.verb, Verb::Feature);
        assert_eq!(parsed.target_tokens, vec!["message", "triage"]);
    }

    #[test]
    fn parses_should_capture_alias_from_shared_vocabulary() {
        let input = vec![
            "should".to_string(),
            "note".to_string(),
            "remember".to_string(),
            "this".to_string(),
        ];
        let parsed = parse(&input).expect("parse");
        assert_eq!(parsed.verb, Verb::Capture);
        assert_eq!(parsed.target_tokens, vec!["remember", "this"]);
    }

    #[test]
    fn parses_should_commit_alias_from_shared_vocabulary() {
        let input = vec![
            "should".to_string(),
            "todo".to_string(),
            "write".to_string(),
            "tests".to_string(),
        ];
        let parsed = parse(&input).expect("parse");
        assert_eq!(parsed.verb, Verb::Commit);
        assert_eq!(parsed.target_tokens, vec!["write", "tests"]);
    }

    #[test]
    fn parses_should_spec() {
        let input = vec![
            "should".to_string(),
            "spec".to_string(),
            "cluster".to_string(),
            "sync".to_string(),
        ];
        let parsed = parse(&input).expect("parse");
        assert_eq!(parsed.verb, Verb::Spec);
        assert_eq!(parsed.target_tokens, vec!["cluster", "sync"]);
    }

    #[test]
    fn parses_should_plan() {
        let input = vec![
            "should".to_string(),
            "plan".to_string(),
            "offline".to_string(),
            "bootstrap".to_string(),
        ];
        let parsed = parse(&input).expect("parse");
        assert_eq!(parsed.verb, Verb::Plan);
        assert_eq!(parsed.target_tokens, vec!["offline", "bootstrap"]);
    }

    #[test]
    fn parses_should_delegate() {
        let input = vec![
            "should".to_string(),
            "delegate".to_string(),
            "queue".to_string(),
            "cleanup".to_string(),
        ];
        let parsed = parse(&input).expect("parse");
        assert_eq!(parsed.verb, Verb::Delegate);
        assert_eq!(parsed.target_tokens, vec!["queue", "cleanup"]);
    }

    #[test]
    fn rejects_unknown_family() {
        let input = vec!["hello".to_string(), "world".to_string()];
        assert!(parse(&input).is_err());
    }
}

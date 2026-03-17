use crate::command_lang::ast::{ParsedCommand, PhraseFamily, Verb};
use crate::command_lang::tokenize::tokenize;
use anyhow::{bail, Result};

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

    let verb = match tokens[1].as_str() {
        "capture" => Verb::Capture,
        "feature" => Verb::Feature,
        "review" => Verb::Review,
        "commit" => Verb::Commit,
        "remind" => Verb::Remind,
        "synthesize" => Verb::Synthesize,
        "import" => Verb::Import,
        "explain" => Verb::Explain,
        "spec" => Verb::Spec,
        "plan" => Verb::Plan,
        "delegate" => Verb::Delegate,
        other => bail!("unsupported `should` verb `{other}`"),
    };

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
        let input = vec!["should".to_string(), "capture".to_string(), "remember".to_string()];
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
    fn rejects_unknown_family() {
        let input = vec!["hello".to_string(), "world".to_string()];
        assert!(parse(&input).is_err());
    }
}

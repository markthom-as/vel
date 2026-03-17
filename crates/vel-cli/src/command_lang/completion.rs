pub fn next_tokens(input: &[String]) -> Vec<&'static str> {
    match input {
        [] => vec!["should"],
        [head] if head == "should" => {
            vec![
                "capture",
                "feature",
                "commit",
                "review",
                "spec",
                "plan",
                "delegate",
            ]
        }
        [head, verb] if head == "should" && verb == "review" => vec!["today", "week"],
        [head, verb] if head == "should" && verb == "spec" => {
            vec!["<topic>", "for", "with"]
        }
        [head, verb] if head == "should" && verb == "plan" => {
            vec!["<goal>", "for", "with"]
        }
        [head, verb] if head == "should" && verb == "commit" => {
            vec!["<text>", "today", "tomorrow"]
        }
        [head, verb] if head == "should" && (verb == "capture" || verb == "feature") => {
            vec!["<text>"]
        }
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::next_tokens;

    #[test]
    fn suggests_spec_and_plan_tails() {
        let spec = next_tokens(&["should".to_string(), "spec".to_string()]);
        assert!(spec.contains(&"<topic>"));
        let plan = next_tokens(&["should".to_string(), "plan".to_string()]);
        assert!(plan.contains(&"<goal>"));
    }
}

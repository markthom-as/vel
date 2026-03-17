pub fn next_tokens(input: &[String]) -> Vec<&'static str> {
    match input {
        [] => vec!["should"],
        [head] if head == "should" => vec!["capture", "feature", "review", "spec", "plan", "delegate"],
        [head, verb] if head == "should" && verb == "review" => vec!["today", "week"],
        [head, verb] if head == "should" && (verb == "capture" || verb == "feature") => {
            vec!["<text>"]
        }
        _ => Vec::new(),
    }
}

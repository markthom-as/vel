use anyhow::{bail, Result};

pub fn tokenize(input: &[String]) -> Result<Vec<String>> {
    if input.is_empty() {
        bail!("provide a sentence command, for example `vel command should capture remember this`");
    }

    Ok(input
        .iter()
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(ToOwned::to_owned)
        .collect())
}

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?s)^\s*---\r?\n(.*?)\s*\r?\n\s*---(?:\r?\n(.*))?$").unwrap();
}

/// Reads and parses frontmatter from the provided string.
/// Returns a tuple of the parsed frontmatter and the remaining content of the document.
pub fn parse_frontmatter<S: AsRef<str>>(
    input: S,
) -> Result<(Frontmatter, String), FrontmatterError> {
    let caps = RE
        .captures(input.as_ref())
        .ok_or(FrontmatterError::InvalidFrontmatter)?;

    // These should be safe to unwrap because they will always exist if there's a match
    let frontmatter_raw = caps.get(1).unwrap().as_str();
    let content = caps.get(2).unwrap().as_str();

    let frontmatter =
        toml::from_str(frontmatter_raw).map_err(|_| FrontmatterError::InvalidFrontmatter)?;

    Ok((frontmatter, content.into()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frontmatter {
    title: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Error)]
pub enum FrontmatterError {
    #[error("invalid frontmatter format")]
    InvalidFrontmatter,
}

#[cfg(test)]
mod tests {
    use super::{parse_frontmatter, FrontmatterError};

    #[test]
    fn parse_simple() -> Result<(), FrontmatterError> {
        let input = r#"
---
title = "test"
---
content
"#;

        let (frontmatter, content) = parse_frontmatter(input)?;

        assert_eq!(frontmatter.title.unwrap(), "test");
        assert_eq!(content, "content\n");

        Ok(())
    }

    #[test]
    fn parse_invalid() {
        let input = "invalid input";

        assert!(parse_frontmatter(input).is_err());
    }
}

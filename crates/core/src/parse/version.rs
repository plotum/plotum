use crate::error::{ParseError, Span};

// Version is represented by "# Version X"

const SUPPORTED_VERSION: &str = "1";

pub(crate) fn is_version_line(line: &str) -> bool {
    line.trim_start().starts_with('#')
}

pub(crate) fn check_version(line: &str, line_number: usize) -> Result<(), ParseError> {
    let text = line.trim();
    let words: Vec<&str> = text.split_whitespace().collect();

    if words == ["#", "plotum", SUPPORTED_VERSION] {
        return Ok(());
    }

    let leading_spaces = line.chars().count() - line.trim_start().chars().count();
    Err(ParseError::InvalidVersion {
        found: text.to_string(),
        span: Span {
            line: line_number,
            column: leading_spaces + 1,
            length: text.chars().count(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_the_supported_version() {
        assert_eq!(check_version("# plotum 1", 1), Ok(()));
        assert_eq!(check_version("  #  plotum   1 ", 1), Ok(()));
    }

    #[test]
    fn rejects_an_unknown_version() {
        let error = check_version("  # plotum 2", 1).unwrap_err();
        assert_eq!(
            error,
            ParseError::InvalidVersion {
                found: "# plotum 2".to_string(),
                span: Span {
                    line: 1,
                    column: 3,
                    length: 10
                }
            }
        );
    }

    #[test]
    fn rejects_a_malformed_version_line() {
        let error = check_version("# mermaid", 1).unwrap_err();
        assert_eq!(
            error.to_string(),
            "1:1: expected `# plotum 1`, found `# mermaid`"
        );
    }
}

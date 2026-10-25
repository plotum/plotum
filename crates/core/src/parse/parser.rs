use super::comment::Comments;
use super::lexer::lex_line;
use super::token::Token;
use crate::error::ParseError;
use crate::ir::{Diagram, Edge};

pub fn parse(source: &str) -> Result<Diagram, ParseError> {
    let mut diagram = Diagram::default();

    let mut comments = Comments::default();

    // Read each line
    for (index, line) in source.lines().enumerate() {
        if comments.skip(line, index + 1) {
            continue;
        }

        let tokens = lex_line(line, index + 1);

        match tokens.as_slice() {
            // Check only here if a -> b exists
            [
                (Token::Ident(from), span),
                (Token::Arrow(kind), _),
                (Token::Ident(to), _),
            ] => {
                diagram.edges.push(Edge {
                    // from : start of first Ident
                    // to : start of the second Ident
                    from: from.to_string(),
                    to: to.to_string(),
                    kind: *kind,
                    span: *span,
                });
            }
            // Anything : Error (for now)
            [(_, span), ..] => return Err(ParseError::ExpectedEdge { span: *span }),
            [] => {}
        }
    }

    comments.finish()?;

    Ok(diagram)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{ArrowKind, Direction, Stroke};

    #[test]
    fn parses_an_edge() {
        let diagram = parse("a -> b").unwrap();
        assert_eq!(diagram.edges[0].from, "a");
    }

    #[test]
    fn keeps_arrow_stroke_and_direction() {
        let diagram = parse("a <-- b").unwrap();
        assert_eq!(
            diagram.edges[0].kind,
            ArrowKind::Pointed {
                stroke: Stroke::Dashed,
                direction: Direction::Backward
            }
        );
    }

    #[test]
    fn skips_line_comments() {
        let diagram = parse("// a comment\na -> b").unwrap();
        assert_eq!(diagram.edges.len(), 1);
    }

    #[test]
    fn skips_block_comments() {
        let source = "/* a comment\n   on several lines */\na -> b\n/* one line */";
        let diagram = parse(source).unwrap();
        assert_eq!(diagram.edges.len(), 1);
    }

    #[test]
    fn rejects_unclosed_block_comment() {
        let error = parse("a -> b\n  /* never closed\nc -> d").unwrap_err();
        assert_eq!(error.to_string(), "2:3: expected `*/` to close the comment");
    }

    #[test]
    fn rejects_an_incomplete_edge() {
        let error = parse("a ->").unwrap_err();
        assert_eq!(error.to_string(), "1:1: expected `a -> b`");
    }
}

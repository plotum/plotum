use super::lexer::{Token, lex_line};
use crate::error::ParseError;
use crate::ir::{Diagram, Edge};

pub fn parse(source: &str) -> Result<Diagram, ParseError> {
    let mut diagram = Diagram::default();

    // Read each line
    for (index, line) in source.lines().enumerate() {

        let tokens = lex_line(line, index + 1);

        match tokens.as_slice() {
            // Check only here if a -> b exists
            [
                (Token::Ident(from), span),
                (Token::Arrow, _),
                (Token::Ident(to), _),
            ] => {
                diagram.edges.push(Edge {
                    // from : start of first Ident
                    // to : start of the second Ident
                    from: from.to_string(),
                    to: to.to_string(),
                    span: *span,
                });
            }
            // Anything : Error (for now)
            [(_, span), ..] => return Err(ParseError::ExpectedEdge { span: *span }),
            [] => {}
        }
    }

    Ok(diagram)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_an_edge() {
        let diagram = parse("a -> b").unwrap();
        assert_eq!(diagram.edges[0].from, "a");
    }

    #[test]
    fn rejects_an_incomplete_edge() {
        let error = parse("a ->").unwrap_err();
        assert_eq!(error.to_string(), "1:1: expected `a -> b`");
    }
}

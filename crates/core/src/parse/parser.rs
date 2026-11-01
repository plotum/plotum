use super::comment::Comments;
use super::edge::{edge, push_edge};
use super::header::set_header;
use super::lexer::lex_line;
use super::node::{box_node, circle_node, push_node};
use super::section::{add_note, open_section};
use super::token::Token;
use super::version::{check_version, is_version_line};
use crate::error::{ParseError, Span};
use crate::ir::Diagram;

pub fn parse(source: &str) -> Result<Diagram, ParseError> {
    let mut diagram = Diagram::default();

    let mut comments = Comments::default();

    // Read each line
    for (index, line) in source.lines().enumerate() {
        // The version line is optional, but only allowed on the first line
        if index == 0 && is_version_line(line) {
            check_version(line, index + 1)?;
            continue;
        }

        if comments.skip(line, index + 1) {
            continue;
        }

        let lexed = lex_line(line, index + 1)?;
        parse_line(lexed, &mut diagram)?;
    }

    comments.finish()?;

    Ok(diagram)
}

// Each arm is one accepted line shape
fn parse_line(lexed: Vec<(Token<'_>, Span)>, diagram: &mut Diagram) -> Result<(), ParseError> {
    use Token::{Arrow, Ident, Quoted, Round, Square, Text};

    // Tokens and their positions in two lists
    let (tokens, spans): (Vec<Token>, Vec<Span>) = lexed.into_iter().unzip();

    // skip empty list
    let Some(&line_span) = spans.first() else {
        return Ok(());
    };

    match tokens.as_slice() {
        // browser (Browser) step=1
        [Ident(id), Round(label), rest @ ..] => {
            let node = circle_node(id, label, line_span);
            push_node(diagram, node, rest, tail_spans(&spans, rest))?;
        }

        // title: Checkout
        [Ident(key @ ("title" | "subtitle" | "footer")), Text(text)] => {
            set_header(diagram, key, text, line_span)?;
        }

        // note: every server reads the same topic
        [Ident("note"), Text(text)] => add_note(diagram, text, line_span)?,

        // section ok "With cache"
        [Ident("section"), Ident(kind), Quoted(label)] => {
            open_section(diagram, kind, label, line_span)?;
        }

        // s1 [Serveur 1 | WebSocket] {brand}
        [Ident(id), Square(body), rest @ ..] => {
            let node = box_node(id, body, line_span);
            push_node(diagram, node, rest, tail_spans(&spans, rest))?;
        }

        // a -> b {red} step=2 : label
        [Ident(from), Arrow(kind), Ident(to), rest @ ..] => {
            let edge = edge(from, *kind, to, line_span);
            push_edge(diagram, edge, rest, tail_spans(&spans, rest))?;
        }

        // `section ok` without label: after the node and edge arms, so `section -> b` stays an edge
        [Ident("section"), ..] => return Err(ParseError::InvalidSection { span: line_span }),

        // Any other line shape
        _ => return Err(ParseError::UnknownLine { span: line_span }),
    }

    Ok(())
}

// The spans of `rest`, the last tokens of the line
fn tail_spans<'a>(spans: &'a [Span], rest: &[Token<'_>]) -> &'a [Span] {
    spans.get(spans.len() - rest.len()..).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Edge;

    fn edges(diagram: &Diagram) -> Vec<&Edge> {
        diagram.sections.iter().flat_map(|s| &s.edges).collect()
    }

    #[test]
    fn skips_line_comments() {
        let diagram = parse("// a comment\na -> b").unwrap();
        assert_eq!(edges(&diagram).len(), 1);
    }

    #[test]
    fn skips_block_comments() {
        let source = "/* a comment\n   on several lines */\na -> b\n/* one line */";
        let diagram = parse(source).unwrap();
        assert_eq!(edges(&diagram).len(), 1);
    }

    #[test]
    fn rejects_unclosed_block_comment() {
        let error = parse("a -> b\n  /* never closed\nc -> d").unwrap_err();
        assert_eq!(error.to_string(), "2:3: expected `*/` to close the comment");
    }

    #[test]
    fn accepts_a_version_line_first() {
        let diagram = parse("# plotum 1\na -> b").unwrap();
        assert_eq!(edges(&diagram).len(), 1);
    }

    #[test]
    fn rejects_an_unsupported_version() {
        let error = parse("# plotum 2\na -> b").unwrap_err();
        assert_eq!(
            error.to_string(),
            "1:1: expected `# plotum 1`, found `# plotum 2`"
        );
    }

    #[test]
    fn rejects_a_version_line_after_the_first_line() {
        let error = parse("a -> b\n# plotum 1").unwrap_err();
        assert!(matches!(error, ParseError::UnknownLine { .. }));
    }
}

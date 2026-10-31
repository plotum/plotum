use super::comment::Comments;
use super::header::set_header;
use super::lexer::lex_line;
use super::section::{current_section, open_section};
use super::token::Token;
use super::version::{check_version, is_version_line};
use crate::error::{ParseError, Span};
use crate::ir::{ArrowKind, Diagram, Edge, Node, Shape};

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
        // browser (Browser)
        [Ident(id), Round(label)] => {
            let node = circle_node(id, label, line_span);
            current_section(diagram, line_span).nodes.push(node);
        }

        // title: Checkout
        [Ident(key @ ("title" | "subtitle" | "footer")), Text(text)] => {
            set_header(diagram, key, text, line_span)?;
        }

        // section ok "With cache"
        [Ident("section"), Ident(kind), Quoted(label)] => {
            open_section(diagram, kind, label, line_span)?;
        }

        // s1 [Serveur 1 | WebSocket]
        [Ident(id), Square(body)] => {
            let node = box_node(id, body, line_span);
            current_section(diagram, line_span).nodes.push(node);
        }

        // a -> b
        [Ident(from), Arrow(kind), Ident(to)] => {
            let edge = edge(from, *kind, to, line_span);
            current_section(diagram, line_span).edges.push(edge);
        }

        // a -> b : label
        [Ident(from), Arrow(kind), Ident(to), Text(label)] => {
            let mut labelled = edge(from, *kind, to, line_span);
            labelled.label = Some(label.to_string());
            current_section(diagram, line_span).edges.push(labelled);
        }

        // `section ok` without label: after the node and edge arms, so `section -> b` stays an edge
        [Ident("section"), ..] => return Err(ParseError::InvalidSection { span: line_span }),

        // Any other line shape
        _ => return Err(ParseError::UnknownLine { span: line_span }),
    }

    Ok(())
}

fn circle_node(id: &str, label: &str, span: Span) -> Node {
    Node {
        id: id.to_string(),
        label: label.trim().to_string(),
        shape: Shape::Circle,
        span,
    }
}

// `Serveur 1 | WebSocket` gives the label "Serveur 1" and the subtext "WebSocket"
fn box_node(id: &str, body: &str, span: Span) -> Node {
    let (label, subtext) = match body.split_once('|') {
        Some((label, subtext)) => (label, Some(subtext.trim().to_string())),
        None => (body, None),
    };

    Node {
        id: id.to_string(),
        label: label.trim().to_string(),
        shape: Shape::Box { subtext },
        span,
    }
}

fn edge(from: &str, kind: ArrowKind, to: &str, span: Span) -> Edge {
    Edge {
        from: from.to_string(),
        to: to.to_string(),
        kind,
        span,
        // label is add after creation (not required)
        label: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Direction, Stroke};

    fn nodes(diagram: &Diagram) -> Vec<&Node> {
        diagram.sections.iter().flat_map(|s| &s.nodes).collect()
    }

    fn edges(diagram: &Diagram) -> Vec<&Edge> {
        diagram.sections.iter().flat_map(|s| &s.edges).collect()
    }

    #[test]
    fn parses_an_edge() {
        let diagram = parse("a -> b").unwrap();
        assert_eq!(edges(&diagram)[0].from, "a");
        assert_eq!(edges(&diagram)[0].label, None);
    }

    #[test]
    fn keeps_arrow_stroke_and_direction() {
        let diagram = parse("a <-- b").unwrap();
        assert_eq!(
            edges(&diagram)[0].kind,
            ArrowKind::Pointed {
                stroke: Stroke::Dashed,
                direction: Direction::Backward
            }
        );
    }

    #[test]
    fn parses_an_edge_label_with_colons() {
        let diagram = parse("a -> b : HTTP 200 : OK").unwrap();
        assert_eq!(edges(&diagram)[0].label.as_deref(), Some("HTTP 200 : OK"));
    }

    #[test]
    fn parses_a_circle_node() {
        let diagram = parse("alice (Alice B)").unwrap();
        assert_eq!(nodes(&diagram)[0].label, "Alice B");
        assert_eq!(nodes(&diagram)[0].shape, Shape::Circle);
    }

    #[test]
    fn parses_a_box_node_with_subtext() {
        let diagram = parse("s1 [Serveur 1 | WebSocket]").unwrap();
        assert_eq!(nodes(&diagram)[0].label, "Serveur 1");
        assert_eq!(
            nodes(&diagram)[0].shape,
            Shape::Box {
                subtext: Some("WebSocket".to_string())
            }
        );
    }

    #[test]
    fn parses_a_box_node_without_subtext() {
        let diagram = parse("api [API]").unwrap();
        assert_eq!(nodes(&diagram)[0].shape, Shape::Box { subtext: None });
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

    #[test]
    fn rejects_an_incomplete_edge() {
        let error = parse("a ->").unwrap_err();
        assert_eq!(
            error,
            ParseError::UnknownLine {
                span: Span {
                    line: 1,
                    column: 1,
                    length: 1
                }
            }
        );
    }

    #[test]
    fn rejects_trailing_tokens_after_an_edge() {
        let error = parse("a -> b c").unwrap_err();
        assert_eq!(
            error,
            ParseError::UnknownLine {
                span: Span {
                    line: 1,
                    column: 1,
                    length: 1
                }
            }
        );
    }

    #[test]
    fn rejects_an_unclosed_node_label() {
        let error = parse("api [API").unwrap_err();
        assert_eq!(error.to_string(), "1:5: expected `]` to close this");
    }
}

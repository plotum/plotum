use super::modifier::{Target, parse_modifiers};
use super::section::current_section;
use super::token::Token;
use crate::error::{ParseError, Span};
use crate::ir::{ArrowKind, Diagram, Edge};

// Everything after `:` is the label, so modifiers come before it
pub(crate) fn push_edge(
    diagram: &mut Diagram,
    mut edge: Edge,
    rest: &[Token<'_>],
    spans: &[Span],
) -> Result<(), ParseError> {
    let modifiers = match rest {
        [modifiers @ .., Token::Text(label)] => {
            edge.label = Some(label.to_string());
            modifiers
        }
        _ => rest,
    };

    parse_modifiers(modifiers, spans, Target::Edge)?.apply_to_edge(&mut edge);
    current_section(diagram, edge.span).edges.push(edge);
    Ok(())
}

pub(crate) fn edge(from: &str, kind: ArrowKind, to: &str, span: Span) -> Edge {
    Edge {
        from: from.to_string(),
        to: to.to_string(),
        kind,
        span,
        // label and modifiers are set after creation (not required)
        label: None,
        color: None,
        step: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{ParseError, Span};
    use crate::ir::{ArrowKind, Direction, Stroke};
    use crate::parse::parse;

    #[test]
    fn parses_an_edge() {
        let diagram = parse("a -> b").unwrap();
        let edge = &diagram.sections[0].edges[0];
        assert_eq!(edge.from, "a");
        assert_eq!(edge.label, None);
    }

    #[test]
    fn keeps_arrow_stroke_and_direction() {
        let diagram = parse("a <-- b").unwrap();
        assert_eq!(
            diagram.sections[0].edges[0].kind,
            ArrowKind::Pointed {
                stroke: Stroke::Dashed,
                direction: Direction::Backward
            }
        );
    }

    #[test]
    fn parses_an_edge_label_with_colons() {
        let diagram = parse("a -> b : HTTP 200 : OK").unwrap();
        assert_eq!(
            diagram.sections[0].edges[0].label.as_deref(),
            Some("HTTP 200 : OK")
        );
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
            ParseError::UnknownModifier {
                span: Span {
                    line: 1,
                    column: 8,
                    length: 1
                }
            }
        );
    }
}

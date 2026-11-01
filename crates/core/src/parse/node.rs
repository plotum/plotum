use super::modifier::{Target, parse_modifiers};
use super::section::current_section;
use super::token::Token;
use crate::error::{ParseError, Span};
use crate::ir::{Diagram, Node, Shape};

pub(crate) fn push_node(
    diagram: &mut Diagram,
    mut node: Node,
    modifiers: &[Token<'_>],
    spans: &[Span],
) -> Result<(), ParseError> {
    parse_modifiers(modifiers, spans, Target::Node)?.apply_to_node(&mut node);
    current_section(diagram, node.span).nodes.push(node);
    Ok(())
}

pub(crate) fn circle_node(id: &str, label: &str, span: Span) -> Node {
    node(id, label, Shape::Circle, span)
}

// `Serveur 1 | WebSocket` gives the label "Serveur 1" and the subtext "WebSocket"
pub(crate) fn box_node(id: &str, body: &str, span: Span) -> Node {
    let (label, subtext) = match body.split_once('|') {
        Some((label, subtext)) => (label, Some(subtext.trim().to_string())),
        None => (body, None),
    };

    node(id, label, Shape::Box { subtext }, span)
}

fn node(id: &str, label: &str, shape: Shape, span: Span) -> Node {
    Node {
        id: id.to_string(),
        label: label.trim().to_string(),
        shape,
        // modifiers are set after creation, by `push_node`
        color: None,
        icon: None,
        pinned: None,
        step: None,
        span,
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::Shape;
    use crate::parse::parse;

    #[test]
    fn parses_a_circle_node() {
        let diagram = parse("alice (Alice B)").unwrap();
        let node = &diagram.sections[0].nodes[0];
        assert_eq!(node.label, "Alice B");
        assert_eq!(node.shape, Shape::Circle);
    }

    #[test]
    fn parses_a_box_node_with_subtext() {
        let diagram = parse("s1 [Serveur 1 | WebSocket]").unwrap();
        let node = &diagram.sections[0].nodes[0];
        assert_eq!(node.label, "Serveur 1");
        assert_eq!(
            node.shape,
            Shape::Box {
                subtext: Some("WebSocket".to_string())
            }
        );
    }

    #[test]
    fn parses_a_box_node_without_subtext() {
        let diagram = parse("api [API]").unwrap();
        assert_eq!(
            diagram.sections[0].nodes[0].shape,
            Shape::Box { subtext: None }
        );
    }

    #[test]
    fn rejects_an_unclosed_node_label() {
        let error = parse("api [API").unwrap_err();
        assert_eq!(error.to_string(), "1:5: expected `]` to close this");
    }
}

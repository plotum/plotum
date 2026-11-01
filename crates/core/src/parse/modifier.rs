use super::token::Token;
use crate::error::{ParseError, Span};
use crate::ir::{Edge, GridPos, Node};

// `icon=` and `@col,row` only make sense on a node
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    Node,
    Edge,
}

// `db [PostgreSQL] icon=database {brand} step=2` gives the modifiers after the node
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Modifiers {
    color: Option<String>,
    icon: Option<String>,
    pinned: Option<GridPos>,
    step: Option<u16>,
}

pub(crate) fn parse_modifiers(
    tokens: &[Token<'_>],
    spans: &[Span],
    target: Target,
) -> Result<Modifiers, ParseError> {
    let mut modifiers = Modifiers::default();
    for (token, &span) in tokens.iter().zip(spans) {
        modifiers.read(token, span, target)?;
    }
    Ok(modifiers)
}

impl Modifiers {
    pub(crate) fn apply_to_node(self, node: &mut Node) {
        node.color = self.color;
        node.icon = self.icon;
        node.pinned = self.pinned;
        node.step = self.step;
    }

    // `icon` and `pinned` are always empty here, `parse_modifiers` rejects them on an edge
    pub(crate) fn apply_to_edge(self, edge: &mut Edge) {
        edge.color = self.color;
        edge.step = self.step;
    }

    fn read(&mut self, token: &Token<'_>, span: Span, target: Target) -> Result<(), ParseError> {
        match token {
            Token::Style(color) => {
                let color = non_empty(color, "color", "a color like `{brand}`", span)?;
                set_once(&mut self.color, "color", color, span)
            }
            Token::Ident(word) => self.read_word(word, span, target),
            _ => Err(ParseError::UnknownModifier { span }),
        }
    }

    fn read_word(&mut self, word: &str, span: Span, target: Target) -> Result<(), ParseError> {
        if let Some(name) = word.strip_prefix("icon=") {
            only_on_node("icon", target, span)?;
            let icon = non_empty(name, "icon", "a name like `icon=database`", span)?;
            return set_once(&mut self.icon, "icon", icon, span);
        }

        if let Some(value) = word.strip_prefix("step=") {
            let step = value
                .parse()
                .map_err(|_| invalid("step", "a whole number like `step=2`", span))?;
            return set_once(&mut self.step, "step", step, span);
        }

        if let Some(position) = word.strip_prefix('@') {
            only_on_node("@col,row", target, span)?;
            let pinned = grid_pos(position)
                .ok_or_else(|| invalid("@col,row", "two whole numbers like `@2,0`", span))?;
            return set_once(&mut self.pinned, "@col,row", pinned, span);
        }

        Err(ParseError::UnknownModifier { span })
    }
}

// `2,0` gives column 2, row 0
fn grid_pos(text: &str) -> Option<GridPos> {
    let (col, row) = text.split_once(',')?;
    Some(GridPos {
        col: col.parse().ok()?,
        row: row.parse().ok()?,
    })
}

fn set_once<T>(
    slot: &mut Option<T>,
    modifier: &'static str,
    value: T,
    span: Span,
) -> Result<(), ParseError> {
    if slot.is_some() {
        return Err(ParseError::DuplicateModifier { modifier, span });
    }
    *slot = Some(value);
    Ok(())
}

fn only_on_node(modifier: &'static str, target: Target, span: Span) -> Result<(), ParseError> {
    match target {
        Target::Node => Ok(()),
        Target::Edge => Err(ParseError::ModifierOnEdge { modifier, span }),
    }
}

fn non_empty(
    text: &str,
    modifier: &'static str,
    expected: &'static str,
    span: Span,
) -> Result<String, ParseError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(invalid(modifier, expected, span));
    }
    Ok(text.to_string())
}

fn invalid(modifier: &'static str, expected: &'static str, span: Span) -> ParseError {
    ParseError::InvalidModifier {
        modifier,
        expected,
        span,
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ParseError;
    use crate::ir::GridPos;
    use crate::parse::parse;

    #[test]
    fn reads_every_node_modifier_in_any_order() {
        let diagram = parse("k [Kafka] step=2 icon=kafka {hero} @1,3").unwrap();
        let node = &diagram.sections[0].nodes[0];
        assert_eq!(node.color.as_deref(), Some("hero"));
        assert_eq!(node.icon.as_deref(), Some("kafka"));
        assert_eq!(node.pinned, Some(GridPos { col: 1, row: 3 }));
        assert_eq!(node.step, Some(2));
    }

    #[test]
    fn reads_modifiers_on_a_circle_node() {
        let diagram = parse("client (Client) @0,0").unwrap();
        let node = &diagram.sections[0].nodes[0];
        assert_eq!(node.pinned, Some(GridPos { col: 0, row: 0 }));
    }

    #[test]
    fn leaves_absent_modifiers_empty() {
        let diagram = parse("api [API]").unwrap();
        let node = &diagram.sections[0].nodes[0];
        assert_eq!((node.color.as_ref(), node.step), (None, None));
    }

    #[test]
    fn reads_edge_modifiers_before_the_label() {
        let diagram = parse("k -> s2 {brand} step=3 : consume {not a color}").unwrap();
        let edge = &diagram.sections[0].edges[0];
        assert_eq!(edge.color.as_deref(), Some("brand"));
        assert_eq!(edge.step, Some(3));
        assert_eq!(edge.label.as_deref(), Some("consume {not a color}"));
    }

    #[test]
    fn reads_edge_modifiers_without_label() {
        let diagram = parse("api -> db {#22C55E}").unwrap();
        let edge = &diagram.sections[0].edges[0];
        assert_eq!(edge.color.as_deref(), Some("#22C55E"));
    }

    #[test]
    fn rejects_an_unknown_modifier() {
        let error = parse("api [API] big").unwrap_err();
        assert_eq!(
            error.to_string(),
            "1:11: expected a modifier (`{color}`, `@col,row`, `icon=name`, `step=N`)"
        );
    }

    #[test]
    fn rejects_a_step_that_is_not_a_number() {
        let error = parse("api [API] step=two").unwrap_err();
        assert_eq!(
            error.to_string(),
            "1:11: expected a whole number like `step=2` for `step`"
        );
    }

    #[test]
    fn rejects_a_malformed_grid_position() {
        let error = parse("api [API] @2").unwrap_err();
        assert!(matches!(
            error,
            ParseError::InvalidModifier {
                modifier: "@col,row",
                ..
            }
        ));
    }

    #[test]
    fn rejects_an_empty_color() {
        let error = parse("api [API] {}").unwrap_err();
        assert!(matches!(
            error,
            ParseError::InvalidModifier {
                modifier: "color",
                ..
            }
        ));
    }

    #[test]
    fn rejects_a_duplicate_modifier() {
        let error = parse("api [API] step=1 step=2").unwrap_err();
        assert_eq!(
            error.to_string(),
            "1:18: expected only one `step` modifier, found a second one"
        );
    }

    #[test]
    fn rejects_node_only_modifiers_on_an_edge() {
        let error = parse("a -> b icon=database").unwrap_err();
        assert_eq!(
            error.to_string(),
            "1:8: expected `icon` only on a node, found it on an edge"
        );
    }
}

use crate::error::{ParseError, Span};
use crate::ir::{Diagram, Section, SectionKind};

// `section bad "Without cache"` opens a section that lasts until the next one
pub(crate) fn open_section(
    diagram: &mut Diagram,
    kind: &str,
    label: &str,
    span: Span,
) -> Result<(), ParseError> {
    let kind = section_kind(kind).ok_or_else(|| ParseError::UnknownSectionKind {
        found: kind.to_string(),
        span,
    })?;

    diagram.sections.push(Section {
        kind,
        label: Some(label.trim().to_string()),
        nodes: Vec::new(),
        edges: Vec::new(),
        span,
    });
    Ok(())
}

// Nodes and edges written before any `section` go to an implicit plain section
pub(crate) fn current_section(diagram: &mut Diagram, span: Span) -> &mut Section {
    if diagram.sections.is_empty() {
        diagram.sections.push(Section {
            kind: SectionKind::Plain,
            label: None,
            nodes: Vec::new(),
            edges: Vec::new(),
            span,
        });
    }

    let last = diagram.sections.len() - 1;
    &mut diagram.sections[last]
}

fn section_kind(word: &str) -> Option<SectionKind> {
    match word {
        "bad" => Some(SectionKind::Bad),
        "ok" => Some(SectionKind::Ok),
        "info" => Some(SectionKind::Info),
        "warn" => Some(SectionKind::Warn),
        "plain" => Some(SectionKind::Plain),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ParseError;
    use crate::ir::SectionKind;
    use crate::parse::parse;

    #[test]
    fn opens_a_section_with_its_nodes_and_edges() {
        let diagram = parse("section ok \"With cache\"\n  api [API]\n  api -> redis").unwrap();
        let section = &diagram.sections[0];
        assert_eq!(section.kind, SectionKind::Ok);
        assert_eq!(section.label.as_deref(), Some("With cache"));
        assert_eq!(section.nodes.len(), 1);
        assert_eq!(section.edges.len(), 1);
    }

    #[test]
    fn a_section_lasts_until_the_next_one() {
        let source = "section bad \"A\"\na -> b\nsection ok \"B\"\nc -> d\ne -> f";
        let diagram = parse(source).unwrap();
        assert_eq!(diagram.sections[0].edges.len(), 1);
        assert_eq!(diagram.sections[1].edges.len(), 2);
    }

    #[test]
    fn puts_items_before_any_section_in_an_implicit_one() {
        let diagram = parse("a -> b\nsection info \"Kafka\"\nk [Kafka]").unwrap();
        assert_eq!(diagram.sections.len(), 2);
        assert_eq!(diagram.sections[0].kind, SectionKind::Plain);
        assert_eq!(diagram.sections[0].label, None);
    }

    #[test]
    fn an_empty_diagram_has_no_section() {
        let diagram = parse("// nothing yet").unwrap();
        assert!(diagram.sections.is_empty());
    }

    #[test]
    fn still_accepts_a_node_named_section() {
        let diagram = parse("section (Section)\nsection -> b").unwrap();
        assert_eq!(diagram.sections[0].nodes[0].id, "section");
        assert_eq!(diagram.sections[0].edges[0].from, "section");
    }

    #[test]
    fn rejects_an_unknown_section_kind() {
        let error = parse("section danger \"X\"").unwrap_err();
        assert_eq!(
            error.to_string(),
            "1:1: expected a section type (`bad`, `ok`, `info`, `warn`, `plain`), found `danger`"
        );
    }

    #[test]
    fn rejects_a_section_without_label() {
        let error = parse("section ok").unwrap_err();
        assert!(matches!(error, ParseError::InvalidSection { .. }));
    }
}

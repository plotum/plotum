use crate::error::{ParseError, Span};
use crate::ir::{Diagram, Note, Section, SectionKind};

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
        notes: Vec::new(),
        wrap: None,
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
            notes: Vec::new(),
            wrap: None,
            span,
        });
    }

    let last = diagram.sections.len() - 1;
    &mut diagram.sections[last]
}

pub(crate) fn add_note(diagram: &mut Diagram, text: &str, span: Span) -> Result<(), ParseError> {
    if text.is_empty() {
        return Err(ParseError::EmptyNote { span });
    }

    let note = Note {
        text: text.to_string(),
        span,
    };
    current_section(diagram, span).notes.push(note);
    Ok(())
}

// `wrap: 4` puts 4 nodes per row, then the next row goes the other way
pub(crate) fn set_wrap(diagram: &mut Diagram, text: &str, span: Span) -> Result<(), ParseError> {
    let wrap = text.parse().map_err(|_| ParseError::InvalidWrap { span })?;

    let section = current_section(diagram, span);
    if section.wrap.is_some() {
        return Err(ParseError::DuplicateWrap { span });
    }
    section.wrap = Some(wrap);
    Ok(())
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
    use std::num::NonZeroU16;

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
    fn adds_a_note_to_the_current_section() {
        let source =
            "section info \"Kafka\"\n  k [Kafka]\n  note: every server reads : the same topic";
        let diagram = parse(source).unwrap();
        let note = &diagram.sections[0].notes[0];
        assert_eq!(note.text, "every server reads : the same topic");
        assert_eq!(note.span.line, 3);
    }

    #[test]
    fn puts_a_note_before_any_section_in_the_implicit_one() {
        let diagram = parse("note: hello").unwrap();
        assert_eq!(diagram.sections[0].kind, SectionKind::Plain);
        assert_eq!(diagram.sections[0].notes.len(), 1);
    }

    #[test]
    fn still_accepts_a_node_named_note() {
        let diagram = parse("note (Note)").unwrap();
        assert_eq!(diagram.sections[0].nodes[0].id, "note");
    }

    #[test]
    fn rejects_an_empty_note() {
        let error = parse("note:").unwrap_err();
        assert_eq!(error.to_string(), "1:1: expected a text after `note:`");
    }

    #[test]
    fn sets_the_wrap_of_the_current_section() {
        let diagram = parse("section info \"A\"\n  a [A]\nsection ok \"B\"\n  wrap: 4").unwrap();
        assert_eq!(diagram.sections[0].wrap, None);
        assert_eq!(diagram.sections[1].wrap, NonZeroU16::new(4));
    }

    #[test]
    fn rejects_a_wrap_that_is_not_a_number() {
        let error = parse("wrap: four").unwrap_err();
        assert_eq!(
            error.to_string(),
            "1:1: expected a whole number above 0 after `wrap:`, like `wrap: 4`"
        );
    }

    #[test]
    fn rejects_a_zero_wrap() {
        let error = parse("wrap: 0").unwrap_err();
        assert!(matches!(error, ParseError::InvalidWrap { .. }));
    }

    #[test]
    fn rejects_a_second_wrap_in_the_same_section() {
        let error = parse("section ok \"A\"\n  wrap: 2\n  wrap: 3").unwrap_err();
        assert!(matches!(error, ParseError::DuplicateWrap { span } if span.line == 3));
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

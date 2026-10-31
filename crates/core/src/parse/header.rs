use crate::error::{ParseError, Span};
use crate::ir::Diagram;

pub(crate) fn set_header(
    diagram: &mut Diagram,
    key: &str,
    text: &str,
    span: Span,
) -> Result<(), ParseError> {
    let key_owned = || key.to_string();

    // Header have to be configure in the top's file
    if !diagram.nodes.is_empty() || !diagram.edges.is_empty() {
        return Err(ParseError::HeaderAfterBody {
            key: key_owned(),
            span,
        });
    }

    if text.is_empty() {
        return Err(ParseError::EmptyHeader {
            key: key_owned(),
            span,
        });
    }

    let field = match key {
        "title" => &mut diagram.header.title,
        "subtitle" => &mut diagram.header.subtitle,
        "footer" => &mut diagram.header.footer,
        _ => return Err(ParseError::UnknownLine { span }),
    };

    if field.is_some() {
        return Err(ParseError::DuplicateHeader {
            key: key_owned(),
            span,
        });
    }

    *field = Some(text.to_string());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::error::ParseError;
    use crate::parse::parse;

    #[test]
    fn reads_every_header_field() {
        let diagram = parse("title: Checkout\nsubtitle: Payments\nfooter: @remy").unwrap();
        assert_eq!(diagram.header.title.as_deref(), Some("Checkout"));
        assert_eq!(diagram.header.subtitle.as_deref(), Some("Payments"));
        assert_eq!(diagram.header.footer.as_deref(), Some("@remy"));
    }

    #[test]
    fn keeps_colons_inside_the_text() {
        let diagram = parse("title: Chat : v2").unwrap();
        assert_eq!(diagram.header.title.as_deref(), Some("Chat : v2"));
    }

    #[test]
    fn accepts_a_header_after_the_version_and_comments() {
        let diagram = parse("# plotum 1\n// intro\ntitle: Checkout\na -> b").unwrap();
        assert_eq!(diagram.header.title.as_deref(), Some("Checkout"));
    }

    #[test]
    fn leaves_missing_fields_empty() {
        let diagram = parse("a -> b").unwrap();
        assert_eq!(diagram.header.title, None);
    }

    #[test]
    fn rejects_an_empty_text() {
        let error = parse("title:").unwrap_err();
        assert_eq!(error.to_string(), "1:1: expected a text after `title:`");
    }

    #[test]
    fn rejects_a_duplicate_field() {
        let error = parse("title: A\ntitle: B").unwrap_err();
        assert!(matches!(error, ParseError::DuplicateHeader { span, .. } if span.line == 2));
    }

    #[test]
    fn rejects_a_header_after_a_node() {
        let error = parse("api [API]\nfooter: @johndoe").unwrap_err();
        assert_eq!(
            error.to_string(),
            "2:1: expected `footer:` before the first node or edge"
        );
    }
}

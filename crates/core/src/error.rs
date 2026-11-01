#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    // Global errors
    #[error(
        "{}:{}: expected a node (`id (Label)`, `id [Label]`) or an edge (`a -> b`)",
        span.line,
        span.column
    )]
    UnknownLine { span: Span },
    #[error("{}:{}: expected `{expected}` to close this", span.line, span.column)]
    UnclosedDelimiter { expected: char, span: Span },
    #[error("{}:{}: expected `*/` to close the comment", span.line, span.column)]
    UnclosedComment { span: Span },
    #[error("{}:{}: expected `# plotum 1`, found `{found}`", span.line, span.column)]
    InvalidVersion { found: String, span: Span },

    // Header errors
    #[error("{}:{}: expected a text after `{key}:`", span.line, span.column)]
    EmptyHeader { key: String, span: Span },
    #[error("{}:{}: expected only one `{key}:`, found a second one", span.line, span.column)]
    DuplicateHeader { key: String, span: Span },
    #[error(
        "{}:{}: expected `{key}:` before the first section, node or edge",
        span.line,
        span.column
    )]
    HeaderAfterBody { key: String, span: Span },

    // Section errors
    #[error(
        "{}:{}: expected a section type (`bad`, `ok`, `info`, `warn`, `plain`), found `{found}`",
        span.line,
        span.column
    )]
    UnknownSectionKind { found: String, span: Span },
    #[error(
        "{}:{}: expected a section as `section <type> \"Label\"`",
        span.line,
        span.column
    )]
    InvalidSection { span: Span },
    #[error("{}:{}: expected a text after `note:`", span.line, span.column)]
    EmptyNote { span: Span },
    #[error(
        "{}:{}: expected a whole number above 0 after `wrap:`, like `wrap: 4`",
        span.line,
        span.column
    )]
    InvalidWrap { span: Span },
    #[error(
        "{}:{}: expected only one `wrap:` per section, found a second one",
        span.line,
        span.column
    )]
    DuplicateWrap { span: Span },

    // Modifier errors
    #[error(
        "{}:{}: expected a modifier (`{{color}}`, `@col,row`, `icon=name`, `step=N`)",
        span.line,
        span.column
    )]
    UnknownModifier { span: Span },
    #[error("{}:{}: expected {expected} for `{modifier}`", span.line, span.column)]
    InvalidModifier {
        modifier: &'static str,
        expected: &'static str,
        span: Span,
    },
    #[error(
        "{}:{}: expected only one `{modifier}` modifier, found a second one",
        span.line,
        span.column
    )]
    DuplicateModifier { modifier: &'static str, span: Span },
    #[error(
        "{}:{}: expected `{modifier}` only on a node, found it on an edge",
        span.line,
        span.column
    )]
    ModifierOnEdge { modifier: &'static str, span: Span },
}

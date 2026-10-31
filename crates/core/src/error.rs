#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
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
}

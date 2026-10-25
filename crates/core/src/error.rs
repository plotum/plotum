#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("{}:{}: expected `a -> b`", span.line, span.column)]
    ExpectedEdge { span: Span },
    #[error("{}:{}: expected `*/` to close the comment", span.line, span.column)]
    UnclosedComment { span: Span },
}

use crate::error::Span;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Diagram {
    pub edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub span: Span,
}

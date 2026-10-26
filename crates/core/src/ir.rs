use crate::error::Span;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Diagram {
    pub edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub kind: ArrowKind,
    pub span: Span,
    pub label: Option<String>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stroke {
    Solid,  // -
    Dashed, // --
    Dotted, // ..
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,  // a -> b
    Backward, // a <- b
    Both,     // a <-> b
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowKind {
    Pointed {
        stroke: Stroke,
        direction: Direction,
    },
    Line,
}

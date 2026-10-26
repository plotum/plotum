use crate::error::Span;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Diagram {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub shape: Shape,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Shape {
    Circle,                          // id (Label)
    Box { subtext: Option<String> }, // id [Label | subtext]
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub kind: ArrowKind,
    pub span: Span,
    pub label: Option<String>,
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

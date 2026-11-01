use std::num::NonZeroU16;

use crate::error::Span;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Diagram {
    pub header: Header,
    pub sections: Vec<Section>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Header {
    pub title: Option<String>,    // title : lorem ipsum
    pub subtitle: Option<String>, // subtitle : lorem ipsum
    pub footer: Option<String>,   // footer: @johndoe
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub kind: SectionKind,     // Ok, Bad....
    pub label: Option<String>, // None for the "plain" section
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub notes: Vec<Note>,
    pub wrap: Option<NonZeroU16>, // wrap: 4, nodes per row in snake layout
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Note {
    pub text: String, // note: lorem ipsum
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub shape: Shape,
    pub color: Option<String>,   // {brand}, {#22C55E}
    pub icon: Option<String>,    // icon=database
    pub pinned: Option<GridPos>, // @2,0
    pub step: Option<u16>,       // step=2
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPos {
    pub col: u16,
    pub row: u16,
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
    pub color: Option<String>, // {red}
    pub step: Option<u16>,     // step=2
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SectionKind {
    Ok,
    Bad,
    Info,
    Warn,
    // Nodes written outside any `section` go to an implicit plain section
    #[default]
    Plain,
}

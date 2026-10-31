#![forbid(unsafe_code)]

mod error;
mod ir;
mod parse;

pub use error::{ParseError, Span};
pub use ir::{ArrowKind, Diagram, Direction, Edge, Header, Node, Shape, Stroke};
pub use parse::parse;

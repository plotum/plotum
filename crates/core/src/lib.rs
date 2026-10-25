#![forbid(unsafe_code)]

mod error;
mod ir;
mod parse;

pub use error::{ParseError, Span};
pub use ir::{Diagram, Edge};
pub use parse::parse;

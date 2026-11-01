use super::comment::Comments;
use super::edge::{edge, push_edge};
use super::header::set_header;
use super::lexer::lex_line;
use super::node::{box_node, circle_node, push_node};
use super::section::{add_note, open_section};
use super::token::Token;
use super::version::{check_version, is_version_line};
use crate::error::{ParseError, Span};
use crate::ir::Diagram;

pub fn parse(source: &str) -> Result<Diagram, ParseError> {
    let mut diagram = Diagram::default();

    let mut comments = Comments::default();

    // Read each line
    for (index, line) in source.lines().enumerate() {
        // The version line is optional, but only allowed on the first line
        if index == 0 && is_version_line(line) {
            check_version(line, index + 1)?;
            continue;
        }

        if comments.skip(line, index + 1) {
            continue;
        }

        let lexed = lex_line(line, index + 1)?;
        parse_line(lexed, &mut diagram)?;
    }

    comments.finish()?;

    Ok(diagram)
}

// Each arm is one accepted line shape
fn parse_line(lexed: Vec<(Token<'_>, Span)>, diagram: &mut Diagram) -> Result<(), ParseError> {
    use Token::{Arrow, Ident, Quoted, Round, Square, Text};

    // Tokens and their positions in two lists
    let (tokens, spans): (Vec<Token>, Vec<Span>) = lexed.into_iter().unzip();

    // skip empty list
    let Some(&line_span) = spans.first() else {
        return Ok(());
    };

    match tokens.as_slice() {
        // browser (Browser) step=1
        [Ident(id), Round(label), rest @ ..] => {
            let node = circle_node(id, label, line_span);
            push_node(diagram, node, rest, tail_spans(&spans, rest))?;
        }

        // title: Checkout
        [Ident(key @ ("title" | "subtitle" | "footer")), Text(text)] => {
            set_header(diagram, key, text, line_span)?;
        }

        // note: every server reads the same topic
        [Ident("note"), Text(text)] => add_note(diagram, text, line_span)?,

        // section ok "With cache"
        [Ident("section"), Ident(kind), Quoted(label)] => {
            open_section(diagram, kind, label, line_span)?;
        }

        // s1 [Serveur 1 | WebSocket] {brand}
        [Ident(id), Square(body), rest @ ..] => {
            let node = box_node(id, body, line_span);
            push_node(diagram, node, rest, tail_spans(&spans, rest))?;
        }

        // a -> b {red} step=2 : label
        [Ident(from), Arrow(kind), Ident(to), rest @ ..] => {
            let edge = edge(from, *kind, to, line_span);
            push_edge(diagram, edge, rest, tail_spans(&spans, rest))?;
        }

        // `section ok` without label: after the node and edge arms, so `section -> b` stays an edge
        [Ident("section"), ..] => return Err(ParseError::InvalidSection { span: line_span }),

        // Any other line shape
        _ => return Err(ParseError::UnknownLine { span: line_span }),
    }

    Ok(())
}

// The spans of `rest`, the last tokens of the line
fn tail_spans<'a>(spans: &'a [Span], rest: &[Token<'_>]) -> &'a [Span] {
    spans.get(spans.len() - rest.len()..).unwrap_or_default()
}

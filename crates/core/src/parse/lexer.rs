use std::iter::Peekable;
use std::str::CharIndices;

use super::token::{Token, get_word};
use crate::error::{ParseError, Span};

type Chars<'a> = Peekable<CharIndices<'a>>;

pub(crate) fn lex_line<'a>(
    line: &'a str,
    line_number: usize,
) -> Result<Vec<(Token<'a>, Span)>, ParseError> {
    let mut tokens = Vec::new();
    let mut chars = line.char_indices().peekable();

    while let Some(&(start, character)) = chars.peek() {
        // ' ' char is skipped
        if character.is_whitespace() {
            chars.next();
            continue;
        }

        // : lorem ipsum is the last text on the line
        if starts_text(line, start) {
            let text = line[start + 1..].trim();
            let span = make_span(line, start, line.len(), line_number);
            tokens.push((Token::Text(text), span));
            break;
        }

        // get the token
        let token = match closing_delimiter(character) {
            // Some(close) contains delimiter closing char : ) }....
            Some(close) => match read_delimited(&mut chars, line, close) {
                Some(token) => token,
                None => {
                    // No delimiter at the end ? Error :(
                    let span = make_span(line, start, start + 1, line_number);
                    return Err(ParseError::UnclosedDelimiter {
                        expected: close,
                        span,
                    });
                }
            },
            // only classic word
            None => get_word(read_word(&mut chars, line, start)),
        };

        let end = current_position(&mut chars, line);
        tokens.push((token, make_span(line, start, end, line_number)));
    }

    Ok(tokens)
}

// `: ` opens a text, but `https://` or `a:b` stay inside a word
fn starts_text(line: &str, index: usize) -> bool {
    let rest = &line[index..];
    if !rest.starts_with(':') {
        return false;
    }

    let after_colon = &rest[1..];
    after_colon.is_empty() || after_colon.starts_with([' ', '\t'])
}

fn closing_delimiter(open: char) -> Option<char> {
    match open {
        '(' => Some(')'),
        '[' => Some(']'),
        '"' => Some('"'),
        '{' => Some('}'),
        // Any other char does not open a delimited text
        _ => None,
    }
}

fn delimited_token(open: char, inside: &str) -> Token<'_> {
    match open {
        '(' => Token::Round(inside),
        '[' => Token::Square(inside),
        '"' => Token::Quoted(inside),
        // Only `{` is left, see `closing_delimiter`
        _ => Token::Style(inside),
    }
}

// `[Serveur 1 | WS]` gives Token::Square("Serveur 1 | WS"), or None if `]` is missing
fn read_delimited<'a>(chars: &mut Chars<'a>, line: &'a str, close: char) -> Option<Token<'a>> {
    let (open_index, open) = chars.next()?;
    let inside_start = open_index + open.len_utf8();

    // get all the string in the delimitation
    for (index, character) in chars.by_ref() {
        if character == close {
            let inside = &line[inside_start..index];
            return Some(delimited_token(open, inside));
        }
    }

    None
}

fn read_word<'a>(chars: &mut Chars<'a>, line: &'a str, start: usize) -> &'a str {
    while let Some(&(index, character)) = chars.peek() {
        if ends_word(line, index, character) {
            return &line[start..index];
        }
        chars.next();
    }

    &line[start..]
}

// `alice(Alice)` : the word stops before `(`
fn ends_word(line: &str, index: usize, character: char) -> bool {
    character.is_whitespace() || closing_delimiter(character).is_some() || starts_text(line, index)
}

fn current_position(chars: &mut Chars<'_>, line: &str) -> usize {
    match chars.peek() {
        Some((index, _character)) => *index,
        None => line.len(),
    }
}

fn make_span(line: &str, start: usize, end: usize, line_number: usize) -> Span {
    Span {
        line: line_number,
        // In VsCode for example, we start with column 1.
        column: line[..start].chars().count() + 1,
        length: line[start..end].chars().count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{ArrowKind, Direction, Stroke};

    fn tokens(line: &str) -> Vec<Token<'_>> {
        let result = lex_line(line, 1).unwrap();
        result.into_iter().map(|(token, _span)| token).collect()
    }

    #[test]
    fn lexes_an_edge() {
        // a give Token::Ident("a")
        // -> give Token::Arrow(Solid, Forward)
        // b give Token::Ident("b")
        assert_eq!(
            tokens("a -> b"),
            [
                Token::Ident("a"),
                Token::Arrow(ArrowKind::Pointed {
                    stroke: Stroke::Solid,
                    direction: Direction::Forward
                }),
                Token::Ident("b")
            ]
        );
    }

    #[test]
    fn lexes_an_edge_reverse() {
        assert_eq!(
            tokens("a <- b"),
            [
                Token::Ident("a"),
                Token::Arrow(ArrowKind::Pointed {
                    stroke: Stroke::Solid,
                    direction: Direction::Backward
                }),
                Token::Ident("b")
            ]
        );
    }

    #[test]
    fn keeps_spaces_inside_delimiters() {
        assert_eq!(
            tokens("s1 [Serveur 1 | WebSocket]"),
            [Token::Ident("s1"), Token::Square("Serveur 1 | WebSocket")]
        );
        assert_eq!(
            tokens("alice(Alice B)"),
            [Token::Ident("alice"), Token::Round("Alice B")]
        );
    }

    #[test]
    fn lexes_quoted_and_style() {
        assert_eq!(
            tokens(r#"section bad "Sans broker" {red}"#),
            [
                Token::Ident("section"),
                Token::Ident("bad"),
                Token::Quoted("Sans broker"),
                Token::Style("red")
            ]
        );
    }

    #[test]
    fn reads_the_rest_of_the_line_as_text() {
        assert_eq!(
            tokens("title: Chat : v2"),
            [Token::Ident("title"), Token::Text("Chat : v2")]
        );
    }

    #[test]
    fn keeps_colons_inside_words() {
        assert_eq!(
            tokens("https://example.com"),
            [Token::Ident("https://example.com")]
        );
    }

    #[test]
    fn counts_columns_in_chars() {
        let result = lex_line("élève (Élève)", 1).unwrap();
        assert_eq!(
            result[1].1,
            Span {
                line: 1,
                column: 7,
                length: 7
            }
        );
    }

    #[test]
    fn rejects_an_unclosed_delimiter() {
        let error = lex_line("api [API", 3).unwrap_err();
        assert_eq!(
            error,
            ParseError::UnclosedDelimiter {
                expected: ']',
                span: Span {
                    line: 3,
                    column: 5,
                    length: 1
                }
            }
        );
    }
}

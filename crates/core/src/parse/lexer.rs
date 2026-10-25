use crate::error::Span;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Ident(&'a str),
    // Arrow will begin another enum at the end (I hope)
    Arrow,
}

fn get_word(word: &str) -> Token<'_> {
    match word {
        "->" => Token::Arrow,
        _ => Token::Ident(word),
    }
}

pub(crate) fn lex_line(line: &str, line_number: usize) -> Vec<(Token<'_>, Span)> {
    // Empty list (type defined with the first push)
    let mut tokens = Vec::new();

    // In VsCode for example, we start with column 1.
    let mut column = 1;

    // Every word is separated by spaces : a -> b => "a", "->" and "b"
    // Empty Word can exist at this step : a  -> b => "a", "", "->" and "b"
    for word in line.split(' ') {
        // .count() give char number, len() will give bytes number if used here
        let length = word.chars().count();

        // Empty char skipped here
        if length > 0 {
            let token = get_word(word);

            // Span builded with position, length and line where is the word
            let span = Span {
                line: line_number,
                column,
                length,
            };

            tokens.push((token, span));
        }

        column += length + 1;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_an_edge() {
        let result = lex_line("a -> b", 1);

        let mut tokens = Vec::new();

        for (token, _position) in result {
            tokens.push(token);
        }

        // a give Token::Indent("a")
        // -> give Token::Arrow
        // b give Token::Indent("b")
        assert_eq!(tokens, [Token::Ident("a"), Token::Arrow, Token::Ident("b")]);
    }
}

use crate::ir::{ArrowKind, Direction, Stroke};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Ident(&'a str),
    // Arrow can have some different style
    Arrow(ArrowKind),
}

pub(crate) fn get_word(word: &str) -> Token<'_> {
    use Direction::{Backward, Both, Forward};
    use Stroke::{Dashed, Dotted, Solid};

    match word {
        // Arrows
        "->" => pointed(Solid, Forward),
        "<-" => pointed(Solid, Backward),
        "<->" => pointed(Solid, Both),
        "-->" => pointed(Dashed, Forward),
        "<--" => pointed(Dashed, Backward),
        "<-->" => pointed(Dashed, Both),
        "..>" => pointed(Dotted, Forward),
        "<.." => pointed(Dotted, Backward),
        "<..>" => pointed(Dotted, Both),
        "---" => Token::Arrow(ArrowKind::Line),

        // Other
        _ => Token::Ident(word),
    }
}

fn pointed(stroke: Stroke, direction: Direction) -> Token<'static> {
    Token::Arrow(ArrowKind::Pointed { stroke, direction })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_arrow_stroke_and_direction() {
        assert_eq!(
            get_word("<--"),
            pointed(Stroke::Dashed, Direction::Backward)
        );
        assert_eq!(get_word("<..>"), pointed(Stroke::Dotted, Direction::Both));
    }

    #[test]
    fn reads_line_without_direction() {
        assert_eq!(get_word("---"), Token::Arrow(ArrowKind::Line));
    }

    #[test]
    fn reads_other_words_as_identifiers() {
        assert_eq!(get_word("api"), Token::Ident("api"));
    }
}

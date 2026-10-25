#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ArrowKind {
    Classic,       // ->
    Dashed,        // -->
    Dotted,        // ..>
    Line,          // ---
    BiDirectional, // <->
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Ident(&'a str),
    // Arrow can have some different style
    Arrow(ArrowKind),
}

pub(crate) fn get_word(word: &str) -> Token<'_> {
    match word {
        // Arrows
        "->" => Token::Arrow(ArrowKind::Classic),
        "-->" => Token::Arrow(ArrowKind::Dashed),
        "..>" => Token::Arrow(ArrowKind::Dotted),
        "---" => Token::Arrow(ArrowKind::Line),
        "<->" => Token::Arrow(ArrowKind::BiDirectional),

        // Other
        _ => Token::Ident(word),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_every_arrow() {
        assert_eq!(get_word("-->"), Token::Arrow(ArrowKind::Dashed));
        assert_eq!(get_word("<->"), Token::Arrow(ArrowKind::BiDirectional));
    }

    #[test]
    fn reads_other_words_as_identifiers() {
        assert_eq!(get_word("api"), Token::Ident("api"));
    }
}

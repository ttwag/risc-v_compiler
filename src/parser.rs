use crate::token::{SyntaxToken, Token};

enum ParseError {
    UnexpectedToken,
    UnexpectedEof,
}

struct Parser<'a> {
    index: usize,
    sts: &'a [SyntaxToken],
}

impl<'a> Parser<'a> {
    /// Creates a new parser over a syntax token slice.
    ///
    /// # Panics
    /// Panics if `sts` is empty or does not end with `Token::Eof`.
    pub fn new(sts: &'a [SyntaxToken]) -> Self {
        assert!(
            matches!(sts.last(), Some(t) if t.token == Token::Eof),
            "syntax token slice must end with EOF"
        );
        Self { index: 0, sts }
    }
}

use crate::token::SyntaxToken;

enum ParseError {
    UnexpectedToken,
    UnexpectedEof
}

struct Parser<'a> {
    index: usize,
    sts: &'a [SyntaxToken],
}

impl<'a> Parser<'a> {
    pub fn new(sts: &'a [SyntaxToken]) -> Self {
        Self { index: 0, sts }
    }
}

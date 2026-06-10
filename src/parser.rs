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

    fn peek(&self) -> &SyntaxToken {
        self.sts
            .get(self.index)
            .expect("peek: parser index out of bounds")
    }

#[cfg(test)]
mod tests {
    use crate::token::Span;

    use super::*;

    // ── peek ──────────────────────────────────────────────────────────────────
    #[test]
    fn peek_return_correct_token() {
        let sts = [SyntaxToken {
            token: Token::Eof,
            span: Span::default(),
        }];
        let p = Parser::new(&sts);

        assert_eq!(p.peek().token, Token::Eof);
    }

    #[test]
    #[should_panic(expected = "peek: parser index out of bounds")]
    fn peek_crash_with_index_out_of_bound() {
        let sts = [SyntaxToken {
            token: Token::Eof,
            span: Span::default(),
        }];
        let mut p = Parser::new(&sts);
        p.index += 1;
        p.peek();
    }
}

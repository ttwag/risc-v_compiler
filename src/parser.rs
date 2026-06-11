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

    fn advance(&mut self) -> &SyntaxToken {
        let curr = self
            .sts
            .get(self.index)
            .expect("advance: parser index out of bounds");
        match curr.token {
            Token::Eof => {}
            _ => {
                self.index += 1;
            }
        }
        curr
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

    // ── advance ──────────────────────────────────────────────────────────────────
    #[test]
    fn advance_pass_token() {
        let sts = [
            SyntaxToken {
                token: Token::RCurly,
                span: Span::default(),
            },
            SyntaxToken {
                token: Token::Eof,
                span: Span::default(),
            },
        ];
        let mut p = Parser::new(&sts);
        assert_eq!(p.advance().token, Token::RCurly);
        assert_eq!(p.index, 1);
    }

    #[test]
    fn advance_not_pass_eof() {
        let sts = [SyntaxToken {
            token: Token::Eof,
            span: Span::default(),
        }];
        let mut p = Parser::new(&sts);
        assert_eq!(p.advance().token, Token::Eof);
        assert_eq!(p.index, 0);
    }
}

use crate::token::{SyntaxToken, Token};

#[derive(Debug, PartialEq)]
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

    fn expect(&mut self, token: Token) -> Result<&SyntaxToken, ParseError> {
        assert!(token != Token::Eof, "expect: cannot expect Eof");
        let curr = self
            .sts
            .get(self.index)
            .expect("expect: parser index out of bounds");
        if curr.token == token {
            self.index += 1;
            Ok(curr)
        } else {
            match curr.token {
                Token::Eof => Err(ParseError::UnexpectedEof),
                _ => Err(ParseError::UnexpectedToken),
            }
        }
    }
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

    // ── expect ──────────────────────────────────────────────────────────────────
    #[test]
    #[should_panic(expected = "expect: cannot expect Eof")]
    fn expect_eof() {
        let sts = [SyntaxToken {
            token: Token::Eof,
            span: Span::default(),
        }];
        let mut p = Parser::new(&sts);
        let _ = p.expect(Token::Eof);
    }

    #[test]
    fn expect_token_return_token_and_advance() {
        let sts = [
            SyntaxToken {
                token: Token::Arrow,
                span: Span::default(),
            },
            SyntaxToken {
                token: Token::Eof,
                span: Span::default(),
            },
        ];
        let mut p = Parser::new(&sts);
        let result = p.expect(Token::Arrow).unwrap();
        assert_eq!(*result, sts[0]);
        assert_eq!(p.index, 1)
    }

    #[test]
    fn expect_mismatch_token_return_unexpected_token() {
        let sts = [
            SyntaxToken {
                token: Token::Arrow,
                span: Span::default(),
            },
            SyntaxToken {
                token: Token::Eof,
                span: Span::default(),
            },
        ];
        let mut p = Parser::new(&sts);
        let result = p.expect(Token::LParen).unwrap_err();
        assert_eq!(result, ParseError::UnexpectedToken);
        assert_eq!(p.index, 0);
    }

    #[test]
    fn expect_mismatch_token_return_unexpected_eof() {
        let sts = [SyntaxToken {
            token: Token::Eof,
            span: Span::default(),
        }];
        let mut p = Parser::new(&sts);
        let result = p.expect(Token::LParen).unwrap_err();
        assert_eq!(result, ParseError::UnexpectedEof);
        assert_eq!(p.index, 0);
    }
}

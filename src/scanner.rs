use crate::token::{Location, Span, SyntaxToken, Token};

#[derive(Debug)]
pub enum ScanError {
    UnexpectedChar(char, Location),
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScanError::UnexpectedChar(c, loc) => {
                write!(f, "Unexpected Character\nCharacter: {}\n{}", c, loc)
            }
        }
    }
}

impl std::error::Error for ScanError {}

pub struct Scanner<'a> {
    input: &'a str,
    loc: Location,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            loc: Location::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.loc.index..].chars().nth(0)
    }

    fn peek_next(&self) -> Option<char> {
        self.input[self.loc.index..].chars().nth(1)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        if let Some(ch) = c {
            self.loc.index += ch.len_utf8();
            match ch {
                '\n' => {
                    self.loc.line += 1;
                    self.loc.col = 1;
                }
                _ => {
                    self.loc.col += 1;
                }
            }
        };
        c
    }

    fn capture_st(&self, kind: Token, start: Location) -> SyntaxToken {
        assert!(
            start.index < self.loc.index,
            "capture_st: starting index must be less than current index"
        );
        assert!(
            self.loc.index <= self.input.len(),
            "capture_st: index out of bound"
        );

        SyntaxToken {
            token: kind,
            span: Span {
                start,
                end: self.loc,
            },
        }
    }

    fn capture_string(&self, start: Location) -> String {
        assert!(
            start.index < self.loc.index,
            "capture_string: starting index must be less than current index"
        );
        assert!(
            self.loc.index <= self.input.len(),
            "capture_string: index out of bound"
        );

        String::from(self.input.get(start.index..self.loc.index).unwrap())
    }

    ///
    /// From current input index, scans for a number matching the pattern 0 | [1-9][0-9]*
    /// Advanced past all consumed digits.
    /// Returns an error when no digit or zero leading other digits
    /// # Panics
    /// Panics if called when the scanner is not positioned at a leading digit
    ///
    fn advance_number(&mut self) -> Result<(), ScanError> {
        match self.peek() {
            Some('0') => {
                self.advance();
                if let Some(ch @ '0'..='9') = self.peek() {
                    return Err(ScanError::UnexpectedChar(ch, self.loc));
                }
                Ok(())
            }
            Some('1'..='9') => {
                while let Some(next) = self.peek() {
                    if !matches!(next, '0'..='9') {
                        break;
                    }
                    self.advance();
                }
                Ok(())
            }
            _ => unreachable!("advance_number called without a leading digit"),
        }
    }

    ///
    /// From current input index, scans for a number matching the pattern [a-zA-Z_][a-zA-Z_0-9]*
    /// Advanced past all consumed characters.
    /// Returns an error when seeing an invalid character
    /// # Panics
    /// Panics if called when the scanner is not positioned at a valid character in [a-zA-Z_]
    ///
    fn advance_id(&mut self) -> Result<(), ScanError> {
        match self.peek() {
            Some('a'..='z' | 'A'..='Z' | '_') => {
                while let Some(ch) = self.peek() {
                    if !(ch.is_ascii_alphanumeric() || ch == '_') {
                        break;
                    }
                    self.advance();
                }
            }
            _ => unreachable!("advance_id called without a valid leading character"),
        }
        Ok(())
    }

    fn apply_keyword(&self, sts: &mut Vec<SyntaxToken>) {
        for st in sts {
            st.token = match self.read_str(&st.span) {
                Some("int") => Token::Int,
                Some("let") => Token::Let,
                Some("while") => Token::While,
                Some("if") => Token::If,
                Some("elif") => Token::ElseIf,
                Some("else") => Token::Else,
                Some("return") => Token::Return,
                Some("fn") => Token::Function,
                _ => continue,
            };
        }
    }

    fn read_str(&self, span: &Span) -> Option<&'a str> {
        self.input.get(span.start.index..span.end.index)
    }

    /// Scans the input and returns a list of tokens as defined in grammar.ebnf.
    ///
    /// # Examples
    /// ```
    /// use risc_v_compiler::scanner;
    /// let mut scanner = scanner::Scanner::new("let x := 42;");
    /// let sts = scanner.scan();
    /// ```
    #[rustfmt::skip]
    pub fn scan(&mut self) -> Result<Vec<SyntaxToken>, ScanError> {
        let mut sts = Vec::new();
        while let Some(curr) = self.peek() {
            let next = self.peek_next();
            let start = self.loc;
            match (curr, next) {
                (' ' | '\t' | '\n' | '\r', _)    => {self.advance();}
                (':', Some('='))                 => sts.push({self.advance(); self.advance(); self.capture_st(Token::Assignment, start)}),
                ('=', Some('='))                 => sts.push({self.advance(); self.advance(); self.capture_st(Token::Equality, start)}),
                ('-', Some('>'))                 => sts.push({self.advance(); self.advance(); self.capture_st(Token::Arrow, start)}),
                ('(', _)                         => sts.push({self.advance(); self.capture_st(Token::LParen, start)}),
                (')', _)                         => sts.push({self.advance(); self.capture_st(Token::RParen, start)}),
                ('{', _)                         => sts.push({self.advance(); self.capture_st(Token::LCurly, start)}),
                ('}', _)                         => sts.push({self.advance(); self.capture_st(Token::RCurly, start)}),
                (';', _)                         => sts.push({self.advance(); self.capture_st(Token::Semi, start)}),
                (':', _)                         => sts.push({self.advance(); self.capture_st(Token::Colon, start)}),
                (',', _)                         => sts.push({self.advance(); self.capture_st(Token::Comma, start)}),
                ('+', _)                         => sts.push({self.advance(); self.capture_st(Token::Plus, start)}),
                ('-', _)                         => sts.push({self.advance(); self.capture_st(Token::Minus, start)}),
                ('>', _)                         => sts.push({self.advance(); self.capture_st(Token::Grt, start)}),
                ('0'..='9', _)                   => sts.push({self.advance_number()?; self.capture_st(Token::Num(self.capture_string(start)), start)}),
                ('a'..='z' | 'A'..='Z' | '_', _) => sts.push({self.advance_id()?; self.capture_st(Token::Id(self.capture_string(start)), start)}),
                (ch, _) => return Err(ScanError::UnexpectedChar(ch, self.loc)),
            }
        }
        self.apply_keyword(&mut sts);
        sts.push(SyntaxToken {token: Token::Eof, span: Span {start: self.loc, end: self.loc,}});
        Ok(sts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── peek ──────────────────────────────────────────────────────────────────
    #[test]
    fn peek_on_empty_input_returns_none() {
        let s = Scanner::new("");
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn peek_returns_ch() {
        let s = Scanner::new("aint");
        assert_eq!(s.peek(), Some('a'));
    }

    #[test]
    fn peek_same_index_after_two_calls() {
        let s = Scanner::new("t");
        s.peek();
        s.peek();
        assert_eq!(s.loc.index, 0);
    }

    #[test]
    fn peek_handles_unicode() {
        let s = Scanner::new("£");
        s.peek();
        s.peek();
        assert_eq!(s.loc.index, 0);
    }

    // ── advance ───────────────────────────────────────────────────────────────
    #[test]
    fn advance_empty_returns_none() {
        let mut s = Scanner::new("");
        assert_eq!(s.advance(), None);
    }

    #[test]
    fn advance_ends() {
        let mut s = Scanner::new("再見");
        s.advance();
        s.advance();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn advance_returns_current_char_and_move_index() {
        let mut s = Scanner::new("abc");
        assert_eq!(s.advance(), Some('a'));
        assert_eq!(s.loc.index, 1);
    }

    #[test]
    fn advance_new_line_increase_line_and_resets_col() {
        let mut s = Scanner::new("\n");
        s.advance();
        assert_eq!(
            s.loc,
            Location {
                index: 1,
                line: 2,
                col: 1
            }
        );
    }

    // ── advance_number ───────────────────────────────────────────────────────────────
    #[test]
    fn advance_number_matches_number() -> Result<(), ScanError> {
        let mut s = Scanner::new("12345");
        let _ = s.advance_number()?;
        assert_eq!(s.loc.index, 5);
        Ok(())
    }

    #[test]
    fn advance_number_ignores_zero_as_head() {
        let mut s = Scanner::new("09");
        let err = s.advance_number().unwrap_err();
        assert!(matches!(err, ScanError::UnexpectedChar('9', ..)));
    }

    #[test]
    #[should_panic]
    fn advance_number_no_digit() {
        let mut s = Scanner::new("e");
        let _ = s.advance_number();
    }

    #[test]
    fn advance_number_advance_zero() -> Result<(), ScanError> {
        let mut s = Scanner::new("0");
        let _ = s.advance_number()?;
        assert_eq!(s.loc.index, 1);
        Ok(())
    }

    // ── advance_id ───────────────────────────────────────────────────────────────
    #[test]
    #[should_panic]
    fn advance_id_no_invalid_char() {
        let mut s = Scanner::new("!");
        let _ = s.advance_id();
    }

    #[test]
    fn advance_id_take_id() -> Result<(), ScanError> {
        let mut s = Scanner::new("this_is_an_id");
        let _ = s.advance_id()?;
        assert_eq!(s.loc.index, 13);
        Ok(())
    }

    #[test]
    fn advance_id_take_id_containing_keyword() -> Result<(), ScanError> {
        let mut s = Scanner::new("this_is_an_int_id");
        let _ = s.advance_id()?;
        assert_eq!(s.loc.index, 17);
        Ok(())
    }

    #[test]
    fn advance_id_match_keyword() -> Result<(), ScanError> {
        let mut s = Scanner::new("return");
        let _ = s.advance_id()?;
        assert_eq!(s.loc.index, 6);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn advance_id_reject_leading_digit() {
        let mut s = Scanner::new("0_hi");
        let _ = s.advance_id();
    }
}

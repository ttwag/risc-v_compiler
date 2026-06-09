use crate::token::{Location, Span, SyntaxToken, TokenType};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ScanError {
    UnexpectedChar(Option<char>, Location), //character, line, col
}
impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScanError::UnexpectedChar(Some(c), loc) => {
                write!(
                    f,
                    "Unexpected character: {} at line {} col {}",
                    c, loc.line, loc.col
                )
            }
            ScanError::UnexpectedChar(None, loc) => {
                write!(
                    f,
                    "Unexpected end of input at line {} col {}",
                    loc.line, loc.col
                )
            }
        }
    }
}
impl Error for ScanError {}

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

    fn capture_st(&self, kind: TokenType, start: Location) -> SyntaxToken {
        assert!(
            start.index < self.loc.index,
            "capture_st: starting index must be less than current index"
        );
        assert!(
            self.loc.index <= self.input.len(),
            "capture_st: index out of bound"
        );

        SyntaxToken {
            kind,
            span: Span {
                start,
                end: self.loc,
            },
        }
    }

    ///
    /// From current input index, scans for a number matching the pattern 0 | [1-9][0-9]*
    /// Advanced past all consumed digits.
    /// Returns an error when no digit or zero leading other digits
    /// # Panics (debug)
    /// Panics in debug builds if `self.index` is out of bounds.
    ///
    fn advance_number(&mut self) -> Result<(), ScanError> {
        debug_assert!(
            self.loc.index < self.input.len(),
            "emit_number: index out of bounds"
        );

        // advance
        match self.peek() {
            Some('0') => {
                self.advance();
                if matches!(self.peek(), Some('0'..='9')) {
                    return Err(ScanError::UnexpectedChar(self.peek(), self.loc));
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
            _ => Err(ScanError::UnexpectedChar(self.peek(), self.loc)),
        }
    }

    ///
    /// From current input index, scans for a number matching the pattern [a-zA-Z_][a-zA-Z_0-9]*
    /// Advanced past all consumed characters.
    /// Returns an error when seeing an invalid character
    /// # Panics (debug)
    /// Panics in debug builds if `self.index` is out of bounds.
    ///
    fn advance_id(&mut self) -> Result<(), ScanError> {
        debug_assert!(
            self.loc.index < self.input.len(),
            "emit_number: index out of bounds"
        );

        //advance
        match self.peek() {
            Some('a'..='z' | 'A'..='Z' | '_') => {
                while let Some(ch) = self.peek() {
                    if !(ch.is_ascii_alphanumeric() || ch == '_') {
                        break;
                    }
                    self.advance();
                }
            }
            _ => {
                return Err(ScanError::UnexpectedChar(self.peek(), self.loc));
            }
        }
        Ok(())
    }

    fn apply_keyword(&self, sts: &mut Vec<SyntaxToken>) {
        for st in sts {
            st.kind = match st.get_str(self.input) {
                Some("int") => TokenType::Int,
                Some("let") => TokenType::Let,
                Some("while") => TokenType::While,
                Some("if") => TokenType::If,
                Some("elif") => TokenType::ElseIf,
                Some("else") => TokenType::Else,
                Some("return") => TokenType::Return,
                Some("fn") => TokenType::Function,
                _ => continue,
            }
        }
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
                (':', Some('='))                 => sts.push({self.advance(); self.advance(); self.capture_st(TokenType::Assignment, start)}),
                ('=', Some('='))                 => sts.push({self.advance(); self.advance(); self.capture_st(TokenType::Equality, start)}),
                ('-', Some('>'))                 => sts.push({self.advance(); self.advance(); self.capture_st(TokenType::Arrow, start)}),
                ('(', _)                         => sts.push({self.advance(); self.capture_st(TokenType::LParen, start)}),
                (')', _)                         => sts.push({self.advance(); self.capture_st(TokenType::RParen, start)}),
                ('{', _)                         => sts.push({self.advance(); self.capture_st(TokenType::LCurly, start)}),
                ('}', _)                         => sts.push({self.advance(); self.capture_st(TokenType::RCurly, start)}),
                (';', _)                         => sts.push({self.advance(); self.capture_st(TokenType::Semi, start)}),
                (':', _)                         => sts.push({self.advance(); self.capture_st(TokenType::Colon, start)}),
                (',', _)                         => sts.push({self.advance(); self.capture_st(TokenType::Comma, start)}),
                ('+', _)                         => sts.push({self.advance(); self.capture_st(TokenType::Plus, start)}),
                ('-', _)                         => sts.push({self.advance(); self.capture_st(TokenType::Minus, start)}),
                ('>', _)                         => sts.push({self.advance(); self.capture_st(TokenType::Grt, start)}),
                ('0'..='9', _)                   => sts.push({self.advance_number()?; self.capture_st(TokenType::Num, start)}),
                ('a'..='z' | 'A'..='Z' | '_', _) => sts.push({self.advance_id()?; self.capture_st(TokenType::Id, start)}),
                (_, _) => return Err(ScanError::UnexpectedChar(self.peek(), self.loc)),
            }
        }
        self.apply_keyword(&mut sts);
        sts.push(SyntaxToken {kind: TokenType::Eof, span: Span {start: self.loc, end: self.loc,}});
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

    // ── emit_number ───────────────────────────────────────────────────────────────
    #[test]
    fn emit_number_matches_number() -> Result<(), ScanError> {
        let mut s = Scanner::new("12345");
        let _ = s.advance_number()?;
        assert_eq!(s.loc.index, 5);
        Ok(())
    }

    #[test]
    fn emit_number_ignores_zero_as_head() {
        let mut s = Scanner::new("09");
        let err = s.advance_number().unwrap_err();
        assert!(matches!(err, ScanError::UnexpectedChar(Some('9'), ..)));
    }

    #[test]
    fn emit_number_no_digit() {
        let mut s = Scanner::new("e");
        let err = s.advance_number().unwrap_err();
        assert!(matches!(err, ScanError::UnexpectedChar(Some('e'), ..)));
    }

    #[test]
    fn emit_number_emits_zero() -> Result<(), ScanError> {
        let mut s = Scanner::new("0");
        let _ = s.advance_number()?;
        assert_eq!(s.loc.index, 1);
        Ok(())
    }

    // ── emit_id ───────────────────────────────────────────────────────────────
    #[test]
    fn emit_id_no_invalid_char() {
        let mut s = Scanner::new("!");
        let err = s.advance_id().unwrap_err();
        assert!(matches!(err, ScanError::UnexpectedChar(Some('!'), ..)));
    }

    #[test]
    fn emit_id_take_id() -> Result<(), ScanError> {
        let mut s = Scanner::new("this_is_an_id");
        let _ = s.advance_id()?;
        assert_eq!(s.loc.index, 13);
        Ok(())
    }

    #[test]
    fn emit_id_take_id_containing_keyword() -> Result<(), ScanError> {
        let mut s = Scanner::new("this_is_an_int_id");
        let _ = s.advance_id()?;
        assert_eq!(s.loc.index, 17);
        Ok(())
    }

    #[test]
    fn emit_id_match_keyword() -> Result<(), ScanError> {
        let mut s = Scanner::new("return");
        let _ = s.advance_id()?;
        assert_eq!(s.loc.index, 6);
        Ok(())
    }

    #[test]
    fn emit_id_reject_leading_digit() {
        let mut s = Scanner::new("0_hi");
        let err = s.advance_id().unwrap_err();
        assert!(matches!(err, ScanError::UnexpectedChar(Some('0'), ..)));
    }
}

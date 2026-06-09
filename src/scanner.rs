use crate::token::{Location, Span, SyntaxToken, TokenType};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar(Option<char>, Location), //character, line, col
}
impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::UnexpectedChar(Some(c), loc) => {
                write!(
                    f,
                    "Unexpected character: {} at line {} col {}",
                    c, loc.line, loc.col
                )
            }
            LexError::UnexpectedChar(None, loc) => {
                write!(
                    f,
                    "Unexpected end of input at line {} col {}",
                    loc.line, loc.col
                )
            }
        }
    }
}
impl Error for LexError {}

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

    pub fn peek(&self) -> Option<char> {
        self.input[self.loc.index..].chars().nth(0)
    }

    pub fn peek_next(&self) -> Option<char> {
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

    fn make_token_from(&self, kind: TokenType, start: Location) -> SyntaxToken<'a> {
        SyntaxToken {
            kind,
            value: self.input.get(start.index..self.loc.index),
            span: Span {
                start,
                end: self.loc,
            },
        }
    }

    ///
    /// From current input index, scans for a number matching the pattern 0 | [1-9][0-9]*
    /// and return the tokenized result. Advanced past all consumed digits.
    /// Returns an error when no digit or zero leading other digits
    /// # Panics (debug)
    /// Panics in debug builds if `self.index` is out of bounds.
    ///
    fn emit_number(&mut self) -> Result<SyntaxToken<'a>, LexError> {
        debug_assert!(
            self.loc.index < self.input.len(),
            "emit_number: index out of bounds"
        );

        let start = self.loc;

        // advance
        match self.peek() {
            Some('0') => {
                self.advance();
                if matches!(self.peek(), Some('0'..='9')) {
                    return Err(LexError::UnexpectedChar(self.peek(), self.loc));
                }
                Ok(self.make_token_from(TokenType::Num, start))
            }
            Some('1'..='9') => {
                self.advance();
                while let Some(next) = self.peek() {
                    if !matches!(next, '0'..='9') {
                        break;
                    }
                    self.advance();
                }
                Ok(self.make_token_from(TokenType::Num, start))
            }
            _ => Err(LexError::UnexpectedChar(self.peek(), self.loc)),
        }
    }

    fn match_keyword(s: Option<&str>) -> Option<TokenType> {
        match s {
            Some("int") => Some(TokenType::Int),
            Some("let") => Some(TokenType::Let),
            Some("while") => Some(TokenType::While),
            Some("if") => Some(TokenType::If),
            Some("elif") => Some(TokenType::ElseIf),
            Some("else") => Some(TokenType::Else),
            Some("return") => Some(TokenType::Return),
            Some("fn") => Some(TokenType::Function),
            _ => None,
        }
    }

    ///
    /// From current input index, scans for a number matching the pattern [a-zA-Z_][a-zA-Z_0-9]*
    /// and return the tokenized result (will replace with keyword if needed).
    /// Advanced past all consumed characters.
    /// Returns an error when seeing an invalid character
    /// # Panics (debug)
    /// Panics in debug builds if `self.index` is out of bounds.
    ///
    fn emit_id(&mut self) -> Result<SyntaxToken<'a>, LexError> {
        debug_assert!(
            self.loc.index < self.input.len(),
            "emit_number: index out of bounds"
        );

        let start = self.loc;

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
                return Err(LexError::UnexpectedChar(self.peek(), self.loc));
            }
        }

        // make token from the current index
        let mut token = self.make_token_from(TokenType::Id, start);
        if let Some(kind) = Scanner::match_keyword(token.value) {
            token.kind = kind;
        }
        Ok(token)
    }

    /// Scans the input and returns a list of tokens as defined in grammar.ebnf.
    ///
    /// # Panics
    /// Panics on unexpected characters.
    ///
    /// # Examples
    /// ```
    /// use risc_v_compiler::scanner;
    /// let mut scanner = scanner::Scanner::new("let x := 42;");
    /// let tokens = scanner.scan();
    /// ```
    pub fn scan(&mut self) -> Result<Vec<SyntaxToken<'a>>, LexError> {
        let mut tokens = Vec::new();
        while let Some(curr) = self.peek() {
            let next = self.peek_next();
            let start = self.loc;
            match (curr, next) {
                (' ' | '\t' | '\n' | '\r', _) => {
                    self.advance();
                }
                (':', Some('=')) => tokens.push({
                    self.advance();
                    self.advance();
                    self.make_token_from(TokenType::Assignment, start)
                }),
                ('=', Some('=')) => tokens.push({
                    self.advance();
                    self.advance();
                    self.make_token_from(TokenType::Equality, start)
                }),
                ('-', Some('>')) => tokens.push({
                    self.advance();
                    self.advance();
                    self.make_token_from(TokenType::Arrow, start)
                }),
                ('(', _) => tokens.push({
                    self.advance();
                    self.make_token_from(TokenType::LParen, start)
                }),
                (')', _) => tokens.push({
                    self.advance();
                    self.make_token_from(TokenType::RParen, start)
                }),
                ('{', _) => tokens.push({
                    self.advance();
                    self.make_token_from(TokenType::LCurly, start)
                }),
                ('}', _) => tokens.push({
                    self.advance();
                    self.make_token_from(TokenType::RCurly, start)
                }),
                (';', _) => tokens.push({
                    self.advance();
                    self.make_token_from(TokenType::Semi, start)
                }),
                (':', _) => tokens.push({
                    self.advance();
                    self.make_token_from(TokenType::Colon, start)
                }),
                (',', _) => tokens.push({
                    self.advance();
                    self.make_token_from(TokenType::Comma, start)
                }),
                ('+', _) => tokens.push({
                    self.advance();
                    self.make_token_from(TokenType::Plus, start)
                }),
                ('-', _) => tokens.push({
                    self.advance();
                    self.make_token_from(TokenType::Minus, start)
                }),
                ('>', _) => tokens.push({
                    self.advance();
                    self.make_token_from(TokenType::Grt, start)
                }),
                ('0'..='9', _) => tokens.push(self.emit_number()?),
                ('a'..='z' | 'A'..='Z' | '_', _) => tokens.push(self.emit_id()?),
                (_, _) => return Err(LexError::UnexpectedChar(self.peek(), self.loc)),
            }
        }
        tokens.push(SyntaxToken {
            kind: TokenType::Eof,
            value: None,
            span: Span {
                start: self.loc,
                end: self.loc,
            },
        });
        Ok(tokens)
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
    fn emit_number_matches_number() -> Result<(), LexError> {
        let mut s = Scanner::new("12345");
        let t = s.emit_number()?;
        assert_eq!(t.value.unwrap(), "12345");
        Ok(())
    }

    #[test]
    fn emit_number_ignores_zero_as_head() {
        let mut s = Scanner::new("09");
        let err = s.emit_number().unwrap_err();
        assert!(matches!(err, LexError::UnexpectedChar(Some('9'), ..)));
    }

    #[test]
    fn emit_number_no_digit() {
        let mut s = Scanner::new("e");
        let err = s.emit_number().unwrap_err();
        assert!(matches!(err, LexError::UnexpectedChar(Some('e'), ..)));
    }

    #[test]
    fn emit_number_emits_zero() -> Result<(), LexError> {
        let mut s = Scanner::new("0");
        let t = s.emit_number()?;
        assert_eq!(t.value.unwrap(), "0");
        Ok(())
    }

    // ── emit_id ───────────────────────────────────────────────────────────────
    #[test]
    fn emit_id_no_invalid_char() {
        let mut s = Scanner::new("!");
        let err = s.emit_id().unwrap_err();
        assert!(matches!(err, LexError::UnexpectedChar(Some('!'), ..)));
    }

    #[test]
    fn emit_id_take_id() -> Result<(), LexError> {
        let mut s = Scanner::new("this_is_an_id");
        let id = s.emit_id()?;
        assert!(matches!(id.value.unwrap(), "this_is_an_id"));
        Ok(())
    }

    #[test]
    fn emit_id_take_id_containing_keyword() -> Result<(), LexError> {
        let mut s = Scanner::new("this_is_an_int_id");
        let id = s.emit_id()?;
        assert!(matches!(id.value.unwrap(), "this_is_an_int_id"));
        assert!(matches!(id.kind, TokenType::Id));
        Ok(())
    }

    #[test]
    fn emit_id_match_keyword() -> Result<(), LexError> {
        let mut s = Scanner::new("return");
        let id = s.emit_id()?;
        assert!(matches!(id.value.unwrap(), "return"));
        assert!(matches!(id.kind, TokenType::Return));
        Ok(())
    }

    #[test]
    fn emit_id_reject_leading_digit() {
        let mut s = Scanner::new("0_hi");
        let err = s.emit_id().unwrap_err();
        assert!(matches!(err, LexError::UnexpectedChar(Some('0'), ..)));
    }
}

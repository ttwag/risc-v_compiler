use std::fs;
use std::vec;

#[derive(Debug)]
pub enum TokenType {
    // Punctuation
    LParen,
    RParen,
    LCurly,
    RCurly,
    Semi,
    Comma,
    Colon,
    // Operators
    Plus,
    Minus,
    Assignment,
    Equality,
    Grt,
    Arrow,
    // Keywords
    Int,
    Let,
    Function,
    While,
    If,
    ElseIf,
    Else,
    Return,
    // Terminals
    Num,
    Id,
    // End
    Eof,
}

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenType,
    value: Option<&'a str>,
    length: usize,
    line: usize,
    col: usize,
}

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar(Option<char>, usize, usize), //character, line, col
}

pub struct Scanner<'a> {
    input: &'a str,
    index: usize,
    line: usize,
    col: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.index..].chars().nth(0)
    }

    fn peek_next(&self) -> Option<char> {
        self.input[self.index..].chars().nth(1)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        if let Some(ch) = c {
            self.index += ch.len_utf8();
            match ch {
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                }
                _ => {
                    self.col += 1;
                }
            }
        };
        c
    }

    fn make_token(&self, kind: TokenType, len: usize) -> Token<'a> {
        Token {
            kind,
            value: self.input.get(self.index..self.index + len),
            length: len,
            line: self.line,
            col: self.col,
        }
    }

    fn make_token_from(&self, kind: TokenType, start: usize, line: usize, col: usize) -> Token<'a> {
        let len = self.index - start;
        Token {
            kind,
            value: self.input.get(start..self.index),
            length: len,
            line: line,
            col: col,
        }
    }

    fn emit(&mut self, kind: TokenType, len: usize) -> Token<'a> {
        let token = self.make_token(kind, len);
        for _ in 0..len {
            self.advance();
        }
        token
    }

    ///
    /// From current input index, scans for a number matching the pattern 0 | [1-9][0-9]*
    /// and return the tokenized result. Advanced past all consumed digits.
    /// Returns an error when no digit or zero leading other digits
    ///
    fn emit_number(&mut self) -> Result<Token<'a>, LexError> {
        let start = self.index;
        let line = self.line;
        let col = self.col;

        // advance
        match self.peek() {
            Some('0') => {
                self.advance();
                if matches!(self.peek(), Some('0'..='9')) {
                    return Err(LexError::UnexpectedChar(self.peek(), self.line, self.col));
                }
                Ok(self.make_token_from(TokenType::Num, start, line, col))
            }
            Some('1'..='9') => {
                self.advance();
                while let Some(next) = self.peek() {
                    if !matches!(next, '0'..='9') {
                        break;
                    }
                    self.advance();
                }
                Ok(self.make_token_from(TokenType::Num, start, line, col))
            }
            _ => Err(LexError::UnexpectedChar(self.peek(), self.line, self.col)),
        }
    }

    fn match_keyword(&self, s: Option<&str>) -> Option<TokenType> {
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

    fn emit_id(&mut self) -> Token<'a> {
        let start = self.index;
        let line = self.line;
        let col = self.col;

        //advance
        match self.peek() {
            Some('a'..='z' | 'A'..='Z' | '_') => {
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            _ => {}
        }

        // make token from the current index
        let mut token = self.make_token_from(TokenType::Id, start, line, col);
        if let Some(kind) = self.match_keyword(token.value) {
            token.kind = kind;
        }
        token
    }

    /// Scans the input and returns a list of tokens as defined in grammar.ebnf.
    ///
    /// # Panics
    /// Panics on unexpected characters.
    ///
    /// # Examples
    /// ```
    /// let mut scanner = Scanner::new("let x := 42;");
    /// let tokens = scanner.scan();
    /// ```
    pub fn scan(&mut self) -> Result<Vec<Token<'a>>, LexError> {
        let mut tokens = Vec::new();
        while let Some(curr) = self.peek() {
            let next = self.peek_next();
            match (curr, next) {
                (' ' | '\t' | '\n' | '\r', _) => {
                    self.advance();
                }
                (':', Some('=')) => tokens.push(self.emit(TokenType::Assignment, 2)),
                ('=', Some('=')) => tokens.push(self.emit(TokenType::Equality, 2)),
                ('-', Some('>')) => tokens.push(self.emit(TokenType::Arrow, 2)),
                ('(', _) => tokens.push(self.emit(TokenType::LParen, 1)),
                (')', _) => tokens.push(self.emit(TokenType::RParen, 1)),
                ('{', _) => tokens.push(self.emit(TokenType::LCurly, 1)),
                ('}', _) => tokens.push(self.emit(TokenType::RCurly, 1)),
                (';', _) => tokens.push(self.emit(TokenType::Semi, 1)),
                (':', _) => tokens.push(self.emit(TokenType::Colon, 1)),
                (',', _) => tokens.push(self.emit(TokenType::Comma, 1)),
                ('+', _) => tokens.push(self.emit(TokenType::Plus, 1)),
                ('-', _) => tokens.push(self.emit(TokenType::Minus, 1)),
                ('>', _) => tokens.push(self.emit(TokenType::Grt, 1)),
                ('0'..='9', _) => tokens.push(self.emit_number()?),
                ('a'..='z' | 'A'..='Z' | '_', _) => tokens.push(self.emit_id()),
                (c, _) => panic!(
                    "unexpected character '{}' at line {} col {}",
                    c, self.line, self.col
                ),
            }
        }
        tokens.push(Token {
            kind: TokenType::Eof,
            value: None,
            length: 0,
            line: self.line,
            col: self.col,
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
        assert_eq!(s.index, 0);
    }

    #[test]
    fn peek_handles_unicode() {
        let s = Scanner::new("£");
        s.peek();
        s.peek();
        assert_eq!(s.index, 0);
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
        assert_eq!(s.index, 1);
    }

    #[test]
    fn advance_new_line_increase_line_and_resets_col() {
        let mut s = Scanner::new("\n");
        s.advance();
        assert_eq!(s.line, 2);
        assert_eq!(s.col, 1);
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
}

use std::fs;
use std::vec;
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

pub struct Token<'a> {
    kind: TokenType,
    value: Option<&'a str>,
    length: usize,
    line: usize,
    col: usize,
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

    fn emit_number(&mut self) -> Token<'a> {
        let start = self.index;
        let line = self.line;
        let col = self.col;

        // advance
        match self.peek() {
            Some('0') => {
                self.advance();
            }
            Some('1'..='9') => {
                self.advance();
                while let Some(num) = self.peek() {
                    if num.is_numeric() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            _ => {}
        }

        // make token from the current index
        self.make_token_from(TokenType::Num, start, line, col)
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
    pub fn scan(&mut self) -> Vec<Token<'a>> {
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
                ('0' | '1'..='9', _) => tokens.push(self.emit_number()),
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
        tokens
    }
}

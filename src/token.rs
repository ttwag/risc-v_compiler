#[derive(Debug, PartialEq)]
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
    pub kind: TokenType,
    pub value: Option<&'a str>,
    pub start: Location,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

impl Location {
    pub fn advance(&mut self, ch: char) {
        match ch {
            '\n' => {
                self.line += 1;
                self.col = 1;
            }
            _ => {
                self.col += 1;
            }
        }
    }
}

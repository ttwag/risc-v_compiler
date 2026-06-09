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
pub struct SyntaxToken {
    pub kind: TokenType,
    pub span: Span,
}

impl<'a> SyntaxToken {
    pub fn get_str(&self, stream: &'a str) -> Option<&'a str> {
        stream.get(self.span.start.index..self.span.end.index)
    }
}

#[derive(Debug)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Location {
    pub index: usize, // byte offset into source file
    pub line: usize,  // current line in the file; \n increments line
    pub col: usize,   // current position in the line
}

impl Location {
    pub fn new() -> Self {
        Self {
            index: 0,
            line: 1,
            col: 1,
        }
    }
}

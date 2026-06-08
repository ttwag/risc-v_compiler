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
pub struct SyntaxToken<'a> {
    pub kind: TokenType,
    pub value: Option<&'a str>,
    pub span: Span,
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

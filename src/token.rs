use std::fmt;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Token {
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
    Num(String),
    Id(String),
    // End
    #[default]
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LCurly => write!(f, "{{"),
            Token::RCurly => write!(f, "}}"),
            Token::Semi => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Assignment => write!(f, ":="),
            Token::Equality => write!(f, "=="),
            Token::Grt => write!(f, ">"),
            Token::Arrow => write!(f, "->"),
            Token::Int => write!(f, "int"),
            Token::Let => write!(f, "let"),
            Token::Function => write!(f, "fn"),
            Token::While => write!(f, "while"),
            Token::If => write!(f, "if"),
            Token::ElseIf => write!(f, "elif"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::Num(n) => write!(f, "{n}"),
            Token::Id(id) => write!(f, "{id}"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SyntaxToken {
    pub token: Token,
    pub span: Span,
}

impl fmt::Display for SyntaxToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token: {}\n{}", self.token, self.span)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Start:\n    {}\nEnd:\n    {}", self.start, self.end)
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
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

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {}, Char: {}", self.line, self.col)
    }
}

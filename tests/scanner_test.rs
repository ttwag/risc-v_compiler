use risc_v_compiler::{
    scanner::{ScanError, Scanner},
    token::{Location, SyntaxToken, TokenType},
};

#[track_caller]
fn assert_token(
    token: &SyntaxToken,
    kind: TokenType,
    value: Option<&str>,
    start: Location,
    end: Location,
) {
    assert_eq!(token.kind, kind);
    assert_eq!(token.value, value);
    assert_eq!(token.span.start, start);
    assert_eq!(token.span.end, end);
}

//
// Input:
// fn has_arg(a:int ) -> int {
// return a;}
//
#[rustfmt::skip]
#[test]
fn scan_function_with_arg() {
    let mut s = Scanner::new("fn has_arg(a:int ) -> int {\nreturn a;}");
    let token = s.scan().unwrap();

    assert_token(&token[0],  TokenType::Function, Some("fn"),     Location { index:  0, line: 1, col:  1 }, Location { index:  2, line: 1, col:  3 });
    assert_token(&token[1],  TokenType::Id,       Some("has_arg"), Location { index:  3, line: 1, col:  4 }, Location { index: 10, line: 1, col: 11 });
    assert_token(&token[2],  TokenType::LParen,   Some("("),      Location { index: 10, line: 1, col: 11 }, Location { index: 11, line: 1, col: 12 });
    assert_token(&token[3],  TokenType::Id,       Some("a"),      Location { index: 11, line: 1, col: 12 }, Location { index: 12, line: 1, col: 13 });
    assert_token(&token[4],  TokenType::Colon,    Some(":"),      Location { index: 12, line: 1, col: 13 }, Location { index: 13, line: 1, col: 14 });
    assert_token(&token[5],  TokenType::Int,      Some("int"),    Location { index: 13, line: 1, col: 14 }, Location { index: 16, line: 1, col: 17 });
    assert_token(&token[6],  TokenType::RParen,   Some(")"),      Location { index: 17, line: 1, col: 18 }, Location { index: 18, line: 1, col: 19 });
    assert_token(&token[7],  TokenType::Arrow,    Some("->"),     Location { index: 19, line: 1, col: 20 }, Location { index: 21, line: 1, col: 22 });
    assert_token(&token[8],  TokenType::Int,      Some("int"),    Location { index: 22, line: 1, col: 23 }, Location { index: 25, line: 1, col: 26 });
    assert_token(&token[9],  TokenType::LCurly,   Some("{"),      Location { index: 26, line: 1, col: 27 }, Location { index: 27, line: 1, col: 28 });
    assert_token(&token[10], TokenType::Return,   Some("return"), Location { index: 28, line: 2, col:  1 }, Location { index: 34, line: 2, col:  7 });
    assert_token(&token[11], TokenType::Id,       Some("a"),      Location { index: 35, line: 2, col:  8 }, Location { index: 36, line: 2, col:  9 });
    assert_token(&token[12], TokenType::Semi,     Some(";"),      Location { index: 36, line: 2, col:  9 }, Location { index: 37, line: 2, col: 10 });
    assert_token(&token[13], TokenType::RCurly,   Some("}"),      Location { index: 37, line: 2, col: 10 }, Location { index: 38, line: 2, col: 11 });
    assert_token(&token[14], TokenType::Eof,      None,           Location { index: 38, line: 2, col: 11 }, Location { index: 38, line: 2, col: 11 });
}

//
// Input:
// fn assignment() -> int {
//  let a: int := 100;
//  return a;
//  }
//
#[rustfmt::skip]
#[test]
fn scan_function_with_assignment() {
    let mut s = Scanner::new("fn assignment() -> int { \n let a: int := 100; \n return a; \n }");
    let token = s.scan().unwrap();

    assert_token(&token[0],  TokenType::Function,    Some("fn"),       Location { index:  0, line: 1, col:  1 }, Location { index:  2, line: 1, col:  3 });
    assert_token(&token[1],  TokenType::Id,          Some("assignment"), Location { index:  3, line: 1, col:  4 }, Location { index: 13, line: 1, col: 14 });
    assert_token(&token[2],  TokenType::LParen,      Some("("),        Location { index: 13, line: 1, col: 14 }, Location { index: 14, line: 1, col: 15 });
    assert_token(&token[3],  TokenType::RParen,      Some(")"),        Location { index: 14, line: 1, col: 15 }, Location { index: 15, line: 1, col: 16 });
    assert_token(&token[4],  TokenType::Arrow,       Some("->"),       Location { index: 16, line: 1, col: 17 }, Location { index: 18, line: 1, col: 19 });
    assert_token(&token[5],  TokenType::Int,         Some("int"),      Location { index: 19, line: 1, col: 20 }, Location { index: 22, line: 1, col: 23 });
    assert_token(&token[6],  TokenType::LCurly,      Some("{"),        Location { index: 23, line: 1, col: 24 }, Location { index: 24, line: 1, col: 25 });
    assert_token(&token[7],  TokenType::Let,         Some("let"),      Location { index: 27, line: 2, col:  2 }, Location { index: 30, line: 2, col:  5 });
    assert_token(&token[8],  TokenType::Id,          Some("a"),        Location { index: 31, line: 2, col:  6 }, Location { index: 32, line: 2, col:  7 });
    assert_token(&token[9],  TokenType::Colon,       Some(":"),        Location { index: 32, line: 2, col:  7 }, Location { index: 33, line: 2, col:  8 });
    assert_token(&token[10], TokenType::Int,         Some("int"),      Location { index: 34, line: 2, col:  9 }, Location { index: 37, line: 2, col: 12 });
    assert_token(&token[11], TokenType::Assignment,  Some(":="),       Location { index: 38, line: 2, col: 13 }, Location { index: 40, line: 2, col: 15 });
    assert_token(&token[12], TokenType::Num,         Some("100"),      Location { index: 41, line: 2, col: 16 }, Location { index: 44, line: 2, col: 19 });
    assert_token(&token[13], TokenType::Semi,        Some(";"),        Location { index: 44, line: 2, col: 19 }, Location { index: 45, line: 2, col: 20 });
    assert_token(&token[14], TokenType::Return,      Some("return"),   Location { index: 48, line: 3, col:  2 }, Location { index: 54, line: 3, col:  8 });
    assert_token(&token[15], TokenType::Id,          Some("a"),        Location { index: 55, line: 3, col:  9 }, Location { index: 56, line: 3, col: 10 });
    assert_token(&token[16], TokenType::Semi,        Some(";"),        Location { index: 56, line: 3, col: 10 }, Location { index: 57, line: 3, col: 11 });
    assert_token(&token[17], TokenType::RCurly,      Some("}"),        Location { index: 60, line: 4, col:  2 }, Location { index: 61, line: 4, col:  3 });
    assert_token(&token[18], TokenType::Eof,         None,             Location { index: 61, line: 4, col:  3 }, Location { index: 61, line: 4, col:  3 });
}

// Input:
// fn branch(a: int) -> int {
//     if(a > 1) {
//         return 0;
//     }
//     return 5;
// }
#[rustfmt::skip]
#[test]
fn scan_function_with_branch() {
    let mut s = Scanner::new(
        "fn branch(a: int) -> int {\n    if(a > 1) {\n        return 0;\n}\n    return 5;\n}",
    );
    let token = s.scan().unwrap();
    assert_token(&token[0],  TokenType::Function, Some("fn"),     Location { index:  0, line: 1, col:  1 }, Location { index:  2, line: 1, col:  3 });
    assert_token(&token[1],  TokenType::Id,       Some("branch"), Location { index:  3, line: 1, col:  4 }, Location { index:  9, line: 1, col: 10 });
    assert_token(&token[2],  TokenType::LParen,   Some("("),      Location { index:  9, line: 1, col: 10 }, Location { index: 10, line: 1, col: 11 });
    assert_token(&token[3],  TokenType::Id,       Some("a"),      Location { index: 10, line: 1, col: 11 }, Location { index: 11, line: 1, col: 12 });
    assert_token(&token[4],  TokenType::Colon,    Some(":"),      Location { index: 11, line: 1, col: 12 }, Location { index: 12, line: 1, col: 13 });
    assert_token(&token[5],  TokenType::Int,      Some("int"),    Location { index: 13, line: 1, col: 14 }, Location { index: 16, line: 1, col: 17 });
    assert_token(&token[6],  TokenType::RParen,   Some(")"),      Location { index: 16, line: 1, col: 17 }, Location { index: 17, line: 1, col: 18 });
    assert_token(&token[7],  TokenType::Arrow,    Some("->"),     Location { index: 18, line: 1, col: 19 }, Location { index: 20, line: 1, col: 21 });
    assert_token(&token[8],  TokenType::Int,      Some("int"),    Location { index: 21, line: 1, col: 22 }, Location { index: 24, line: 1, col: 25 });
    assert_token(&token[9],  TokenType::LCurly,   Some("{"),      Location { index: 25, line: 1, col: 26 }, Location { index: 26, line: 1, col: 27 });
    assert_token(&token[10], TokenType::If,       Some("if"),     Location { index: 31, line: 2, col:  5 }, Location { index: 33, line: 2, col:  7 });
    assert_token(&token[11], TokenType::LParen,   Some("("),      Location { index: 33, line: 2, col:  7 }, Location { index: 34, line: 2, col:  8 });
    assert_token(&token[12], TokenType::Id,       Some("a"),      Location { index: 34, line: 2, col:  8 }, Location { index: 35, line: 2, col:  9 });
    assert_token(&token[13], TokenType::Grt,      Some(">"),      Location { index: 36, line: 2, col: 10 }, Location { index: 37, line: 2, col: 11 });
    assert_token(&token[14], TokenType::Num,      Some("1"),      Location { index: 38, line: 2, col: 12 }, Location { index: 39, line: 2, col: 13 });
    assert_token(&token[15], TokenType::RParen,   Some(")"),      Location { index: 39, line: 2, col: 13 }, Location { index: 40, line: 2, col: 14 });
    assert_token(&token[16], TokenType::LCurly,   Some("{"),      Location { index: 41, line: 2, col: 15 }, Location { index: 42, line: 2, col: 16 });
    assert_token(&token[17], TokenType::Return,   Some("return"), Location { index: 51, line: 3, col:  9 }, Location { index: 57, line: 3, col: 15 });
    assert_token(&token[18], TokenType::Num,      Some("0"),      Location { index: 58, line: 3, col: 16 }, Location { index: 59, line: 3, col: 17 });
    assert_token(&token[19], TokenType::Semi,     Some(";"),      Location { index: 59, line: 3, col: 17 }, Location { index: 60, line: 3, col: 18 });
    assert_token(&token[20], TokenType::RCurly,   Some("}"),      Location { index: 61, line: 4, col:  1 }, Location { index: 62, line: 4, col:  2 });
    assert_token(&token[21], TokenType::Return,   Some("return"), Location { index: 67, line: 5, col:  5 }, Location { index: 73, line: 5, col: 11 });
    assert_token(&token[22], TokenType::Num,      Some("5"),      Location { index: 74, line: 5, col: 12 }, Location { index: 75, line: 5, col: 13 });
    assert_token(&token[23], TokenType::Semi,     Some(";"),      Location { index: 75, line: 5, col: 13 }, Location { index: 76, line: 5, col: 14 });
    assert_token(&token[24], TokenType::RCurly,   Some("}"),      Location { index: 77, line: 6, col:  1 }, Location { index: 78, line: 6, col:  2 });
    assert_token(&token[25], TokenType::Eof,      None,           Location { index: 78, line: 6, col:  2 }, Location { index: 78, line: 6, col:  2 });
}

// Input:
// fn loop() -> int {
//     let i: int := 0;
//     while (657 - i > 0) {
//         i := i + 1;
//     }
//     return i;
// }
#[rustfmt::skip]
#[test]
fn scan_function_with_while_loop() {
    let mut s = Scanner::new(
        "fn loop() -> int {\n    let i: int := 0;\n    while (657 - i > 0) {\n    i := i + 1;\n    }\n    return i;\n}",
    );
    let token = s.scan().unwrap();

    assert_token(&token[0],  TokenType::Function,   Some("fn"),      Location { index:  0, line: 1, col:  1 }, Location { index:  2, line: 1, col:  3 });
    assert_token(&token[1],  TokenType::Id,         Some("loop"),    Location { index:  3, line: 1, col:  4 }, Location { index:  7, line: 1, col:  8 });
    assert_token(&token[2],  TokenType::LParen,     Some("("),       Location { index:  7, line: 1, col:  8 }, Location { index:  8, line: 1, col:  9 });
    assert_token(&token[3],  TokenType::RParen,     Some(")"),       Location { index:  8, line: 1, col:  9 }, Location { index:  9, line: 1, col: 10 });
    assert_token(&token[4],  TokenType::Arrow,      Some("->"),      Location { index: 10, line: 1, col: 11 }, Location { index: 12, line: 1, col: 13 });
    assert_token(&token[5],  TokenType::Int,        Some("int"),     Location { index: 13, line: 1, col: 14 }, Location { index: 16, line: 1, col: 17 });
    assert_token(&token[6],  TokenType::LCurly,     Some("{"),       Location { index: 17, line: 1, col: 18 }, Location { index: 18, line: 1, col: 19 });
    assert_token(&token[7],  TokenType::Let,        Some("let"),     Location { index: 23, line: 2, col:  5 }, Location { index: 26, line: 2, col:  8 });
    assert_token(&token[8],  TokenType::Id,         Some("i"),       Location { index: 27, line: 2, col:  9 }, Location { index: 28, line: 2, col: 10 });
    assert_token(&token[9],  TokenType::Colon,      Some(":"),       Location { index: 28, line: 2, col: 10 }, Location { index: 29, line: 2, col: 11 });
    assert_token(&token[10], TokenType::Int,        Some("int"),     Location { index: 30, line: 2, col: 12 }, Location { index: 33, line: 2, col: 15 });
    assert_token(&token[11], TokenType::Assignment, Some(":="),      Location { index: 34, line: 2, col: 16 }, Location { index: 36, line: 2, col: 18 });
    assert_token(&token[12], TokenType::Num,        Some("0"),       Location { index: 37, line: 2, col: 19 }, Location { index: 38, line: 2, col: 20 });
    assert_token(&token[13], TokenType::Semi,       Some(";"),       Location { index: 38, line: 2, col: 20 }, Location { index: 39, line: 2, col: 21 });
    assert_token(&token[14], TokenType::While,      Some("while"),   Location { index: 44, line: 3, col:  5 }, Location { index: 49, line: 3, col: 10 });
    assert_token(&token[15], TokenType::LParen,     Some("("),       Location { index: 50, line: 3, col: 11 }, Location { index: 51, line: 3, col: 12 });
    assert_token(&token[16], TokenType::Num,        Some("657"),     Location { index: 51, line: 3, col: 12 }, Location { index: 54, line: 3, col: 15 });
    assert_token(&token[17], TokenType::Minus,      Some("-"),       Location { index: 55, line: 3, col: 16 }, Location { index: 56, line: 3, col: 17 });
    assert_token(&token[18], TokenType::Id,         Some("i"),       Location { index: 57, line: 3, col: 18 }, Location { index: 58, line: 3, col: 19 });
    assert_token(&token[19], TokenType::Grt,        Some(">"),       Location { index: 59, line: 3, col: 20 }, Location { index: 60, line: 3, col: 21 });
    assert_token(&token[20], TokenType::Num,        Some("0"),       Location { index: 61, line: 3, col: 22 }, Location { index: 62, line: 3, col: 23 });
    assert_token(&token[21], TokenType::RParen,     Some(")"),       Location { index: 62, line: 3, col: 23 }, Location { index: 63, line: 3, col: 24 });
    assert_token(&token[22], TokenType::LCurly,     Some("{"),       Location { index: 64, line: 3, col: 25 }, Location { index: 65, line: 3, col: 26 });
    assert_token(&token[23], TokenType::Id,         Some("i"),       Location { index: 70, line: 4, col:  5 }, Location { index: 71, line: 4, col:  6 });
    assert_token(&token[24], TokenType::Assignment, Some(":="),      Location { index: 72, line: 4, col:  7 }, Location { index: 74, line: 4, col:  9 });
    assert_token(&token[25], TokenType::Id,         Some("i"),       Location { index: 75, line: 4, col: 10 }, Location { index: 76, line: 4, col: 11 });
    assert_token(&token[26], TokenType::Plus,       Some("+"),       Location { index: 77, line: 4, col: 12 }, Location { index: 78, line: 4, col: 13 });
    assert_token(&token[27], TokenType::Num,        Some("1"),       Location { index: 79, line: 4, col: 14 }, Location { index: 80, line: 4, col: 15 });
    assert_token(&token[28], TokenType::Semi,       Some(";"),       Location { index: 80, line: 4, col: 15 }, Location { index: 81, line: 4, col: 16 });
    assert_token(&token[29], TokenType::RCurly,     Some("}"),       Location { index: 86, line: 5, col:  5 }, Location { index: 87, line: 5, col:  6 });
    assert_token(&token[30], TokenType::Return,     Some("return"),  Location { index: 92, line: 6, col:  5 }, Location { index: 98, line: 6, col: 11 });
    assert_token(&token[31], TokenType::Id,         Some("i"),       Location { index: 99, line: 6, col: 12 }, Location { index: 100, line: 6, col: 13 });
    assert_token(&token[32], TokenType::Semi,       Some(";"),       Location { index: 100, line: 6, col: 13 }, Location { index: 101, line: 6, col: 14 });
    assert_token(&token[33], TokenType::RCurly,     Some("}"),       Location { index: 102, line: 7, col:  1 }, Location { index: 103, line: 7, col:  2 });
    assert_token(&token[34], TokenType::Eof,        None,            Location { index: 103, line: 7, col:  2 }, Location { index: 103, line: 7, col:  2 });
}

// Input:
// fn fib(n: int) -> int {
//     if (n == 0) {
//         return 0;
//     }
//     elif (n == 1) {
//         return 1;
//     }
//     return fib(n - 1) + fib(n - 2);
// }
#[rustfmt::skip]
#[test]
fn scan_function_fib() {
    let mut s = Scanner::new(
        "fn fib(n: int) -> int {\n    if (n == 0) {\n        return 0;\n    }\n    elif (n == 1) {\n        return 1;\n    }\n    return fib(n - 1) + fib(n - 2);\n}",
    );
    let token = s.scan().unwrap();

    assert_token(&token[0],  TokenType::Function,  Some("fn"),       Location { index:   0, line: 1, col:  1 }, Location { index:   2, line: 1, col:  3 });
    assert_token(&token[1],  TokenType::Id,        Some("fib"),      Location { index:   3, line: 1, col:  4 }, Location { index:   6, line: 1, col:  7 });
    assert_token(&token[2],  TokenType::LParen,    Some("("),        Location { index:   6, line: 1, col:  7 }, Location { index:   7, line: 1, col:  8 });
    assert_token(&token[3],  TokenType::Id,        Some("n"),        Location { index:   7, line: 1, col:  8 }, Location { index:   8, line: 1, col:  9 });
    assert_token(&token[4],  TokenType::Colon,     Some(":"),        Location { index:   8, line: 1, col:  9 }, Location { index:   9, line: 1, col: 10 });
    assert_token(&token[5],  TokenType::Int,       Some("int"),      Location { index:  10, line: 1, col: 11 }, Location { index:  13, line: 1, col: 14 });
    assert_token(&token[6],  TokenType::RParen,    Some(")"),        Location { index:  13, line: 1, col: 14 }, Location { index:  14, line: 1, col: 15 });
    assert_token(&token[7],  TokenType::Arrow,     Some("->"),       Location { index:  15, line: 1, col: 16 }, Location { index:  17, line: 1, col: 18 });
    assert_token(&token[8],  TokenType::Int,       Some("int"),      Location { index:  18, line: 1, col: 19 }, Location { index:  21, line: 1, col: 22 });
    assert_token(&token[9],  TokenType::LCurly,    Some("{"),        Location { index:  22, line: 1, col: 23 }, Location { index:  23, line: 1, col: 24 });
    assert_token(&token[10], TokenType::If,        Some("if"),       Location { index:  28, line: 2, col:  5 }, Location { index:  30, line: 2, col:  7 });
    assert_token(&token[11], TokenType::LParen,    Some("("),        Location { index:  31, line: 2, col:  8 }, Location { index:  32, line: 2, col:  9 });
    assert_token(&token[12], TokenType::Id,        Some("n"),        Location { index:  32, line: 2, col:  9 }, Location { index:  33, line: 2, col: 10 });
    assert_token(&token[13], TokenType::Equality,  Some("=="),       Location { index:  34, line: 2, col: 11 }, Location { index:  36, line: 2, col: 13 });
    assert_token(&token[14], TokenType::Num,       Some("0"),        Location { index:  37, line: 2, col: 14 }, Location { index:  38, line: 2, col: 15 });
    assert_token(&token[15], TokenType::RParen,    Some(")"),        Location { index:  38, line: 2, col: 15 }, Location { index:  39, line: 2, col: 16 });
    assert_token(&token[16], TokenType::LCurly,    Some("{"),        Location { index:  40, line: 2, col: 17 }, Location { index:  41, line: 2, col: 18 });
    assert_token(&token[17], TokenType::Return,    Some("return"),   Location { index:  50, line: 3, col:  9 }, Location { index:  56, line: 3, col: 15 });
    assert_token(&token[18], TokenType::Num,       Some("0"),        Location { index:  57, line: 3, col: 16 }, Location { index:  58, line: 3, col: 17 });
    assert_token(&token[19], TokenType::Semi,      Some(";"),        Location { index:  58, line: 3, col: 17 }, Location { index:  59, line: 3, col: 18 });
    assert_token(&token[20], TokenType::RCurly,    Some("}"),        Location { index:  64, line: 4, col:  5 }, Location { index:  65, line: 4, col:  6 });
    assert_token(&token[21], TokenType::ElseIf,    Some("elif"),     Location { index:  70, line: 5, col:  5 }, Location { index:  74, line: 5, col:  9 });
    assert_token(&token[22], TokenType::LParen,    Some("("),        Location { index:  75, line: 5, col: 10 }, Location { index:  76, line: 5, col: 11 });
    assert_token(&token[23], TokenType::Id,        Some("n"),        Location { index:  76, line: 5, col: 11 }, Location { index:  77, line: 5, col: 12 });
    assert_token(&token[24], TokenType::Equality,  Some("=="),       Location { index:  78, line: 5, col: 13 }, Location { index:  80, line: 5, col: 15 });
    assert_token(&token[25], TokenType::Num,       Some("1"),        Location { index:  81, line: 5, col: 16 }, Location { index:  82, line: 5, col: 17 });
    assert_token(&token[26], TokenType::RParen,    Some(")"),        Location { index:  82, line: 5, col: 17 }, Location { index:  83, line: 5, col: 18 });
    assert_token(&token[27], TokenType::LCurly,    Some("{"),        Location { index:  84, line: 5, col: 19 }, Location { index:  85, line: 5, col: 20 });
    assert_token(&token[28], TokenType::Return,    Some("return"),   Location { index:  94, line: 6, col:  9 }, Location { index: 100, line: 6, col: 15 });
    assert_token(&token[29], TokenType::Num,       Some("1"),        Location { index: 101, line: 6, col: 16 }, Location { index: 102, line: 6, col: 17 });
    assert_token(&token[30], TokenType::Semi,      Some(";"),        Location { index: 102, line: 6, col: 17 }, Location { index: 103, line: 6, col: 18 });
    assert_token(&token[31], TokenType::RCurly,    Some("}"),        Location { index: 108, line: 7, col:  5 }, Location { index: 109, line: 7, col:  6 });
    assert_token(&token[32], TokenType::Return,    Some("return"),   Location { index: 114, line: 8, col:  5 }, Location { index: 120, line: 8, col: 11 });
    assert_token(&token[33], TokenType::Id,        Some("fib"),      Location { index: 121, line: 8, col: 12 }, Location { index: 124, line: 8, col: 15 });
    assert_token(&token[34], TokenType::LParen,    Some("("),        Location { index: 124, line: 8, col: 15 }, Location { index: 125, line: 8, col: 16 });
    assert_token(&token[35], TokenType::Id,        Some("n"),        Location { index: 125, line: 8, col: 16 }, Location { index: 126, line: 8, col: 17 });
    assert_token(&token[36], TokenType::Minus,     Some("-"),        Location { index: 127, line: 8, col: 18 }, Location { index: 128, line: 8, col: 19 });
    assert_token(&token[37], TokenType::Num,       Some("1"),        Location { index: 129, line: 8, col: 20 }, Location { index: 130, line: 8, col: 21 });
    assert_token(&token[38], TokenType::RParen,    Some(")"),        Location { index: 130, line: 8, col: 21 }, Location { index: 131, line: 8, col: 22 });
    assert_token(&token[39], TokenType::Plus,      Some("+"),        Location { index: 132, line: 8, col: 23 }, Location { index: 133, line: 8, col: 24 });
    assert_token(&token[40], TokenType::Id,        Some("fib"),      Location { index: 134, line: 8, col: 25 }, Location { index: 137, line: 8, col: 28 });
    assert_token(&token[41], TokenType::LParen,    Some("("),        Location { index: 137, line: 8, col: 28 }, Location { index: 138, line: 8, col: 29 });
    assert_token(&token[42], TokenType::Id,        Some("n"),        Location { index: 138, line: 8, col: 29 }, Location { index: 139, line: 8, col: 30 });
    assert_token(&token[43], TokenType::Minus,     Some("-"),        Location { index: 140, line: 8, col: 31 }, Location { index: 141, line: 8, col: 32 });
    assert_token(&token[44], TokenType::Num,       Some("2"),        Location { index: 142, line: 8, col: 33 }, Location { index: 143, line: 8, col: 34 });
    assert_token(&token[45], TokenType::RParen,    Some(")"),        Location { index: 143, line: 8, col: 34 }, Location { index: 144, line: 8, col: 35 });
    assert_token(&token[46], TokenType::Semi,      Some(";"),        Location { index: 144, line: 8, col: 35 }, Location { index: 145, line: 8, col: 36 });
    assert_token(&token[47], TokenType::RCurly,    Some("}"),        Location { index: 146, line: 9, col:  1 }, Location { index: 147, line: 9, col:  2 });
    assert_token(&token[48], TokenType::Eof,       None,             Location { index: 147, line: 9, col:  2 }, Location { index: 147, line: 9, col:  2 });
}

//
// Input:
// fn fib(n: int) -> int []
//
#[test]
fn scan_bad_function() {
    let mut s = Scanner::new("fn fib(n: int) -> int []");
    let err = s.scan().unwrap_err();
    assert!(matches!(err, ScanError::UnexpectedChar(Some('['), ..)));
}

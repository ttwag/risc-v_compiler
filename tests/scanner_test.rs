use risc_v_compiler::{
    scanner::{LexError, Scanner},
    token::{Location, Token, TokenType},
};

#[track_caller]
fn assert_token(token: &Token, kind: TokenType, value: Option<&str>, loc: Location) {
    assert_eq!(token.kind, kind);
    assert_eq!(token.value, value);
    assert_eq!(token.start, loc);
}

//
// Input:
// fn has_arg(a:int ) -> int {
// return a;}
//
#[test]
fn scan_function_with_arg() {
    let mut s = Scanner::new("fn has_arg(a:int ) -> int {\nreturn a;}");
    let token = s.scan().unwrap();

    assert_token(
        &token[0],
        TokenType::Function,
        Some("fn"),
        Location { line: 1, col: 1 },
    );
    assert_token(
        &token[1],
        TokenType::Id,
        Some("has_arg"),
        Location { line: 1, col: 4 },
    );
    assert_token(
        &token[2],
        TokenType::LParen,
        Some("("),
        Location { line: 1, col: 11 },
    );
    assert_token(
        &token[3],
        TokenType::Id,
        Some("a"),
        Location { line: 1, col: 12 },
    );
    assert_token(
        &token[4],
        TokenType::Colon,
        Some(":"),
        Location { line: 1, col: 13 },
    );
    assert_token(
        &token[5],
        TokenType::Int,
        Some("int"),
        Location { line: 1, col: 14 },
    );
    assert_token(
        &token[6],
        TokenType::RParen,
        Some(")"),
        Location { line: 1, col: 18 },
    );
    assert_token(
        &token[7],
        TokenType::Arrow,
        Some("->"),
        Location { line: 1, col: 20 },
    );
    assert_token(
        &token[8],
        TokenType::Int,
        Some("int"),
        Location { line: 1, col: 23 },
    );
    assert_token(
        &token[9],
        TokenType::LCurly,
        Some("{"),
        Location { line: 1, col: 27 },
    );
    assert_token(
        &token[10],
        TokenType::Return,
        Some("return"),
        Location { line: 2, col: 1 },
    );
    assert_token(
        &token[11],
        TokenType::Id,
        Some("a"),
        Location { line: 2, col: 8 },
    );
    assert_token(
        &token[12],
        TokenType::Semi,
        Some(";"),
        Location { line: 2, col: 9 },
    );
    assert_token(
        &token[13],
        TokenType::RCurly,
        Some("}"),
        Location { line: 2, col: 10 },
    );
    assert_token(
        &token[14],
        TokenType::Eof,
        None,
        Location { line: 2, col: 11 },
    );
}

//
// Input:
// fn assignment() -> int {
//  let a: int := 100;
//  return a;
//  }
//
#[test]
fn scan_function_with_assignment() {
    let mut s = Scanner::new("fn assignment() -> int { \n let a: int := 100; \n return a; \n }");
    let token = s.scan().unwrap();

    assert_token(
        &token[0],
        TokenType::Function,
        Some("fn"),
        Location { line: 1, col: 1 },
    );
    assert_token(
        &token[1],
        TokenType::Id,
        Some("assignment"),
        Location { line: 1, col: 4 },
    );
    assert_token(
        &token[2],
        TokenType::LParen,
        Some("("),
        Location { line: 1, col: 14 },
    );
    assert_token(
        &token[3],
        TokenType::RParen,
        Some(")"),
        Location { line: 1, col: 15 },
    );
    assert_token(
        &token[4],
        TokenType::Arrow,
        Some("->"),
        Location { line: 1, col: 17 },
    );
    assert_token(
        &token[5],
        TokenType::Int,
        Some("int"),
        Location { line: 1, col: 20 },
    );
    assert_token(
        &token[6],
        TokenType::LCurly,
        Some("{"),
        Location { line: 1, col: 24 },
    );
    assert_token(
        &token[7],
        TokenType::Let,
        Some("let"),
        Location { line: 2, col: 2 },
    );
    assert_token(
        &token[8],
        TokenType::Id,
        Some("a"),
        Location { line: 2, col: 6 },
    );
    assert_token(
        &token[9],
        TokenType::Colon,
        Some(":"),
        Location { line: 2, col: 7 },
    );
    assert_token(
        &token[10],
        TokenType::Int,
        Some("int"),
        Location { line: 2, col: 9 },
    );
    assert_token(
        &token[11],
        TokenType::Assignment,
        Some(":="),
        Location { line: 2, col: 13 },
    );
    assert_token(
        &token[12],
        TokenType::Num,
        Some("100"),
        Location { line: 2, col: 16 },
    );
    assert_token(
        &token[13],
        TokenType::Semi,
        Some(";"),
        Location { line: 2, col: 19 },
    );
    assert_token(
        &token[14],
        TokenType::Return,
        Some("return"),
        Location { line: 3, col: 2 },
    );
    assert_token(
        &token[15],
        TokenType::Id,
        Some("a"),
        Location { line: 3, col: 9 },
    );
    assert_token(
        &token[16],
        TokenType::Semi,
        Some(";"),
        Location { line: 3, col: 10 },
    );
    assert_token(
        &token[17],
        TokenType::RCurly,
        Some("}"),
        Location { line: 4, col: 2 },
    );
    assert_token(
        &token[18],
        TokenType::Eof,
        None,
        Location { line: 4, col: 3 },
    );
}

// Input:
// fn branch(a: int) -> int {
//     if(a > 1) {
//         return 0;
//     }
//     return 5;
// }
#[test]
fn scan_function_with_branch() {
    let mut s = Scanner::new(
        "fn branch(a: int) -> int {\n    if(a > 1) {\n        return 0;\n}\n    return 5;\n}",
    );
    let token = s.scan().unwrap();
    assert_token(
        &token[0],
        TokenType::Function,
        Some("fn"),
        Location { line: 1, col: 1 },
    );
    assert_token(
        &token[1],
        TokenType::Id,
        Some("branch"),
        Location { line: 1, col: 4 },
    );
    assert_token(
        &token[2],
        TokenType::LParen,
        Some("("),
        Location { line: 1, col: 10 },
    );
    assert_token(
        &token[3],
        TokenType::Id,
        Some("a"),
        Location { line: 1, col: 11 },
    );
    assert_token(
        &token[4],
        TokenType::Colon,
        Some(":"),
        Location { line: 1, col: 12 },
    );
    assert_token(
        &token[5],
        TokenType::Int,
        Some("int"),
        Location { line: 1, col: 14 },
    );
    assert_token(
        &token[6],
        TokenType::RParen,
        Some(")"),
        Location { line: 1, col: 17 },
    );
    assert_token(
        &token[7],
        TokenType::Arrow,
        Some("->"),
        Location { line: 1, col: 19 },
    );
    assert_token(
        &token[8],
        TokenType::Int,
        Some("int"),
        Location { line: 1, col: 22 },
    );
    assert_token(
        &token[9],
        TokenType::LCurly,
        Some("{"),
        Location { line: 1, col: 26 },
    );
    assert_token(
        &token[10],
        TokenType::If,
        Some("if"),
        Location { line: 2, col: 5 },
    );
    assert_token(
        &token[11],
        TokenType::LParen,
        Some("("),
        Location { line: 2, col: 7 },
    );
    assert_token(
        &token[12],
        TokenType::Id,
        Some("a"),
        Location { line: 2, col: 8 },
    );
    assert_token(
        &token[13],
        TokenType::Grt,
        Some(">"),
        Location { line: 2, col: 10 },
    );
    assert_token(
        &token[14],
        TokenType::Num,
        Some("1"),
        Location { line: 2, col: 12 },
    );
    assert_token(
        &token[15],
        TokenType::RParen,
        Some(")"),
        Location { line: 2, col: 13 },
    );
    assert_token(
        &token[16],
        TokenType::LCurly,
        Some("{"),
        Location { line: 2, col: 15 },
    );
    assert_token(
        &token[17],
        TokenType::Return,
        Some("return"),
        Location { line: 3, col: 9 },
    );
    assert_token(
        &token[18],
        TokenType::Num,
        Some("0"),
        Location { line: 3, col: 16 },
    );
    assert_token(
        &token[19],
        TokenType::Semi,
        Some(";"),
        Location { line: 3, col: 17 },
    );
    assert_token(
        &token[20],
        TokenType::RCurly,
        Some("}"),
        Location { line: 4, col: 1 },
    );
    assert_token(
        &token[21],
        TokenType::Return,
        Some("return"),
        Location { line: 5, col: 5 },
    );
    assert_token(
        &token[22],
        TokenType::Num,
        Some("5"),
        Location { line: 5, col: 12 },
    );
    assert_token(
        &token[23],
        TokenType::Semi,
        Some(";"),
        Location { line: 5, col: 13 },
    );
    assert_token(
        &token[24],
        TokenType::RCurly,
        Some("}"),
        Location { line: 6, col: 1 },
    );
    assert_token(
        &token[25],
        TokenType::Eof,
        None,
        Location { line: 6, col: 2 },
    );
}

// Input:
// fn loop() -> int {
//     let i: int := 0;
//     while (657 - i > 0) {
//         i := i + 1;
//     }
//     return i;
// }

#[test]
fn scan_function_with_while_loop() {
    let mut s = Scanner::new(
        "fn loop() -> int {\n    let i: int := 0;\n    while (657 - i > 0) {\n    i := i + 1;\n    }\n    return i;\n}",
    );
    let token = s.scan().unwrap();

    assert_token(
        &token[0],
        TokenType::Function,
        Some("fn"),
        Location { line: 1, col: 1 },
    );
    assert_token(
        &token[1],
        TokenType::Id,
        Some("loop"),
        Location { line: 1, col: 4 },
    );
    assert_token(
        &token[2],
        TokenType::LParen,
        Some("("),
        Location { line: 1, col: 8 },
    );
    assert_token(
        &token[3],
        TokenType::RParen,
        Some(")"),
        Location { line: 1, col: 9 },
    );
    assert_token(
        &token[4],
        TokenType::Arrow,
        Some("->"),
        Location { line: 1, col: 11 },
    );
    assert_token(
        &token[5],
        TokenType::Int,
        Some("int"),
        Location { line: 1, col: 14 },
    );
    assert_token(
        &token[6],
        TokenType::LCurly,
        Some("{"),
        Location { line: 1, col: 18 },
    );
    assert_token(
        &token[7],
        TokenType::Let,
        Some("let"),
        Location { line: 2, col: 5 },
    );
    assert_token(
        &token[8],
        TokenType::Id,
        Some("i"),
        Location { line: 2, col: 9 },
    );
    assert_token(
        &token[9],
        TokenType::Colon,
        Some(":"),
        Location { line: 2, col: 10 },
    );
    assert_token(
        &token[10],
        TokenType::Int,
        Some("int"),
        Location { line: 2, col: 12 },
    );
    assert_token(
        &token[11],
        TokenType::Assignment,
        Some(":="),
        Location { line: 2, col: 16 },
    );
    assert_token(
        &token[12],
        TokenType::Num,
        Some("0"),
        Location { line: 2, col: 19 },
    );
    assert_token(
        &token[13],
        TokenType::Semi,
        Some(";"),
        Location { line: 2, col: 20 },
    );
    assert_token(
        &token[14],
        TokenType::While,
        Some("while"),
        Location { line: 3, col: 5 },
    );
    assert_token(
        &token[15],
        TokenType::LParen,
        Some("("),
        Location { line: 3, col: 11 },
    );
    assert_token(
        &token[16],
        TokenType::Num,
        Some("657"),
        Location { line: 3, col: 12 },
    );
    assert_token(
        &token[17],
        TokenType::Minus,
        Some("-"),
        Location { line: 3, col: 16 },
    );
    assert_token(
        &token[18],
        TokenType::Id,
        Some("i"),
        Location { line: 3, col: 18 },
    );
    assert_token(
        &token[19],
        TokenType::Grt,
        Some(">"),
        Location { line: 3, col: 20 },
    );
    assert_token(
        &token[20],
        TokenType::Num,
        Some("0"),
        Location { line: 3, col: 22 },
    );
    assert_token(
        &token[21],
        TokenType::RParen,
        Some(")"),
        Location { line: 3, col: 23 },
    );
    assert_token(
        &token[22],
        TokenType::LCurly,
        Some("{"),
        Location { line: 3, col: 25 },
    );
    assert_token(
        &token[23],
        TokenType::Id,
        Some("i"),
        Location { line: 4, col: 5 },
    );
    assert_token(
        &token[24],
        TokenType::Assignment,
        Some(":="),
        Location { line: 4, col: 7 },
    );
    assert_token(
        &token[25],
        TokenType::Id,
        Some("i"),
        Location { line: 4, col: 10 },
    );
    assert_token(
        &token[26],
        TokenType::Plus,
        Some("+"),
        Location { line: 4, col: 12 },
    );
    assert_token(
        &token[27],
        TokenType::Num,
        Some("1"),
        Location { line: 4, col: 14 },
    );
    assert_token(
        &token[28],
        TokenType::Semi,
        Some(";"),
        Location { line: 4, col: 15 },
    );
    assert_token(
        &token[29],
        TokenType::RCurly,
        Some("}"),
        Location { line: 5, col: 5 },
    );
    assert_token(
        &token[30],
        TokenType::Return,
        Some("return"),
        Location { line: 6, col: 5 },
    );
    assert_token(
        &token[31],
        TokenType::Id,
        Some("i"),
        Location { line: 6, col: 12 },
    );
    assert_token(
        &token[32],
        TokenType::Semi,
        Some(";"),
        Location { line: 6, col: 13 },
    );
    assert_token(
        &token[33],
        TokenType::RCurly,
        Some("}"),
        Location { line: 7, col: 1 },
    );
    assert_token(
        &token[34],
        TokenType::Eof,
        None,
        Location { line: 7, col: 2 },
    );
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

#[test]
fn scan_function_fib() {
    let mut s = Scanner::new(
        "fn fib(n: int) -> int {\n    if (n == 0) {\n        return 0;\n    }\n    elif (n == 1) {\n        return 1;\n    }\n    return fib(n - 1) + fib(n - 2);\n}",
    );
    let token = s.scan().unwrap();

    assert_token(
        &token[0],
        TokenType::Function,
        Some("fn"),
        Location { line: 1, col: 1 },
    );
    assert_token(
        &token[1],
        TokenType::Id,
        Some("fib"),
        Location { line: 1, col: 4 },
    );
    assert_token(
        &token[2],
        TokenType::LParen,
        Some("("),
        Location { line: 1, col: 7 },
    );
    assert_token(
        &token[3],
        TokenType::Id,
        Some("n"),
        Location { line: 1, col: 8 },
    );
    assert_token(
        &token[4],
        TokenType::Colon,
        Some(":"),
        Location { line: 1, col: 9 },
    );
    assert_token(
        &token[5],
        TokenType::Int,
        Some("int"),
        Location { line: 1, col: 11 },
    );
    assert_token(
        &token[6],
        TokenType::RParen,
        Some(")"),
        Location { line: 1, col: 14 },
    );
    assert_token(
        &token[7],
        TokenType::Arrow,
        Some("->"),
        Location { line: 1, col: 16 },
    );
    assert_token(
        &token[8],
        TokenType::Int,
        Some("int"),
        Location { line: 1, col: 19 },
    );
    assert_token(
        &token[9],
        TokenType::LCurly,
        Some("{"),
        Location { line: 1, col: 23 },
    );
    assert_token(
        &token[10],
        TokenType::If,
        Some("if"),
        Location { line: 2, col: 5 },
    );
    assert_token(
        &token[11],
        TokenType::LParen,
        Some("("),
        Location { line: 2, col: 8 },
    );
    assert_token(
        &token[12],
        TokenType::Id,
        Some("n"),
        Location { line: 2, col: 9 },
    );
    assert_token(
        &token[13],
        TokenType::Equality,
        Some("=="),
        Location { line: 2, col: 11 },
    );
    assert_token(
        &token[14],
        TokenType::Num,
        Some("0"),
        Location { line: 2, col: 14 },
    );
    assert_token(
        &token[15],
        TokenType::RParen,
        Some(")"),
        Location { line: 2, col: 15 },
    );
    assert_token(
        &token[16],
        TokenType::LCurly,
        Some("{"),
        Location { line: 2, col: 17 },
    );
    assert_token(
        &token[17],
        TokenType::Return,
        Some("return"),
        Location { line: 3, col: 9 },
    );
    assert_token(
        &token[18],
        TokenType::Num,
        Some("0"),
        Location { line: 3, col: 16 },
    );
    assert_token(
        &token[19],
        TokenType::Semi,
        Some(";"),
        Location { line: 3, col: 17 },
    );
    assert_token(
        &token[20],
        TokenType::RCurly,
        Some("}"),
        Location { line: 4, col: 5 },
    );
    assert_token(
        &token[21],
        TokenType::ElseIf,
        Some("elif"),
        Location { line: 5, col: 5 },
    );
    assert_token(
        &token[22],
        TokenType::LParen,
        Some("("),
        Location { line: 5, col: 10 },
    );
    assert_token(
        &token[23],
        TokenType::Id,
        Some("n"),
        Location { line: 5, col: 11 },
    );
    assert_token(
        &token[24],
        TokenType::Equality,
        Some("=="),
        Location { line: 5, col: 13 },
    );
    assert_token(
        &token[25],
        TokenType::Num,
        Some("1"),
        Location { line: 5, col: 16 },
    );
    assert_token(
        &token[26],
        TokenType::RParen,
        Some(")"),
        Location { line: 5, col: 17 },
    );
    assert_token(
        &token[27],
        TokenType::LCurly,
        Some("{"),
        Location { line: 5, col: 19 },
    );
    assert_token(
        &token[28],
        TokenType::Return,
        Some("return"),
        Location { line: 6, col: 9 },
    );
    assert_token(
        &token[29],
        TokenType::Num,
        Some("1"),
        Location { line: 6, col: 16 },
    );
    assert_token(
        &token[30],
        TokenType::Semi,
        Some(";"),
        Location { line: 6, col: 17 },
    );
    assert_token(
        &token[31],
        TokenType::RCurly,
        Some("}"),
        Location { line: 7, col: 5 },
    );
    assert_token(
        &token[32],
        TokenType::Return,
        Some("return"),
        Location { line: 8, col: 5 },
    );
    assert_token(
        &token[33],
        TokenType::Id,
        Some("fib"),
        Location { line: 8, col: 12 },
    );
    assert_token(
        &token[34],
        TokenType::LParen,
        Some("("),
        Location { line: 8, col: 15 },
    );
    assert_token(
        &token[35],
        TokenType::Id,
        Some("n"),
        Location { line: 8, col: 16 },
    );
    assert_token(
        &token[36],
        TokenType::Minus,
        Some("-"),
        Location { line: 8, col: 18 },
    );
    assert_token(
        &token[37],
        TokenType::Num,
        Some("1"),
        Location { line: 8, col: 20 },
    );
    assert_token(
        &token[38],
        TokenType::RParen,
        Some(")"),
        Location { line: 8, col: 21 },
    );
    assert_token(
        &token[39],
        TokenType::Plus,
        Some("+"),
        Location { line: 8, col: 23 },
    );
    assert_token(
        &token[40],
        TokenType::Id,
        Some("fib"),
        Location { line: 8, col: 25 },
    );
    assert_token(
        &token[41],
        TokenType::LParen,
        Some("("),
        Location { line: 8, col: 28 },
    );
    assert_token(
        &token[42],
        TokenType::Id,
        Some("n"),
        Location { line: 8, col: 29 },
    );
    assert_token(
        &token[43],
        TokenType::Minus,
        Some("-"),
        Location { line: 8, col: 31 },
    );
    assert_token(
        &token[44],
        TokenType::Num,
        Some("2"),
        Location { line: 8, col: 33 },
    );
    assert_token(
        &token[45],
        TokenType::RParen,
        Some(")"),
        Location { line: 8, col: 34 },
    );
    assert_token(
        &token[46],
        TokenType::Semi,
        Some(";"),
        Location { line: 8, col: 35 },
    );
    assert_token(
        &token[47],
        TokenType::RCurly,
        Some("}"),
        Location { line: 9, col: 1 },
    );
    assert_token(
        &token[48],
        TokenType::Eof,
        None,
        Location { line: 9, col: 2 },
    );
}

//
// Input:
// fn fib(n: int) -> int []
//
#[test]
fn scan_bad_function() {
    let mut s = Scanner::new("fn fib(n: int) -> int []");
    let err = s.scan().unwrap_err();
    assert!(matches!(err, LexError::UnexpectedChar(Some('['), ..)));
}

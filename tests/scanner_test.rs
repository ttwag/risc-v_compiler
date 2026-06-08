use risc_v_compiler::{
    scanner::{LexError, Scanner},
    token::{Location, SyntaxToken, TokenType},
};

#[track_caller]
fn assert_token(
    token: &SyntaxToken,
    kind: TokenType,
    value: Option<&str>,
    line: usize,
    col: usize,
) {
    assert_eq!(token.kind, kind);
    assert_eq!(token.value, value);
    assert_eq!(token.span.start.line, line);
    assert_eq!(token.span.start.col, col);
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

    assert_token(&token[0], TokenType::Function, Some("fn"), 1, 1);
    assert_token(&token[1], TokenType::Id, Some("has_arg"), 1, 4);
    assert_token(&token[2], TokenType::LParen, Some("("), 1, 11);
    assert_token(&token[3], TokenType::Id, Some("a"), 1, 12);
    assert_token(&token[4], TokenType::Colon, Some(":"), 1, 13);
    assert_token(&token[5], TokenType::Int, Some("int"), 1, 14);
    assert_token(&token[6], TokenType::RParen, Some(")"), 1, 18);
    assert_token(&token[7], TokenType::Arrow, Some("->"), 1, 20);
    assert_token(&token[8], TokenType::Int, Some("int"), 1, 23);
    assert_token(&token[9], TokenType::LCurly, Some("{"), 1, 27);
    assert_token(&token[10], TokenType::Return, Some("return"), 2, 1);
    assert_token(&token[11], TokenType::Id, Some("a"), 2, 8);
    assert_token(&token[12], TokenType::Semi, Some(";"), 2, 9);
    assert_token(&token[13], TokenType::RCurly, Some("}"), 2, 10);
    assert_token(&token[14], TokenType::Eof, None, 2, 11);
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

    assert_token(&token[0], TokenType::Function, Some("fn"), 1, 1);
    assert_token(&token[1], TokenType::Id, Some("assignment"), 1, 4);
    assert_token(&token[2], TokenType::LParen, Some("("), 1, 14);
    assert_token(&token[3], TokenType::RParen, Some(")"), 1, 15);
    assert_token(&token[4], TokenType::Arrow, Some("->"), 1, 17);
    assert_token(&token[5], TokenType::Int, Some("int"), 1, 20);
    assert_token(&token[6], TokenType::LCurly, Some("{"), 1, 24);
    assert_token(&token[7], TokenType::Let, Some("let"), 2, 2);
    assert_token(&token[8], TokenType::Id, Some("a"), 2, 6);
    assert_token(&token[9], TokenType::Colon, Some(":"), 2, 7);
    assert_token(&token[10], TokenType::Int, Some("int"), 2, 9);
    assert_token(&token[11], TokenType::Assignment, Some(":="), 2, 13);
    assert_token(&token[12], TokenType::Num, Some("100"), 2, 16);
    assert_token(&token[13], TokenType::Semi, Some(";"), 2, 19);
    assert_token(&token[14], TokenType::Return, Some("return"), 3, 2);
    assert_token(&token[15], TokenType::Id, Some("a"), 3, 9);
    assert_token(&token[16], TokenType::Semi, Some(";"), 3, 10);
    assert_token(&token[17], TokenType::RCurly, Some("}"), 4, 2);
    assert_token(&token[18], TokenType::Eof, None, 4, 3);
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
    assert_token(&token[0], TokenType::Function, Some("fn"), 1, 1);
    assert_token(&token[1], TokenType::Id, Some("branch"), 1, 4);
    assert_token(&token[2], TokenType::LParen, Some("("), 1, 10);
    assert_token(&token[3], TokenType::Id, Some("a"), 1, 11);
    assert_token(&token[4], TokenType::Colon, Some(":"), 1, 12);
    assert_token(&token[5], TokenType::Int, Some("int"), 1, 14);
    assert_token(&token[6], TokenType::RParen, Some(")"), 1, 17);
    assert_token(&token[7], TokenType::Arrow, Some("->"), 1, 19);
    assert_token(&token[8], TokenType::Int, Some("int"), 1, 22);
    assert_token(&token[9], TokenType::LCurly, Some("{"), 1, 26);
    assert_token(&token[10], TokenType::If, Some("if"), 2, 5);
    assert_token(&token[11], TokenType::LParen, Some("("), 2, 7);
    assert_token(&token[12], TokenType::Id, Some("a"), 2, 8);
    assert_token(&token[13], TokenType::Grt, Some(">"), 2, 10);
    assert_token(&token[14], TokenType::Num, Some("1"), 2, 12);
    assert_token(&token[15], TokenType::RParen, Some(")"), 2, 13);
    assert_token(&token[16], TokenType::LCurly, Some("{"), 2, 15);
    assert_token(&token[17], TokenType::Return, Some("return"), 3, 9);
    assert_token(&token[18], TokenType::Num, Some("0"), 3, 16);
    assert_token(&token[19], TokenType::Semi, Some(";"), 3, 17);
    assert_token(&token[20], TokenType::RCurly, Some("}"), 4, 1);
    assert_token(&token[21], TokenType::Return, Some("return"), 5, 5);
    assert_token(&token[22], TokenType::Num, Some("5"), 5, 12);
    assert_token(&token[23], TokenType::Semi, Some(";"), 5, 13);
    assert_token(&token[24], TokenType::RCurly, Some("}"), 6, 1);
    assert_token(&token[25], TokenType::Eof, None, 6, 2);
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

    assert_token(&token[0], TokenType::Function, Some("fn"), 1, 1);
    assert_token(&token[1], TokenType::Id, Some("loop"), 1, 4);
    assert_token(&token[2], TokenType::LParen, Some("("), 1, 8);
    assert_token(&token[3], TokenType::RParen, Some(")"), 1, 9);
    assert_token(&token[4], TokenType::Arrow, Some("->"), 1, 11);
    assert_token(&token[5], TokenType::Int, Some("int"), 1, 14);
    assert_token(&token[6], TokenType::LCurly, Some("{"), 1, 18);
    assert_token(&token[7], TokenType::Let, Some("let"), 2, 5);
    assert_token(&token[8], TokenType::Id, Some("i"), 2, 9);
    assert_token(&token[9], TokenType::Colon, Some(":"), 2, 10);
    assert_token(&token[10], TokenType::Int, Some("int"), 2, 12);
    assert_token(&token[11], TokenType::Assignment, Some(":="), 2, 16);
    assert_token(&token[12], TokenType::Num, Some("0"), 2, 19);
    assert_token(&token[13], TokenType::Semi, Some(";"), 2, 20);
    assert_token(&token[14], TokenType::While, Some("while"), 3, 5);
    assert_token(&token[15], TokenType::LParen, Some("("), 3, 11);
    assert_token(&token[16], TokenType::Num, Some("657"), 3, 12);
    assert_token(&token[17], TokenType::Minus, Some("-"), 3, 16);
    assert_token(&token[18], TokenType::Id, Some("i"), 3, 18);
    assert_token(&token[19], TokenType::Grt, Some(">"), 3, 20);
    assert_token(&token[20], TokenType::Num, Some("0"), 3, 22);
    assert_token(&token[21], TokenType::RParen, Some(")"), 3, 23);
    assert_token(&token[22], TokenType::LCurly, Some("{"), 3, 25);
    assert_token(&token[23], TokenType::Id, Some("i"), 4, 5);
    assert_token(&token[24], TokenType::Assignment, Some(":="), 4, 7);
    assert_token(&token[25], TokenType::Id, Some("i"), 4, 10);
    assert_token(&token[26], TokenType::Plus, Some("+"), 4, 12);
    assert_token(&token[27], TokenType::Num, Some("1"), 4, 14);
    assert_token(&token[28], TokenType::Semi, Some(";"), 4, 15);
    assert_token(&token[29], TokenType::RCurly, Some("}"), 5, 5);
    assert_token(&token[30], TokenType::Return, Some("return"), 6, 5);
    assert_token(&token[31], TokenType::Id, Some("i"), 6, 12);
    assert_token(&token[32], TokenType::Semi, Some(";"), 6, 13);
    assert_token(&token[33], TokenType::RCurly, Some("}"), 7, 1);
    assert_token(&token[34], TokenType::Eof, None, 7, 2);
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

    assert_token(&token[0], TokenType::Function, Some("fn"), 1, 1);
    assert_token(&token[1], TokenType::Id, Some("fib"), 1, 4);
    assert_token(&token[2], TokenType::LParen, Some("("), 1, 7);
    assert_token(&token[3], TokenType::Id, Some("n"), 1, 8);
    assert_token(&token[4], TokenType::Colon, Some(":"), 1, 9);
    assert_token(&token[5], TokenType::Int, Some("int"), 1, 11);
    assert_token(&token[6], TokenType::RParen, Some(")"), 1, 14);
    assert_token(&token[7], TokenType::Arrow, Some("->"), 1, 16);
    assert_token(&token[8], TokenType::Int, Some("int"), 1, 19);
    assert_token(&token[9], TokenType::LCurly, Some("{"), 1, 23);
    assert_token(&token[10], TokenType::If, Some("if"), 2, 5);
    assert_token(&token[11], TokenType::LParen, Some("("), 2, 8);
    assert_token(&token[12], TokenType::Id, Some("n"), 2, 9);
    assert_token(&token[13], TokenType::Equality, Some("=="), 2, 11);
    assert_token(&token[14], TokenType::Num, Some("0"), 2, 14);
    assert_token(&token[15], TokenType::RParen, Some(")"), 2, 15);
    assert_token(&token[16], TokenType::LCurly, Some("{"), 2, 17);
    assert_token(&token[17], TokenType::Return, Some("return"), 3, 9);
    assert_token(&token[18], TokenType::Num, Some("0"), 3, 16);
    assert_token(&token[19], TokenType::Semi, Some(";"), 3, 17);
    assert_token(&token[20], TokenType::RCurly, Some("}"), 4, 5);
    assert_token(&token[21], TokenType::ElseIf, Some("elif"), 5, 5);
    assert_token(&token[22], TokenType::LParen, Some("("), 5, 10);
    assert_token(&token[23], TokenType::Id, Some("n"), 5, 11);
    assert_token(&token[24], TokenType::Equality, Some("=="), 5, 13);
    assert_token(&token[25], TokenType::Num, Some("1"), 5, 16);
    assert_token(&token[26], TokenType::RParen, Some(")"), 5, 17);
    assert_token(&token[27], TokenType::LCurly, Some("{"), 5, 19);
    assert_token(&token[28], TokenType::Return, Some("return"), 6, 9);
    assert_token(&token[29], TokenType::Num, Some("1"), 6, 16);
    assert_token(&token[30], TokenType::Semi, Some(";"), 6, 17);
    assert_token(&token[31], TokenType::RCurly, Some("}"), 7, 5);
    assert_token(&token[32], TokenType::Return, Some("return"), 8, 5);
    assert_token(&token[33], TokenType::Id, Some("fib"), 8, 12);
    assert_token(&token[34], TokenType::LParen, Some("("), 8, 15);
    assert_token(&token[35], TokenType::Id, Some("n"), 8, 16);
    assert_token(&token[36], TokenType::Minus, Some("-"), 8, 18);
    assert_token(&token[37], TokenType::Num, Some("1"), 8, 20);
    assert_token(&token[38], TokenType::RParen, Some(")"), 8, 21);
    assert_token(&token[39], TokenType::Plus, Some("+"), 8, 23);
    assert_token(&token[40], TokenType::Id, Some("fib"), 8, 25);
    assert_token(&token[41], TokenType::LParen, Some("("), 8, 28);
    assert_token(&token[42], TokenType::Id, Some("n"), 8, 29);
    assert_token(&token[43], TokenType::Minus, Some("-"), 8, 31);
    assert_token(&token[44], TokenType::Num, Some("2"), 8, 33);
    assert_token(&token[45], TokenType::RParen, Some(")"), 8, 34);
    assert_token(&token[46], TokenType::Semi, Some(";"), 8, 35);
    assert_token(&token[47], TokenType::RCurly, Some("}"), 9, 1);
    assert_token(&token[48], TokenType::Eof, None, 9, 2);
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

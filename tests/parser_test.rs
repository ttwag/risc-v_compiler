use risc_v_compiler::{
    ast::*,
    parser::{ParseError, Parser},
    scanner::{ScanError, Scanner},
    token::{Location, Span, SyntaxToken, Token},
};

use indoc::indoc;
use pretty_assertions::assert_eq;

#[track_caller]
fn assert_ast(input: &str, expected_ast: Program) {
    let mut s = Scanner::new(input);
    let sts = s.scan().unwrap();
    let mut p = Parser::new(&sts);
    let ast = p.parse().unwrap();
    assert_eq!(expected_ast, ast);
}

#[test]
fn parse_simple_ret() {
    let input = "fn example() -> int { return 0; }";
}

#[test]
fn parse_assign_stmt() {
    let input = indoc! {"
    fn example() -> int {
        let a: int := 5;
        a := a + 1;
        return a > 6;
    }
    "};
}

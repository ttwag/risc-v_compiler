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
    let expected_ast = Program(
        [FuncDef {
            name: Id {
                st: SyntaxToken {
                    token: Token::Id,
                    span: Span {
                        start: Location {
                            index: 3,
                            line: 1,
                            col: 4,
                        },
                        end: Location {
                            index: 10,
                            line: 1,
                            col: 11,
                        },
                    },
                },
            },
            params: vec![],
            ret: Type::Int,
            body: vec![],
            ret_stmt: ReturnStmt(CompExpr(
                ArithExpr(
                    AtomExpr::Num(Num {
                        st: SyntaxToken {
                            token: Token::Num,
                            span: Span {
                                start: Location {
                                    index: 29,
                                    line: 1,
                                    col: 30,
                                },
                                end: Location {
                                    index: 30,
                                    line: 1,
                                    col: 31,
                                },
                            },
                        },
                    }),
                    vec![],
                ),
                None,
            )),
        }]
        .to_vec(),
    );
    assert_ast(input, expected_ast);
}


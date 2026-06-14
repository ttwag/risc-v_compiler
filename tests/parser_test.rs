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

#[test]
fn parse_assign_stmt() {
    let input = indoc! {"
    fn example() -> int {
        let a: int := 5;
        a := a + 1;
        return a > 6;
    }
    "};
    let expected_ast = Program(vec![FuncDef {
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
        body: vec![
            Stmt::Let(
                Id {
                    st: SyntaxToken {
                        token: Token::Id,
                        span: Span {
                            start: Location {
                                index: 30,
                                line: 2,
                                col: 9,
                            },
                            end: Location {
                                index: 31,
                                line: 2,
                                col: 10,
                            },
                        },
                    },
                },
                Type::Int,
                CompExpr(
                    ArithExpr(
                        AtomExpr::Num(Num {
                            st: SyntaxToken {
                                token: Token::Num,
                                span: Span {
                                    start: Location {
                                        index: 40,
                                        line: 2,
                                        col: 19,
                                    },
                                    end: Location {
                                        index: 41,
                                        line: 2,
                                        col: 20,
                                    },
                                },
                            },
                        }),
                        vec![],
                    ),
                    None,
                ),
            ),
            Stmt::Assign(
                Id {
                    st: SyntaxToken {
                        token: Token::Id,
                        span: Span {
                            start: Location {
                                index: 47,
                                line: 3,
                                col: 5,
                            },
                            end: Location {
                                index: 48,
                                line: 3,
                                col: 6,
                            },
                        },
                    },
                },
                CompExpr(
                    ArithExpr(
                        AtomExpr::Id(Id {
                            st: SyntaxToken {
                                token: Token::Id,
                                span: Span {
                                    start: Location {
                                        index: 52,
                                        line: 3,
                                        col: 10,
                                    },
                                    end: Location {
                                        index: 53,
                                        line: 3,
                                        col: 11,
                                    },
                                },
                            },
                        }),
                        vec![(
                            ArithOp::Plus,
                            AtomExpr::Num(Num {
                                st: SyntaxToken {
                                    token: Token::Num,
                                    span: Span {
                                        start: Location {
                                            index: 56,
                                            line: 3,
                                            col: 14,
                                        },
                                        end: Location {
                                            index: 57,
                                            line: 3,
                                            col: 15,
                                        },
                                    },
                                },
                            }),
                        )],
                    ),
                    None,
                ),
            ),
        ],
        ret_stmt: ReturnStmt(CompExpr(
            ArithExpr(
                AtomExpr::Id(Id {
                    st: SyntaxToken {
                        token: Token::Id,
                        span: Span {
                            start: Location {
                                index: 70,
                                line: 4,
                                col: 12,
                            },
                            end: Location {
                                index: 71,
                                line: 4,
                                col: 13,
                            },
                        },
                    },
                }),
                vec![],
            ),
            Some((
                CompOp::Grt,
                ArithExpr(
                    AtomExpr::Num(Num {
                        st: SyntaxToken {
                            token: Token::Num,
                            span: Span {
                                start: Location {
                                    index: 74,
                                    line: 4,
                                    col: 16,
                                },
                                end: Location {
                                    index: 75,
                                    line: 4,
                                    col: 17,
                                },
                            },
                        },
                    }),
                    vec![],
                ),
            )),
        )),
    }]);
    assert_ast(input, expected_ast);
}

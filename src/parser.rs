use crate::ast::{
    ArithExpr, ArithOp, AtomExpr, Branch, CompExpr, CompOp, Expr, FuncDef, Id, Num, Param, Program,
    ReturnStmt, Stmt, Type,
};
use crate::token::{SyntaxToken, Token};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(SyntaxToken),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(st) => write!(f, "unexpected token {st}"),
        }
    }
}

impl std::error::Error for ParseError {}

pub struct Parser<'a> {
    index: usize,
    sts: &'a [SyntaxToken],
}

impl<'a> Parser<'a> {
    /// Creates a new parser over a syntax token slice.
    ///
    /// # Panics
    /// Panics if `sts` does not end with `Token::Eof` or have more than one `Token::Eof`.
    pub fn new(sts: &'a [SyntaxToken]) -> Self {
        assert!(
            matches!(sts.last(), Some(t) if t.token == Token::Eof),
            "syntax tokens must end with EOF"
        );
        assert!(
            sts.iter().filter(|t| t.token == Token::Eof).count() == 1,
            "syntax tokens must only have one EOF"
        );
        Self { index: 0, sts }
    }

    fn peek(&self) -> &SyntaxToken {
        self.sts
            .get(self.index)
            .unwrap_or_else(|| self.sts.last().unwrap())
    }

    fn peek_next(&self) -> &SyntaxToken {
        self.sts
            .get(self.index + 1)
            .unwrap_or_else(|| self.sts.last().unwrap())
    }

    fn advance(&mut self) -> &SyntaxToken {
        let curr = self
            .sts
            .get(self.index)
            .expect("advance: parser index out of bounds");
        match curr.token {
            Token::Eof => {}
            _ => {
                self.index += 1;
            }
        }
        curr
    }

    fn expect(&mut self, token: Token) -> Result<&SyntaxToken, ParseError> {
        assert!(token != Token::Eof, "expect: cannot expect Eof");
        let st = self
            .sts
            .get(self.index)
            .expect("expect: parser index out of bounds");
        if st.token == token {
            self.index += 1;
            Ok(st)
        } else {
            Err(ParseError::UnexpectedToken(st.clone()))
        }
    }

    /// Parse the src syntax tokens and returns an Abstract Syntax Tree
    /// The grammar is defined in `grammar.ebnf`.
    ///
    /// # Errors
    ///
    /// Parsing stops and return an error when hitting unexpected Eof or invalid token
    ///
    /// # Examples
    /// ```
    /// use risc_v_compiler::scanner::Scanner;
    /// use risc_v_compiler::parser::Parser;
    /// let src = "fn main() -> int { return 0; }";
    /// let sts = Scanner::new(src).scan().unwrap();
    /// let ast = Parser::new(&sts).parse().unwrap();
    /// ```
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut func_defs = Vec::new();
        func_defs.push(self.parse_func_def()?);
        while self.peek().token != Token::Eof {
            func_defs.push(self.parse_func_def()?);
        }
        Ok(Program(func_defs))
    }

    // ── Function Definition ──────────────────────────────────────────────────────────────────
    fn parse_func_def(&mut self) -> Result<FuncDef, ParseError> {
        self.expect(Token::Function)?;
        let name = self.parse_id()?;
        let params = self.parse_param_list()?;

        self.expect(Token::Arrow)?;
        let ret = self.parse_type()?;

        let mut body = Vec::new();
        self.expect(Token::LCurly)?;
        while self.peek().token != Token::Return {
            body.push(self.parse_stmt()?);
        }
        let ret_stmt = self.parse_return_stmt()?;
        self.expect(Token::RCurly)?;

        Ok(FuncDef {
            name,
            params,
            ret,
            body,
            ret_stmt,
        })
    }

    fn parse_param_list(&mut self) -> Result<Vec<Param>, ParseError> {
        self.expect(Token::LParen)?;
        let mut params = Vec::new();

        if self.peek().token != Token::RParen {
            params.push(self.parse_param()?);
            while let Ok(_) = self.expect(Token::Comma) {
                params.push(self.parse_param()?);
            }
        }
        self.expect(Token::RParen)?;
        Ok(params)
    }

    fn parse_param(&mut self) -> Result<Param, ParseError> {
        let var = self.parse_id()?;
        self.expect(Token::Colon)?;
        let param_type = self.parse_type()?;
        Ok(Param(var, param_type))
    }

    // ── Statements ──────────────────────────────────────────────────────────────────
    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let st = self.peek();
        match st.token {
            Token::Id(_) => self.parse_assign_stmt(),
            Token::Let => self.parse_let_stmt(),
            Token::If => self.parse_if_stmt(),
            Token::While => self.parse_while_stmt(),
            _ => Err(ParseError::UnexpectedToken(st.clone())),
        }
    }

    fn parse_assign_stmt(&mut self) -> Result<Stmt, ParseError> {
        let var = self.parse_id()?;
        self.expect(Token::Assignment)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semi)?;
        Ok(Stmt::Assign(var, expr))
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(Token::Let)?;
        let var = self.parse_id()?;
        self.expect(Token::Colon)?;
        let let_type = self.parse_type()?;
        self.expect(Token::Assignment)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semi)?;
        Ok(Stmt::Let(var, let_type, expr))
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(Token::If)?;
        let if_branch = self.parse_branch()?;
        let mut elif_branch = Vec::new();
        let mut else_stmts = None;

        while self.peek().token == Token::ElseIf {
            self.advance();
            elif_branch.push(self.parse_branch()?);
        }
        if self.peek().token == Token::Else {
            self.advance();
            else_stmts = Some(self.parse_stmt_block()?);
        }
        Ok(Stmt::If(if_branch, elif_branch, else_stmts))
    }

    fn parse_branch(&mut self) -> Result<Branch, ParseError> {
        self.expect(Token::LParen)?;
        let expr = self.parse_expr()?;
        self.expect(Token::RParen)?;
        let stmts = self.parse_stmt_block()?;
        Ok(Branch(expr, stmts))
    }

    fn parse_stmt_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.expect(Token::LCurly)?;
        let mut stmts = Vec::new();

        while self.peek().token != Token::RCurly {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(Token::RCurly)?;
        Ok(stmts)
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(Token::While)?;
        Ok(Stmt::While(self.parse_branch()?))
    }

    fn parse_return_stmt(&mut self) -> Result<ReturnStmt, ParseError> {
        self.expect(Token::Return)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semi)?;
        Ok(ReturnStmt(expr))
    }

    // ── Expressions ──────────────────────────────────────────────────────────────────
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let arith_expr_left = self.parse_arith_expr()?;
        let Ok(op) = self.parse_comp_op() else {
            return Ok(CompExpr(arith_expr_left, None));
        };
        let arith_expr_right = self.parse_arith_expr()?;
        Ok(CompExpr(arith_expr_left, Some((op, arith_expr_right))))
    }

    fn parse_comp_op(&mut self) -> Result<CompOp, ParseError> {
        let st = self.peek();
        match st.token {
            Token::Equality => {
                self.advance();
                Ok(CompOp::Equality)
            }
            Token::Grt => {
                self.advance();
                Ok(CompOp::Grt)
            }
            _ => Err(ParseError::UnexpectedToken(st.clone())),
        }
    }

    fn parse_arith_expr(&mut self) -> Result<ArithExpr, ParseError> {
        let atom_expr_left = self.parse_atom_expr()?;
        let mut op_exprs = Vec::new();
        while let Ok(op) = self.parse_arith_op() {
            let atom_expr_right = self.parse_atom_expr()?;
            op_exprs.push((op, atom_expr_right));
        }
        Ok(ArithExpr(atom_expr_left, op_exprs))
    }

    fn parse_arith_op(&mut self) -> Result<ArithOp, ParseError> {
        let st = self.peek();
        match st.token {
            Token::Plus => {
                self.advance();
                Ok(ArithOp::Plus)
            }
            Token::Minus => {
                self.advance();
                Ok(ArithOp::Minus)
            }
            _ => Err(ParseError::UnexpectedToken(st.clone())),
        }
    }

    fn parse_atom_expr(&mut self) -> Result<AtomExpr, ParseError> {
        let st = self.peek();
        let token = &st.token;
        let next_token = &self.peek_next().token;
        match (token, next_token) {
            (Token::Id(_), Token::LParen) => self.parse_func_call(),
            (Token::Id(_), _) => Ok(AtomExpr::Id(self.parse_id()?)),
            (Token::Num(_), _) => Ok(AtomExpr::Num(self.parse_num()?)),
            (Token::LParen, _) => self.parse_group(),
            _ => Err(ParseError::UnexpectedToken(st.clone())),
        }
    }

    fn parse_group(&mut self) -> Result<AtomExpr, ParseError> {
        self.expect(Token::LParen)?;
        let expr = self.parse_expr()?;
        self.expect(Token::RParen)?;
        Ok(AtomExpr::Group(Box::new(expr)))
    }

    fn parse_func_call(&mut self) -> Result<AtomExpr, ParseError> {
        let func = self.parse_id()?;
        self.expect(Token::LParen)?;
        let mut args = Vec::new();

        if self.peek().token != Token::RParen {
            args.push(self.parse_expr()?);
            while let Ok(_) = self.expect(Token::Comma) {
                args.push(self.parse_expr()?);
            }
        }
        self.expect(Token::RParen)?;
        Ok(AtomExpr::Call(func, args))
    }

    // ── Type ──────────────────────────────────────────────────────────────────
    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let st = self.peek();
        match st.token {
            Token::Int => {
                self.advance();
                Ok(Type::Int)
            }
            _ => Err(ParseError::UnexpectedToken(st.clone())),
        }
    }

    // ── Terminals ──────────────────────────────────────────────────────────────────
    fn parse_id(&mut self) -> Result<Id, ParseError> {
        let st = self.peek();
        match &st.token {
            Token::Id(s) => {
                let id = Id {
                    st: st.clone(),
                    name: String::from(s),
                };
                self.advance();
                Ok(id)
            }
            _ => Err(ParseError::UnexpectedToken(st.clone())),
        }
    }

    fn parse_num(&mut self) -> Result<Num, ParseError> {
        let st = self.peek();
        match &st.token {
            Token::Num(s) => {
                let num = Num {
                    st: st.clone(),
                    name: String::from(s),
                };
                self.advance();
                Ok(num)
            }
            _ => Err(ParseError::UnexpectedToken(st.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Span;

    // ── new ──────────────────────────────────────────────────────────────────
    #[test]
    #[should_panic = "syntax tokens must end with EOF"]
    fn new_sts_without_eof() {
        let sts = [];
        let _ = Parser::new(&sts);
    }

    #[test]
    #[should_panic = "syntax tokens must only have one EOF"]
    fn new_sts_with_multiple_eof() {
        let sts = [
            SyntaxToken {
                token: Token::Comma,
                span: Span::default(),
            },
            SyntaxToken {
                token: Token::Eof,
                span: Span::default(),
            },
            SyntaxToken {
                token: Token::Comma,
                span: Span::default(),
            },
            SyntaxToken {
                token: Token::Eof,
                span: Span::default(),
            },
        ];
        let _ = Parser::new(&sts);
    }

    // ── peek ──────────────────────────────────────────────────────────────────
    #[test]
    fn peek_return_correct_token() {
        let sts = [SyntaxToken {
            token: Token::Eof,
            span: Span::default(),
        }];
        let p = Parser::new(&sts);

        assert_eq!(p.peek().token, Token::Eof);
    }

    #[test]
    fn peek_out_of_bound_return_eof() {
        let sts = [SyntaxToken {
            token: Token::Eof,
            span: Span::default(),
        }];
        let mut p = Parser::new(&sts);
        p.index += 1;
        assert_eq!(p.peek().token, Token::Eof);
    }

    // ── advance ──────────────────────────────────────────────────────────────────
    #[test]
    fn advance_pass_token() {
        let sts = [
            SyntaxToken {
                token: Token::RCurly,
                span: Span::default(),
            },
            SyntaxToken {
                token: Token::Eof,
                span: Span::default(),
            },
        ];
        let mut p = Parser::new(&sts);
        assert_eq!(p.advance().token, Token::RCurly);
        assert_eq!(p.index, 1);
    }

    #[test]
    fn advance_not_pass_eof() {
        let sts = [SyntaxToken {
            token: Token::Eof,
            span: Span::default(),
        }];
        let mut p = Parser::new(&sts);
        assert_eq!(p.advance().token, Token::Eof);
        assert_eq!(p.index, 0);
    }

    // ── expect ──────────────────────────────────────────────────────────────────
    #[test]
    #[should_panic(expected = "expect: cannot expect Eof")]
    fn expect_eof() {
        let sts = [SyntaxToken {
            token: Token::Eof,
            span: Span::default(),
        }];
        let mut p = Parser::new(&sts);
        let _ = p.expect(Token::Eof);
    }

    #[test]
    fn expect_token_return_token_and_advance() {
        let sts = [
            SyntaxToken {
                token: Token::Arrow,
                span: Span::default(),
            },
            SyntaxToken {
                token: Token::Eof,
                span: Span::default(),
            },
        ];
        let mut p = Parser::new(&sts);
        let result = p.expect(Token::Arrow).unwrap();
        assert_eq!(*result, sts[0]);
        assert_eq!(p.index, 1)
    }

    #[test]
    fn expect_mismatch_token_return_unexpected_token() {
        let sts = [
            SyntaxToken {
                token: Token::Arrow,
                span: Span::default(),
            },
            SyntaxToken {
                token: Token::Eof,
                span: Span::default(),
            },
        ];
        let mut p = Parser::new(&sts);
        let result = p.expect(Token::LParen).unwrap_err();
        assert!(matches!(result, ParseError::UnexpectedToken(..)));
        assert_eq!(p.index, 0);
    }

    #[test]
    fn expect_mismatch_token_return_unexpected_eof() {
        let sts = [SyntaxToken {
            token: Token::Eof,
            span: Span::default(),
        }];
        let mut p = Parser::new(&sts);
        let result = p.expect(Token::LParen).unwrap_err();
        assert!(matches!(result, ParseError::UnexpectedToken(..)));
        assert_eq!(p.index, 0);
    }
}

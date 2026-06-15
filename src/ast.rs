use crate::token::SyntaxToken;
use core::fmt;
use std::fmt::Display;

// ── AST Definition ───────────────────────────────────────────────────────────────
#[derive(Debug, PartialEq, Clone)]
pub struct Program(pub Vec<FuncDef>);

// ── Function Definition ───────────────────────────────────────────────────────────────
#[derive(Debug, PartialEq, Clone)]
pub struct FuncDef {
    pub name: Id,
    pub params: Vec<Param>,
    pub ret: Type,
    pub body: Vec<Stmt>,
    pub ret_stmt: ReturnStmt,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param(pub Id, pub Type);

// ── Statement ───────────────────────────────────────────────────────────────
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Assign(Id, Expr),
    Let(Id, Type, Expr),
    If(Branch, Vec<Branch>, Option<Vec<Stmt>>),
    While(Branch),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Branch(pub Expr, pub Vec<Stmt>);

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt(pub Expr);

// ── Expression ───────────────────────────────────────────────────────────────
pub type Expr = CompExpr;

#[derive(Debug, PartialEq, Clone)]
pub enum CompOp {
    Grt,
    Equality,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompExpr(pub ArithExpr, pub Option<(CompOp, ArithExpr)>);

#[derive(Debug, PartialEq, Clone)]
pub enum ArithOp {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArithExpr(pub AtomExpr, pub Vec<(ArithOp, AtomExpr)>);

#[derive(Debug, PartialEq, Clone)]
pub enum AtomExpr {
    Id(Id),
    Num(Num),
    Group(Box<Expr>),
    Call(Id, Vec<Expr>),
}

// ── Type ───────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
}

// ── Terminals ───────────────────────────────────────────────────────────────
#[derive(Debug, PartialEq, Clone)]
pub struct Id {
    pub st: SyntaxToken,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Num {
    pub st: SyntaxToken,
    pub name: String,
}

// ── Display Implementation ───────────────────────────────────────────────────────────────
impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let func_defs: Vec<String> = self.0.iter().map(|e| e.to_string()).collect();
        write!(f, "{}", func_defs.join(" "))
    }
}

impl Display for FuncDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let params: Vec<String> = self.params.iter().map(|e| e.to_string()).collect();
        let stmts: Vec<String> = self.body.iter().map(|e| e.to_string()).collect();
        let params_str = format_list_str(params, " , ");
        let stmts_str = format_list_str(stmts, " ");
        write!(
            f,
            "fn {} ({}) -> {} {{{}{} }}",
            self.name, params_str, self.ret, stmts_str, self.ret_stmt
        )
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {}", self.0, self.1)
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Assign(var, expr) => write!(f, "{} := {} ;", var, expr),
            Stmt::Let(var, kind, expr) => write!(f, "let {} : {} := {} ;", var, kind, expr),
            Stmt::If(if_branch, elif_branch, else_branch) => {
                let else_ifs: Vec<String> =
                    elif_branch.iter().map(|b| format!(" elif {}", b)).collect();
                write!(f, "if {}{}", if_branch, else_ifs.join(" "))?;
                if let Some(stmts) = else_branch {
                    let stmts = stmts.iter().map(|e| e.to_string()).collect::<Vec<_>>();
                    let stmts_str = format_list_str(stmts, " ");
                    write!(f, " else {{{}}}", stmts_str)?;
                }
                Ok(())
            }
            Stmt::While(branch) => write!(f, "while {}", branch),
        }
    }
}

impl Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stmts: Vec<String> = self.1.iter().map(|e| e.to_string()).collect();
        let stmts_str = format_list_str(stmts, " ");
        write!(f, "( {} ) {{{}}}", self.0, stmts_str)
    }
}

impl Display for ReturnStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "return {} ;", self.0)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        if let Some((op, expr)) = &self.1 {
            write!(f, " {} {}", op, expr)?;
        }
        Ok(())
    }
}

impl Display for CompOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompOp::Grt => write!(f, ">"),
            CompOp::Equality => write!(f, "=="),
        }
    }
}

impl Display for ArithExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        for (op, expr) in &self.1 {
            write!(f, " {} {}", op, expr)?;
        }
        Ok(())
    }
}

impl Display for ArithOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithOp::Plus => write!(f, "+"),
            ArithOp::Minus => write!(f, "-"),
        }
    }
}

impl Display for AtomExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AtomExpr::Id(id) => write!(f, "{}", id),
            AtomExpr::Num(num) => write!(f, "{}", num),
            AtomExpr::Group(exprs) => write!(f, "( {} )", exprs),
            AtomExpr::Call(name, exprs) => {
                let args: Vec<String> = exprs.iter().map(|e| e.to_string()).collect();
                let args_str = format_list_str(args, " , ");
                write!(f, "{} ({})", name, args_str)
            }
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn format_list_str(strs: Vec<String>, del: &str) -> String {
    if strs.is_empty() {
        String::from(" ")
    } else {
        format!(" {} ", strs.join(del))
    }
}

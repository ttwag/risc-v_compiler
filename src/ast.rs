use crate::token::SyntaxToken;

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
    While(Expr, Vec<Stmt>),
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct Num {
    pub st: SyntaxToken,
}

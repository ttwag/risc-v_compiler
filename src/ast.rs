use crate::token::SyntaxToken;

// ── AST Definition ───────────────────────────────────────────────────────────────
pub struct Program(pub Vec<FuncDef>);

// ── Function Definition ───────────────────────────────────────────────────────────────
pub struct FuncDef {
    pub name: Id,
    pub params: Vec<Param>,
    pub ret: Type,
    pub body: Vec<Stmt>,
    pub ret_stmt: ReturnStmt,
}
pub struct Param(pub Id, pub Type);

// ── Statement ───────────────────────────────────────────────────────────────
pub enum Stmt {
    Assign(Id, Expr),
    Let(Id, Type, Expr),
    If(Branch, Vec<Branch>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
}

pub struct Branch(pub Expr, pub Vec<Stmt>);

pub struct ReturnStmt(pub Expr);

// ── Expression ───────────────────────────────────────────────────────────────
pub type Expr = CompExpr;
pub enum CompOp {
    Grt,
    Equality,
}
pub struct CompExpr(pub ArithExpr, pub Option<(CompOp, ArithExpr)>);

pub enum ArithOp {
    Plus,
    Minus,
}
pub struct ArithExpr(pub AtomExpr, pub Vec<(ArithOp, AtomExpr)>);

pub enum AtomExpr {
    Id(Id),
    Num(Num),
    Group(Box<Expr>),
    Call(Id, Vec<Expr>),
}

// ── Type ───────────────────────────────────────────────────────────────
pub enum Type {
    Int,
}

// ── Terminals ───────────────────────────────────────────────────────────────
pub struct Id {
    pub st: SyntaxToken,
}

pub struct Num {
    pub st: SyntaxToken,
}

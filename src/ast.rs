use crate::token::SyntaxToken;

// ── AST Definition ───────────────────────────────────────────────────────────────
struct Program(Vec<FuncDef>);

// ── Function Definition ───────────────────────────────────────────────────────────────
struct FuncDef {
    name: Id,
    params: Vec<Param>,
    ret: Type,
    body: Vec<Stmt>,
    ret_stmt: ReturnStmt,
}
struct Param(Id, Type);

// ── Statement ───────────────────────────────────────────────────────────────
enum Stmt {
    Assign(Id, Expr),
    Let(Id, Type, Expr),
    If(Branch, Vec<Branch>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
}

struct Branch(Expr, Vec<Stmt>);

struct ReturnStmt(Expr);

// ── Expression ───────────────────────────────────────────────────────────────
type Expr = CompExpr;
enum CompOp {
    Grt,
    Equality,
}
struct CompExpr(ArithExpr, Option<(CompOp, ArithExpr)>);

enum ArithOp {
    Plus,
    Minus,
}
struct ArithExpr(AtomExpr, Vec<(ArithOp, AtomExpr)>);

enum AtomExpr {
    Id(Id),
    Num(Num),
    Group(Box<Expr>),
    Call(Id, Vec<Expr>),
}

// ── Type ───────────────────────────────────────────────────────────────
enum Type {
    Int,
}

// ── Terminals ───────────────────────────────────────────────────────────────
struct Id {
    st: SyntaxToken,
}
struct Num {
    st: SyntaxToken,
}

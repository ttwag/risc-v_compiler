use crate::ast::{ArithExpr, AtomExpr, CompExpr, Expr, Num, Program};
use crate::token::{Span, SyntaxToken, Token};
use core::fmt;
use std::collections::HashMap;
use std::fmt::Display;

enum Reg {
    // stack pointer
    Sp,
    // frame pointer
    S0,
    // return value
    A0,
    // return address
    Ra,
    // function arguments
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    // scratch
    T0,
    T1,
    T2,
    // hardwired to 0
    Zero,
}

impl Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Reg::Zero => write!(f, "zero"),
            Reg::A0 => write!(f, "a0"),
            Reg::T0 => write!(f, "t0"),
            Reg::T1 => write!(f, "t1"),
            Reg::T2 => write!(f, "t2"),
            _ => {
                todo!()
            }
        }
    }
}

enum Instr {
    // arithmetic
    Add(Reg, Reg, Reg),  // add rd, rs1, rs2
    Sub(Reg, Reg, Reg),  // sub rd, rs1, rs2
    Addi(Reg, Reg, i32), // addi rd, rs1, imm

    // load/store
    Lw(Reg, i32, Reg), //lw rd, offset(rs1)
    Sw(Reg, i32, Reg), //sw rs2, offset(rs1)

    // load immediate
    Li(Reg, i32), //li rd, imm

    Mv(Reg, Reg), //rd, rs1

    // branch
    Bge(Reg, Reg, String), //bge rs1, rs2, label

    // jump and link register
    Jalr(Reg, Reg, i32), //jalr rd, rs1, imm
}

impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instr::Add(rd, rs1, rs2) => write!(f, "add {}, {}, {}", rd, rs1, rs2),
            Instr::Li(rd, imm) => write!(f, "li {}, {}", rd, imm),
            Instr::Mv(rd, rs1) => write!(f, "mv {}, {}", rd, rs1),
            _ => {
                todo!()
            }
        }
    }
}
struct CodeGen<'a> {
    ast: &'a Program,
}

impl<'a> CodeGen<'a> {
    fn new(ast: &'a Program) -> Self {
        Self { ast }
    }

    fn gen_program(&self) -> Vec<Instr> {
        todo!()
    }

    pub fn gen_code(program: Vec<Instr>) -> String {
        program
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn gen_expr(expr: &Expr, dst: Reg) -> Vec<Instr> {
        let mut instrs = Vec::new();

        match expr {
            CompExpr(lhs, None) => {
                instrs.extend(CodeGen::gen_arith_expr(lhs, Reg::T0));
                instrs.push(Instr::Mv(dst, Reg::T0));
            }
            _ => {
                todo!()
            }
        }
        instrs
    }

    fn gen_arith_expr(expr: &ArithExpr, dst: Reg) -> Vec<Instr> {
        let mut instrs = Vec::new();
        match expr {
            ArithExpr(lhs, _v) if _v.is_empty() => {
                instrs.extend(CodeGen::gen_atom_expr(lhs, Reg::T1));
                instrs.push(Instr::Mv(dst, Reg::T1));
            }
            _ => {
                todo!()
            }
        }
        instrs
    }

    fn gen_atom_expr(expr: &AtomExpr, dst: Reg) -> Vec<Instr> {
        match expr {
            AtomExpr::Num(num) => {
                vec![Instr::Li(dst, num.name.parse().unwrap())]
            }
            _ => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn gen_num_atom_expr() {
        let expected_instrs = indoc! {"
        li t1, 5
        mv t0, t1
        mv a0, t0"};
        let st = SyntaxToken {
            token: Token::Num(String::from("5")),
            span: Span::default(),
        };
        let expr = CompExpr(
            ArithExpr(
                AtomExpr::Num(Num {
                    st: st,
                    name: String::from("5"),
                }),
                vec![],
            ),
            None,
        );
        let instrs = CodeGen::gen_code(CodeGen::gen_expr(&expr, Reg::A0));
        assert_eq!(expected_instrs, instrs);
    }
}

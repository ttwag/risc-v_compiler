use crate::ast::{ArithExpr, ArithOp, AtomExpr, CompExpr, CompOp, Expr, Id, Program, Stmt, Type};
use crate::token::{Location, SyntaxToken};
use core::fmt;
use std::{collections::HashMap, fmt::Display};

const WORD_SIZE: usize = 4;

#[derive(Debug)]
enum CGError {
    UndefinedVariable(SyntaxToken),
    VarRedefinition(SyntaxToken),
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
            Reg::Sp => write!(f, "sp"),
            Reg::S0 => write!(f, "s0"),
            _ => {
                todo!()
            }
        }
    }
}

#[derive(Debug)]
enum Instr {
    // arithmetic
    Add(Reg, Reg, Reg),  // add rd, rs1, rs2
    Sub(Reg, Reg, Reg),  // sub rd, rs1, rs2
    Addi(Reg, Reg, i32), // addi rd, rs1, imm
    Slt(Reg, Reg, Reg),  // rd, rs1, rs2
    Xor(Reg, Reg, Reg),  // rd, rs1, rs2
    Seqz(Reg, Reg),      // rd, rs1

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

impl Instr {
    fn gen_mv(dst: Reg, rs1: Reg) -> Vec<Instr> {
        if dst == rs1 {
            vec![]
        } else {
            vec![Instr::Mv(dst, rs1)]
        }
    }
}

#[rustfmt::skip]
impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instr::Add(rd, rs1, rs2) => write!(f, "add {}, {}, {}", rd, rs1, rs2),
            Instr::Addi(rd, rs1, imm) => write!(f, "addi {}, {}, {}", rd, rs1, imm),
            Instr::Sub(rd, rs1, rs2) => write!(f, "sub {}, {}, {}", rd, rs1, rs2),
            Instr::Li(rd, imm)             => write!(f, "li {}, {}", rd, imm),
            Instr::Mv(rd, rs1)             => write!(f, "mv {}, {}", rd, rs1),
            Instr::Slt(rd, rs1, rs2) => write!(f, "slt {}, {}, {}", rd, rs1, rs2),
            Instr::Xor(rd, rs1, rs2) => write!(f, "xor {}, {}, {}", rd, rs1, rs2),
            Instr::Seqz(rd, rs1)           => write!(f, "seqz {}, {}", rd, rs1),
            Instr::Sw(rs2, offset, rs1) => write!(f, "sw {}, {}({})", rs2, offset, rs1),
            Instr::Lw(rd, offset, rs1) => write!(f, "lw {}, {}({})", rd, offset, rs1),
            _ => {
                todo!()
            }
        }
    }
}
struct CodeGen<'a> {
    ast: &'a Program,
    stack: Vec<Vec<Reg>>,
    locals: HashMap<String, i32>,
    next_local_offset: i32,
}

impl<'a> CodeGen<'a> {
    fn new(ast: &'a Program) -> Self {
        Self {
            ast,
            stack: vec![],
            locals: HashMap::new(),
            next_local_offset: 0,
        }
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

    fn gen_stmt(&mut self, stmt: &Stmt) -> Result<Vec<Instr>, CGError> {
        match stmt {
            Stmt::Let(id, _var_type, expr) => {
                let var = id.name.clone();
                if self.locals.contains_key(&var) {
                    Err(CGError::VarRedefinition(id.st.clone()))
                } else {
                    let mut instrs = Vec::new();
                    let dst = Reg::T0;
                    instrs.extend(self.gen_expr(expr, dst)?);
                    instrs.push(self.declare_local(var, dst));
                    Ok(instrs)
                }
            }
            _ => {
                todo!()
            }
        }
    }

    fn gen_expr(&mut self, expr: &Expr, dst: Reg) -> Result<Vec<Instr>, CGError> {
        let mut instrs = Vec::new();

        match expr {
            CompExpr(lhs, None) => {
                instrs.extend(self.gen_arith_expr(lhs, Reg::T0)?);
                instrs.extend(Instr::gen_mv(dst, Reg::T0));
            }
            CompExpr(lhs, Some((op, rhs))) => {
                instrs.extend(self.gen_arith_expr(lhs, Reg::T0)?);
                instrs.extend(self.gen_arith_expr(rhs, Reg::T1)?);
                instrs.extend(CodeGen::gen_comp_op(op, dst, Reg::T0, Reg::T1));
            }
        }
        Ok(instrs)
    }

    fn gen_comp_op(op: &CompOp, dst: Reg, rs1: Reg, rs2: Reg) -> Vec<Instr> {
        match op {
            CompOp::Equality => vec![Instr::Xor(rs1, rs1, rs2), Instr::Seqz(dst, rs1)],
            CompOp::Grt => vec![Instr::Slt(dst, rs2, rs1)],
        }
    }

    fn gen_arith_expr(&mut self, expr: &ArithExpr, dst: Reg) -> Result<Vec<Instr>, CGError> {
        let mut instrs = Vec::new();
        match expr {
            ArithExpr(lhs, _v) if _v.is_empty() => {
                instrs.extend(self.gen_atom_expr(lhs, Reg::T1)?);
            }
            ArithExpr(lhs, v) => {
                instrs.extend(self.gen_atom_expr(lhs, Reg::T1)?);
                for (op, rhs) in v {
                    instrs.extend(self.gen_atom_expr(rhs, Reg::T2)?);
                    instrs.extend(CodeGen::gen_arith_op(op, Reg::T1, Reg::T1, Reg::T2));
                }
            }
        }
        instrs.extend(Instr::gen_mv(dst, Reg::T1));
        Ok(instrs)
    }

    fn gen_arith_op(op: &ArithOp, dst: Reg, rs1: Reg, rs2: Reg) -> Vec<Instr> {
        match op {
            ArithOp::Plus => vec![Instr::Add(dst, rs1, rs2)],
            ArithOp::Minus => vec![Instr::Sub(dst, rs1, rs2)],
        }
    }

    fn gen_atom_expr(&mut self, expr: &AtomExpr, dst: Reg) -> Result<Vec<Instr>, CGError> {
        match expr {
            AtomExpr::Num(num) => Ok(vec![Instr::Li(dst, num.name.parse().unwrap())]),
            AtomExpr::Id(id) => Ok(vec![self.load_local(id, dst)?]),
            AtomExpr::Group(expr) => {
                let mut instrs = Vec::new();
                let regs = [Reg::T0, Reg::T1, Reg::T2]
                    .into_iter()
                    .filter(|&r| r != dst)
                    .collect::<Vec<_>>();
                instrs.extend(self.push(regs));
                instrs.extend(self.gen_expr(expr, dst)?);
                instrs.extend(self.pop());
                Ok(instrs)
            }
            _ => {
                todo!()
            }
        }
    }

    fn push(&mut self, regs: Vec<Reg>) -> Vec<Instr> {
        let mut instrs = Vec::new();
        let size = regs.len();

        let mut offset = 0;
        instrs.push(Instr::Addi(Reg::Sp, Reg::Sp, -((size * WORD_SIZE) as i32)));
        for reg in &regs {
            instrs.push(Instr::Sw(reg.clone(), offset, Reg::Sp));
            offset += WORD_SIZE as i32;
        }

        self.stack.push(regs);
        instrs
    }

    fn pop(&mut self) -> Vec<Instr> {
        let regs = self.stack.pop().expect("pop with empty stack");
        let size = regs.len();
        let mut instrs = Vec::new();

        let mut offset = 0;
        for reg in regs {
            instrs.push(Instr::Lw(reg, offset, Reg::Sp));
            offset += WORD_SIZE as i32;
        }

        instrs.push(Instr::Addi(Reg::Sp, Reg::Sp, (size * WORD_SIZE) as i32));
        instrs
    }

    fn declare_local(&mut self, var: String, dst: Reg) -> Instr {
        let offset = self.next_local_offset;
        self.locals.insert(var, offset);
        self.next_local_offset -= WORD_SIZE as i32;
        Instr::Sw(dst, offset, Reg::S0)
    }

    fn load_local(&self, id: &Id, dst: Reg) -> Result<Instr, CGError> {
        let var = &id.name;
        if let Some(&offset) = self.locals.get(var) {
            Ok(Instr::Lw(dst, offset, Reg::S0))
        } else {
            Err(CGError::UndefinedVariable(id.st.clone()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::Num;
    use crate::token::SyntaxToken;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    fn assert_cg_expr(expected_instrs: &str, expr: &CompExpr) {
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_expr(&expr, Reg::A0).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    // ── Expr ───────────────────────────────────────────────────────────────
    #[test]
    fn gen_comp_with_lhs_num() {
        let expected_instrs = indoc! {"
        li t1, 5
        mv t0, t1
        mv a0, t0"};
        let expr = CompExpr(
            ArithExpr(
                AtomExpr::Num(Num {
                    st: SyntaxToken::default(),
                    name: String::from("5"),
                }),
                vec![],
            ),
            None,
        );
        assert_cg_expr(expected_instrs, &expr);
    }

    // Input: 5 > 6
    #[test]
    fn gen_comp_grt_with_lhs_rhs_num() {
        let expected_instrs = indoc! {"
        li t1, 5
        mv t0, t1
        li t1, 6
        slt a0, t1, t0"};
        let expr = CompExpr(
            ArithExpr(
                AtomExpr::Num(Num {
                    st: SyntaxToken::default(),
                    name: String::from("5"),
                }),
                vec![],
            ),
            Some((
                CompOp::Grt,
                ArithExpr(
                    AtomExpr::Num(Num {
                        st: SyntaxToken::default(),
                        name: String::from("6"),
                    }),
                    vec![],
                ),
            )),
        );
        assert_cg_expr(expected_instrs, &expr);
    }

    // Input: 5 == 6
    #[test]
    fn gen_comp_equality_with_lhs_rhs_num() {
        let expected_instrs = indoc! {"
        li t1, 5
        mv t0, t1
        li t1, 6
        xor t0, t0, t1
        seqz a0, t0"};
        let expr = CompExpr(
            ArithExpr(
                AtomExpr::Num(Num {
                    st: SyntaxToken::default(),
                    name: String::from("5"),
                }),
                vec![],
            ),
            Some((
                CompOp::Equality,
                ArithExpr(
                    AtomExpr::Num(Num {
                        st: SyntaxToken::default(),
                        name: String::from("6"),
                    }),
                    vec![],
                ),
            )),
        );
        assert_cg_expr(expected_instrs, &expr);
    }

    // Input: 5 + 7 - 10
    #[test]
    fn gen_arith_plus_minus_with_lhs_rhs_num() {
        let expected_instrs = indoc! {"
        li t1, 5
        li t2, 7
        add t1, t1, t2
        li t2, 10
        sub t1, t1, t2
        mv t0, t1
        mv a0, t0"};
        let expr = CompExpr(
            ArithExpr(
                AtomExpr::Num(Num {
                    st: SyntaxToken::default(),
                    name: String::from("5"),
                }),
                vec![
                    (
                        ArithOp::Plus,
                        AtomExpr::Num(Num {
                            st: SyntaxToken::default(),
                            name: String::from("7"),
                        }),
                    ),
                    (
                        ArithOp::Minus,
                        AtomExpr::Num(Num {
                            st: SyntaxToken::default(),
                            name: String::from("10"),
                        }),
                    ),
                ],
            ),
            None,
        );
        assert_cg_expr(expected_instrs, &expr);
    }

    // Input: 5 + 8 == 6 + 7
    #[test]
    fn gen_comp_equality_with_lhs_rhs_arith() {
        let expected_instrs = indoc! {"
        li t1, 5
        li t2, 8
        add t1, t1, t2
        mv t0, t1
        li t1, 6
        li t2, 7
        add t1, t1, t2
        xor t0, t0, t1
        seqz a0, t0"};
        let expr = CompExpr(
            ArithExpr(
                AtomExpr::Num(Num {
                    st: SyntaxToken::default(),
                    name: String::from("5"),
                }),
                vec![(
                    ArithOp::Plus,
                    AtomExpr::Num(Num {
                        st: SyntaxToken::default(),
                        name: String::from("8"),
                    }),
                )],
            ),
            Some((
                CompOp::Equality,
                ArithExpr(
                    AtomExpr::Num(Num {
                        st: SyntaxToken::default(),
                        name: String::from("6"),
                    }),
                    vec![(
                        ArithOp::Plus,
                        AtomExpr::Num(Num {
                            st: SyntaxToken::default(),
                            name: String::from("7"),
                        }),
                    )],
                ),
            )),
        );
        assert_cg_expr(expected_instrs, &expr);
    }

    // Input: 1 + (5)
    #[test]
    fn gen_comp_with_lhs_group() {
        let expected_instrs = indoc! {"
        li t1, 1
        addi sp, sp, -8
        sw t0, 0(sp)
        sw t1, 4(sp)
        li t1, 5
        mv t0, t1
        mv t2, t0
        lw t0, 0(sp)
        lw t1, 4(sp)
        addi sp, sp, 8
        add t1, t1, t2
        mv t0, t1
        mv a0, t0"};
        let expr: Expr = CompExpr(
            ArithExpr(
                AtomExpr::Num(Num {
                    st: SyntaxToken::default(),
                    name: String::from("1"),
                }),
                vec![(
                    ArithOp::Plus,
                    AtomExpr::Group(Box::new(CompExpr(
                        ArithExpr(
                            AtomExpr::Num(Num {
                                st: SyntaxToken::default(),
                                name: String::from("5"),
                            }),
                            vec![],
                        ),
                        None,
                    ))),
                )],
            ),
            None,
        );
        assert_cg_expr(expected_instrs, &expr);
    }

    // Input: let a: int := 5 ;
    #[test]
    fn gen_let_stmt() {
        let expected_instrs = indoc! {"
            li t1, 5
            mv t0, t1
            sw t0, 0(s0)"};

        let stmt: Stmt = Stmt::Let(
            Id {
                st: SyntaxToken::default(),
                name: String::from("a"),
            },
            Type::Int,
            CompExpr(
                ArithExpr(
                    AtomExpr::Num(Num {
                        st: SyntaxToken::default(),
                        name: String::from("5"),
                    }),
                    vec![],
                ),
                None,
            ),
        );
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_stmt(&stmt).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    // Input: b ;
    #[test]
    fn gen_error_undefined_var() {
        let expr = CompExpr(
            ArithExpr(
                AtomExpr::Id(Id {
                    st: SyntaxToken::default(),
                    name: String::from("b"),
                }),
                vec![],
            ),
            None,
        );
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let err = cg.gen_expr(&expr, Reg::A0).unwrap_err();
        assert!(matches!(err, CGError::UndefinedVariable(..)));
    }
}

use crate::ast::{
    ArithExpr, ArithOp, AtomExpr, CompExpr, CompOp, Expr, FuncDef, Id, Param, Program, ReturnStmt,
    Stmt, Type,
};
use crate::token::{Location, SyntaxToken};
use core::fmt;
use std::vec;
use std::{collections::HashMap, fmt::Display};

const WORD_SIZE: usize = 4;

#[derive(Debug)]
enum CGError {
    UndefinedVariable(SyntaxToken),
    VarRedefinition(SyntaxToken),
    TooManyParam(SyntaxToken),
}

impl CGError {
    pub fn undefined_variable(st: &SyntaxToken) -> Self {
        Self::UndefinedVariable(st.clone())
    }

    pub fn var_redefinition(st: &SyntaxToken) -> Self {
        Self::VarRedefinition(st.clone())
    }

    pub fn too_many_param(st: &SyntaxToken) -> Self {
        Self::TooManyParam(st.clone())
    }
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
            Reg::A1 => write!(f, "a1"),
            Reg::A2 => write!(f, "a2"),
            Reg::A3 => write!(f, "a3"),
            Reg::A4 => write!(f, "a4"),
            Reg::A5 => write!(f, "a5"),
            Reg::A6 => write!(f, "a6"),
            Reg::A7 => write!(f, "a7"),
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

    // return (jalr x0, ra, 0)
    Ret,

    Label(String),
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
            Instr::Ret => write!(f, "ret"),
            _ => {
                todo!()
            }
        }
    }
}

struct Frame {
    spill_stack: Vec<Vec<(Reg, i32)>>,
    locals: HashMap<String, i32>,
    frame_offset: i32,
}

impl Frame {
    fn new() -> Self {
        Self {
            spill_stack: vec![],
            locals: HashMap::new(),
            frame_offset: 0,
        }
    }

    fn alloc_slot(&mut self) -> i32 {
        self.frame_offset -= WORD_SIZE as i32;
        self.frame_offset
    }

    fn spill(&mut self, regs: Vec<Reg>) -> Vec<Instr> {
        let mut instrs = Vec::new();
        let mut spill = Vec::new();
        for &reg in &regs {
            let offset = self.alloc_slot();
            spill.push((reg, offset));
            instrs.push(Instr::Sw(reg, offset, Reg::S0));
        }
        self.spill_stack.push(spill);

        instrs
    }

    fn unspill(&mut self) -> Vec<Instr> {
        let spill = self.spill_stack.pop().expect("pop with empty stack");
        let mut instrs = Vec::new();

        for (reg, offset) in spill {
            instrs.push(Instr::Lw(reg, offset, Reg::S0));
        }

        instrs
    }

    fn define_local(&mut self, id: &Id, src: Reg) -> Result<Instr, CGError> {
        let var = &id.name;
        if self.locals.contains_key(var) {
            Err(CGError::var_redefinition(&id.st))
        } else {
            let offset = self.alloc_slot();
            self.locals.insert(var.to_owned(), offset);
            Ok(Instr::Sw(src, self.frame_offset, Reg::S0))
        }
    }

    fn get_local_offset(&self, id: &Id) -> Result<i32, CGError> {
        let var = &id.name;
        if let Some(&offset) = self.locals.get(var) {
            Ok(offset)
        } else {
            Err(CGError::undefined_variable(&id.st))
        }
    }

    fn load_local(&self, id: &Id, dst: Reg) -> Result<Instr, CGError> {
        let offset = self.get_local_offset(id)?;
        Ok(Instr::Lw(dst, offset, Reg::S0))
    }
}

struct CodeGen<'a> {
    ast: &'a Program,
    frame: Frame,
}

impl<'a> CodeGen<'a> {
    fn new(ast: &'a Program) -> Self {
        Self {
            ast,
            frame: Frame::new(),
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

    fn gen_func_def(&mut self, func: &FuncDef) -> Result<Vec<Instr>, CGError> {
        let FuncDef {
            params,
            body,
            ret_stmt,
            ..
        } = func;
        let ReturnStmt(return_expr) = ret_stmt;
        let mut instrs = Vec::new();
        self.frame = Frame::new();

        // ra (caller saved), s0 (callee saved)
        let s0_offset = self.frame.alloc_slot();
        let ra_offset = self.frame.alloc_slot();

        // code generation
        instrs.extend(self.gen_params(params)?);
        for stmt in body {
            instrs.extend(self.gen_stmt(stmt)?);
        }
        instrs.extend(self.gen_expr(return_expr, Reg::A0)?);

        // code for entering and exiting the function
        let frame_size = (((-self.frame.frame_offset as usize) + 15) & !15) as i32; // frame size must be 16 byte-aligned
        let mut prologue = vec![
            Instr::Label(func.name.name.clone()),
            Instr::Addi(Reg::Sp, Reg::Sp, -frame_size), // move Sp up to the frame size
            // sp-relative = frame_size + s0-relative
            Instr::Sw(Reg::Ra, frame_size + ra_offset, Reg::Sp), // save return address to frame
            Instr::Sw(Reg::S0, frame_size + s0_offset, Reg::Sp), // save current S0 to frame
            Instr::Addi(Reg::S0, Reg::Sp, frame_size),           // put S0 at the base of the frame
        ];
        let epilogue = vec![
            Instr::Lw(Reg::Ra, ra_offset, Reg::S0), // load return address from frame to reg
            Instr::Lw(Reg::S0, s0_offset, Reg::S0), // load s0 from frame to reg
            Instr::Addi(Reg::Sp, Reg::Sp, frame_size), // move sp back to base
            Instr::Ret,                             // return to return address
        ];

        instrs.extend(epilogue);
        prologue.extend(instrs);
        Ok(prologue)
    }

    fn gen_params(&mut self, params: &Vec<Param>) -> Result<Vec<Instr>, CGError> {
        // spill param
        let mut src_iter = [
            Reg::A0,
            Reg::A1,
            Reg::A2,
            Reg::A3,
            Reg::A4,
            Reg::A5,
            Reg::A6,
            Reg::A7,
        ]
        .into_iter();

        let mut instrs = Vec::new();
        for Param(id, _param_type) in params {
            let src = src_iter
                .next()
                .ok_or_else(|| CGError::too_many_param(&id.st))?;
            instrs.push(self.frame.define_local(id, src)?);
        }
        Ok(instrs)
    }

    fn gen_return_stmt(&mut self, ReturnStmt(expr): &ReturnStmt) -> Result<Vec<Instr>, CGError> {
        let mut instrs = Vec::new();
        instrs.extend(self.gen_expr(expr, Reg::A0)?);
        instrs.push(Instr::Ret);
        Ok(instrs)
    }

    fn gen_stmt(&mut self, stmt: &Stmt) -> Result<Vec<Instr>, CGError> {
        match stmt {
            Stmt::Let(id, _var_type, expr) => {
                let mut instrs = Vec::new();
                let dst = Reg::T0;

                instrs.extend(self.gen_expr(expr, dst)?);
                instrs.push(self.frame.define_local(id, dst)?);
                Ok(instrs)
            }
            Stmt::Assign(id, expr) => {
                let offset = self.frame.get_local_offset(id)?;
                let mut instrs = Vec::new();
                let src = Reg::T0;

                instrs.extend(self.gen_expr(expr, src)?);
                instrs.push(Instr::Sw(src, offset, Reg::S0));
                Ok(instrs)
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
                instrs.extend(self.gen_arith_expr(lhs, dst)?);
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
                // safe to store to dst because there's no temporary result from arithmetic
                instrs.extend(self.gen_atom_expr(lhs, dst)?);
            }
            ArithExpr(lhs, v) => {
                instrs.extend(self.gen_atom_expr(lhs, Reg::T1)?);
                for (op, rhs) in v {
                    instrs.extend(self.gen_atom_expr(rhs, Reg::T2)?);
                    instrs.extend(CodeGen::gen_arith_op(op, Reg::T1, Reg::T1, Reg::T2));
                }
                // a final move is needed in this arm because the temporary result of arithmetic is stored in T1, but dst might be T0
                instrs.extend(Instr::gen_mv(dst, Reg::T1));
            }
        }
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
            AtomExpr::Id(id) => Ok(vec![self.frame.load_local(id, dst)?]),
            AtomExpr::Group(expr) => {
                let mut instrs = Vec::new();
                let regs = [Reg::T0, Reg::T1, Reg::T2]
                    .into_iter()
                    .filter(|&r| r != dst)
                    .collect::<Vec<_>>();
                instrs.extend(self.frame.spill(regs));
                instrs.extend(self.gen_expr(expr, dst)?);
                instrs.extend(self.frame.unspill());
                Ok(instrs)
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
    use crate::ast::Num;
    use crate::token::SyntaxToken;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[track_caller]
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
        li a0, 5"};
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
        li t0, 5
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
        li t0, 5
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
        mv a0, t1"};
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
        sw t0, -4(s0)
        sw t1, -8(s0)
        li t2, 5
        lw t0, -4(s0)
        lw t1, -8(s0)
        add t1, t1, t2
        mv a0, t1"};
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
            li t0, 5
            sw t0, -4(s0)"};

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

    // Input: b := 5 ;
    #[test]
    fn gen_error_undefined_var_assign() {
        let stmt = Stmt::Assign(
            Id {
                st: SyntaxToken::default(),
                name: String::from("b"),
            },
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
        let err = cg.gen_stmt(&stmt).unwrap_err();
        assert!(matches!(err, CGError::UndefinedVariable(..)));
    }

    // Input: return 5;
    #[test]
    fn gen_return_stmt() {
        let expected_instrs = indoc! {"
            li a0, 5
            ret"};
        let stmt = ReturnStmt(CompExpr(
            ArithExpr(
                AtomExpr::Num(Num {
                    st: SyntaxToken::default(),
                    name: String::from("5"),
                }),
                vec![],
            ),
            None,
        ));
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_return_stmt(&stmt).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    // Input: (a: int, b: int)
    #[test]
    fn gen_params_with_two_param() {
        let expected_instrs = indoc! {"
            sw a0, -4(s0)
            sw a1, -8(s0)"};

        let param = vec![
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("a"),
                },
                Type::Int,
            ),
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("b"),
                },
                Type::Int,
            ),
        ];
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_params(&param).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    // Input: (a: int, b: int, c: int, d: int, e: int, f: int, g: int, h: int, i: int)
    #[test]
    fn gen_params_with_too_many_param() {
        let expected_instrs = indoc! {"
            sw a0, 0(s0)
            sw a1, -4(s0)"};

        let param = vec![
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("a"),
                },
                Type::Int,
            ),
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("b"),
                },
                Type::Int,
            ),
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("c"),
                },
                Type::Int,
            ),
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("d"),
                },
                Type::Int,
            ),
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("e"),
                },
                Type::Int,
            ),
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("f"),
                },
                Type::Int,
            ),
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("g"),
                },
                Type::Int,
            ),
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("h"),
                },
                Type::Int,
            ),
            Param(
                Id {
                    st: SyntaxToken::default(),
                    name: String::from("i"),
                },
                Type::Int,
            ),
        ];
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let err = cg.gen_params(&param).unwrap_err();
        assert!(matches!(err, CGError::TooManyParam(..)));
    }
}

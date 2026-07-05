use crate::ast::{
    ArithExpr, ArithOp, AtomExpr, Branch, CompExpr, CompOp, Expr, FuncDef, Id, Param, Program,
    ReturnStmt, Stmt,
};
use crate::token::SyntaxToken;
use core::{fmt, fmt::Display};
use std::collections::HashMap;

#[derive(Debug)]
pub enum CGError {
    UndefinedVariable(SyntaxToken),
    UndefinedFunction(SyntaxToken),
    VarRedefinition(SyntaxToken),
    TooManyParam(SyntaxToken),
    UndefinedMain,
}

impl fmt::Display for CGError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CGError::UndefinedVariable(st) => {
                write!(f, "undefined variable {st}")
            }
            CGError::UndefinedFunction(st) => {
                write!(f, "undefined function {st}")
            }
            CGError::VarRedefinition(st) => {
                write!(f, "variable redefinition {st}")
            }
            CGError::TooManyParam(st) => {
                write!(f, "too many parameter {st}")
            }
            CGError::UndefinedMain => write!(f, "undefined 'main' function"),
        }
    }
}

impl std::error::Error for CGError {}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Reg {
    // stack pointer
    Sp,
    // frame pointer
    S0,
    S1,
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
}

impl Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
            Reg::S1 => write!(f, "s1"),
            Reg::Ra => write!(f, "ra"),
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

    // branch if <= 0
    Blez(Reg, String), //blez rs1, label

    // jump to label
    J(String), //jal x0, label

    // return (jalr x0, ra, 0)
    Ret,

    Label(String),

    Directive(String),

    Call(String),

    Ecall,
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
        const INDENT: &str = "    "; // 4 spaces
        match self {
            Instr::Directive(text)               => write!(f, "{}", text),
            Instr::Label(name)                   => write!(f, "{}:", name),
            Instr::Add(rd, rs1, rs2)    => write!(f, "{INDENT}add {}, {}, {}", rd, rs1, rs2),
            Instr::Addi(rd, rs1, imm)   => write!(f, "{INDENT}addi {}, {}, {}", rd, rs1, imm),
            Instr::Sub(rd, rs1, rs2)    => write!(f, "{INDENT}sub {}, {}, {}", rd, rs1, rs2),
            Instr::Li(rd, imm)                => write!(f, "{INDENT}li {}, {}", rd, imm),
            Instr::Mv(rd, rs1)                => write!(f, "{INDENT}mv {}, {}", rd, rs1),
            Instr::Slt(rd, rs1, rs2)    => write!(f, "{INDENT}slt {}, {}, {}", rd, rs1, rs2),
            Instr::Xor(rd, rs1, rs2)    => write!(f, "{INDENT}xor {}, {}, {}", rd, rs1, rs2),
            Instr::Seqz(rd, rs1)              => write!(f, "{INDENT}seqz {}, {}", rd, rs1),
            Instr::Blez(rs1, label)        => write!(f, "{INDENT}blez {}, {}", rs1, label),
            Instr::J(label)                      => write!(f, "{INDENT}j {}", label),
            Instr::Sw(rs2, offset, rs1) => write!(f, "{INDENT}sw {}, {}({})", rs2, offset, rs1),
            Instr::Lw(rd, offset, rs1)  => write!(f, "{INDENT}lw {}, {}({})", rd, offset, rs1),
            Instr::Ret                                    => write!(f, "{INDENT}ret"),
            Instr::Call(label)                   => write!(f, "{INDENT}call {}", label),
            Instr::Ecall                                  => write!(f, "{INDENT}ecall"),
        }
    }
}

struct Frame {
    spill_stack: Vec<Vec<(Reg, i32)>>,
    locals: Vec<HashMap<String, i32>>,
    frame_offset: i32,
}

impl Frame {
    const WORD_SIZE: usize = 4;
    fn new() -> Self {
        Self {
            spill_stack: vec![],
            locals: vec![],
            frame_offset: 0,
        }
    }

    fn alloc_slot(&mut self) -> i32 {
        self.frame_offset -= Self::WORD_SIZE as i32;
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
        if self
            .locals
            .last()
            .expect("locals stack should never be empty")
            .contains_key(var)
        {
            Err(CGError::VarRedefinition(id.st.clone()))
        } else {
            let offset = self.alloc_slot();
            self.locals
                .last_mut()
                .expect("locals stack should never be empty")
                .insert(var.to_owned(), offset);
            Ok(Instr::Sw(src, self.frame_offset, Reg::S0))
        }
    }

    fn get_local_offset(&self, id: &Id) -> Result<i32, CGError> {
        let var = &id.name;
        for local in self.locals.iter().rev() {
            if let Some(&offset) = local.get(var) {
                return Ok(offset);
            }
        }
        Err(CGError::UndefinedVariable(id.st.clone()))
    }

    fn load_local(&self, id: &Id, dst: Reg) -> Result<Instr, CGError> {
        let offset = self.get_local_offset(id)?;
        Ok(Instr::Lw(dst, offset, Reg::S0))
    }

    fn push_local_scope(&mut self) {
        self.locals.push(HashMap::new());
    }

    fn pop_local_scope(&mut self) {
        self.locals.pop();
    }
}

enum LabeledBranch<'a> {
    If(String, String, &'a Expr, &'a Vec<Stmt>), // cond_label, jump_label, cond, body
    Elif(String, String, String, &'a Expr, &'a Vec<Stmt>), // body_label, cond_label, jump_label, cond, body
    Else(String, &'a Vec<Stmt>),                           // body_label, body
}

pub struct CodeGen<'a> {
    ast: &'a Program,
    frame: Frame,
    func_arity: HashMap<String, usize>,
    label_counter: usize,
}

impl<'a> CodeGen<'a> {
    // Creates a new code generation unit for the abstract syntax tree.
    pub fn new(ast: &'a Program) -> Self {
        Self {
            ast,
            frame: Frame::new(),
            func_arity: HashMap::new(),
            label_counter: 0,
        }
    }

    /// Generates ABI-compliant RISC-V assembly code for the abstract syntax tree given in CodeGen::new()
    ///
    /// # Examples
    /// ```
    /// use risc_v_compiler::scanner::Scanner;
    /// use risc_v_compiler::parser::Parser;
    /// use risc_v_compiler::code_gen::CodeGen;
    ///
    /// let src = "fn main() -> int { return 0; }";
    /// let sts = Scanner::new(src).scan().unwrap();
    /// let ast = Parser::new(&sts).parse().unwrap();
    /// let code = CodeGen::new(&ast).generate();
    /// ```
    pub fn generate(&mut self) -> Result<String, CGError> {
        let program = self.gen_program()?;
        Ok(CodeGen::gen_code(program))
    }

    fn gen_program(&mut self) -> Result<Vec<Instr>, CGError> {
        let mut instrs = Vec::new();
        let mut has_main = false;
        let Program(func_defs) = self.ast;

        for func_def in func_defs {
            let name = func_def.name.name.to_owned();
            has_main = name == "main";
            self.func_arity.insert(name, func_def.params.len());
            instrs.extend(self.gen_func_def(func_def)?);
        }

        if !has_main {
            Err(CGError::UndefinedMain)
        } else {
            let mut directives = vec![
                Instr::Directive(String::from(".text")),
                Instr::Directive(String::from(".global _start")),
                Instr::Label(String::from("_start")),
                Instr::Call(String::from("main")),
                Instr::Li(Reg::A7, 93),
                Instr::Ecall,
            ];
            directives.extend(instrs);
            Ok(directives)
        }
    }

    fn gen_code(program: Vec<Instr>) -> String {
        program
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }

    // ── Function Definition ──────────────────────────────────────────────────────────────────
    fn gen_func_def(&mut self, func_def: &FuncDef) -> Result<Vec<Instr>, CGError> {
        let FuncDef {
            params,
            body,
            ret_stmt,
            ..
        } = func_def;
        let ReturnStmt(return_expr) = ret_stmt;
        let mut instrs = Vec::new();
        self.frame = Frame::new();

        // ra (caller saved), s0 (callee saved), s1 (callee saved)
        let s0_offset = self.frame.alloc_slot();
        let s1_offset = self.frame.alloc_slot();
        let ra_offset = self.frame.alloc_slot();

        // code generation
        self.frame.push_local_scope();
        instrs.extend(self.gen_params(params)?);
        for stmt in body {
            instrs.extend(self.gen_stmt(stmt)?);
        }
        instrs.extend(self.gen_expr(return_expr, Reg::A0)?);
        self.frame.pop_local_scope();

        // code for entering and exiting the function
        let frame_size = (((-self.frame.frame_offset as usize) + 15) & !15) as i32; // frame size must be 16 byte-aligned
        let mut prologue = vec![
            Instr::Label(func_def.name.name.clone()),
            Instr::Addi(Reg::Sp, Reg::Sp, -frame_size), // move Sp up to the frame size
            // sp-relative = frame_size + s0-relative
            Instr::Sw(Reg::Ra, frame_size + ra_offset, Reg::Sp), // save return address to frame
            Instr::Sw(Reg::S1, frame_size + s1_offset, Reg::Sp),
            Instr::Sw(Reg::S0, frame_size + s0_offset, Reg::Sp), // save current S0 to frame
            Instr::Addi(Reg::S0, Reg::Sp, frame_size),           // put S0 at the base of the frame
        ];
        let epilogue = vec![
            Instr::Lw(Reg::Ra, ra_offset, Reg::S0), // load return address from frame to reg
            Instr::Lw(Reg::S1, s1_offset, Reg::S0),
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
                .ok_or_else(|| CGError::TooManyParam(id.st.clone()))?;
            instrs.push(self.frame.define_local(id, src)?);
        }
        Ok(instrs)
    }

    // ── Statements ──────────────────────────────────────────────────────────────────
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
            Stmt::If(Branch(if_expr, if_stmts), elif_branches, else_branch) => {
                let mut instrs = Vec::new();
                let jump_label = format!("if_end_{}", self.inc_label_counter());

                // generate all labels for elif and else
                let mut cond_labels: Vec<String> = (0..elif_branches.len())
                    .map(|_| format!("elif_{}", self.inc_label_counter()))
                    .collect();
                if else_branch.is_some() {
                    cond_labels.push(format!("else_{}", self.inc_label_counter()))
                }
                let else_label = cond_labels.last().cloned();
                cond_labels.push(jump_label.clone());

                // labeled the conditions and bodies of if, elif, else with the generated labels
                let mut labeled_branches = Vec::new();
                let mut body_label;
                let mut cond_label_iter = cond_labels.iter();
                let mut cond_label = cond_label_iter.next().unwrap_or(&jump_label);
                labeled_branches.push(LabeledBranch::If(
                    cond_label.clone(),
                    jump_label.clone(),
                    if_expr,
                    if_stmts,
                ));
                for Branch(elif_expr, elif_stmts) in elif_branches {
                    body_label = cond_label.clone();
                    cond_label = cond_label_iter.next().unwrap_or(&jump_label);
                    labeled_branches.push(LabeledBranch::Elif(
                        body_label,
                        cond_label.clone(),
                        jump_label.clone(),
                        elif_expr,
                        elif_stmts,
                    ));
                }
                if let Some(body) = else_branch {
                    labeled_branches.push(LabeledBranch::Else(
                        else_label.expect("Else label should match else branch"),
                        body,
                    ));
                }

                // generate actual branch code
                for branch_label in labeled_branches {
                    instrs.extend(self.gen_branch(&branch_label)?);
                }

                // generate label for code after branch
                instrs.push(Instr::Label(jump_label));

                Ok(instrs)
            }
            Stmt::While(Branch(cond, body)) => {
                let mut instrs = Vec::new();
                let label_num = self.inc_label_counter();
                let start_label = format!("while_start_{}", label_num);
                let end_label = format!("while_end_{}", label_num);

                instrs.push(Instr::Label(start_label.clone()));
                instrs.extend(self.gen_expr(cond, Reg::T0)?);
                instrs.push(Instr::Blez(Reg::T0, end_label.clone()));
                instrs.extend(self.gen_body(body)?);
                instrs.push(Instr::J(start_label));
                instrs.push(Instr::Label(end_label));

                Ok(instrs)
            }
        }
    }

    fn gen_branch(&mut self, branch_label: &LabeledBranch) -> Result<Vec<Instr>, CGError> {
        let mut instrs = Vec::new();
        let cond_reg = Reg::T0;
        match branch_label {
            LabeledBranch::If(cond_label, jump_label, cond, body) => {
                instrs.extend(self.gen_expr(cond, cond_reg)?);
                instrs.push(Instr::Blez(cond_reg, cond_label.clone()));
                instrs.extend(self.gen_body(body)?);
                instrs.push(Instr::J(jump_label.clone()));
            }
            LabeledBranch::Elif(body_label, cond_label, jump_label, cond, body) => {
                instrs.push(Instr::Label(body_label.clone()));
                instrs.extend(self.gen_expr(cond, cond_reg)?);
                instrs.push(Instr::Blez(cond_reg, cond_label.clone()));
                instrs.extend(self.gen_body(body)?);
                instrs.push(Instr::J(jump_label.clone()));
            }
            LabeledBranch::Else(body_label, body) => {
                instrs.push(Instr::Label(body_label.clone()));
                instrs.extend(self.gen_body(body)?);
            }
        }

        Ok(instrs)
    }

    fn gen_body(&mut self, body: &Vec<Stmt>) -> Result<Vec<Instr>, CGError> {
        let mut instrs = Vec::new();
        self.frame.push_local_scope();
        for stmt in body {
            instrs.extend(self.gen_stmt(stmt)?);
        }
        self.frame.pop_local_scope();

        Ok(instrs)
    }

    // ── Expressions ──────────────────────────────────────────────────────────────────
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
            AtomExpr::Call(Id { st, name }, exprs) => {
                // check name in scope to see if function exist
                let param_len = exprs.len();
                if !self.func_arity.contains_key(name) {
                    Err(CGError::UndefinedFunction(st.clone()))
                } else if self.func_arity[name] != param_len {
                    Err(CGError::TooManyParam(st.clone()))
                } else {
                    let mut instrs = Vec::new();
                    let mut spill_reg: Vec<Reg> = vec![Reg::T0, Reg::T1, Reg::T2]
                        .into_iter()
                        .filter(|&x| x != dst)
                        .collect();
                    // caution: nested call could overwrite the a register, so we need to spill function parameter register
                    // ex: in (f(a, b(c))) , b could overwrite the a0 reg.
                    let param_regs = &([
                        Reg::A0,
                        Reg::A1,
                        Reg::A2,
                        Reg::A3,
                        Reg::A4,
                        Reg::A5,
                        Reg::A6,
                        Reg::A7,
                    ])[0..param_len]; //only spill the used A regs
                    spill_reg.extend(param_regs.to_vec());
                    instrs.extend(self.frame.spill(spill_reg));

                    // evaluate expr and put them into a0 - a7
                    for (expr, reg) in exprs.iter().zip(param_regs.iter()) {
                        instrs.extend(self.gen_expr(expr, reg.clone())?);
                    }
                    instrs.push(Instr::Call(name.clone()));
                    instrs.extend(Instr::gen_mv(Reg::S1, Reg::A0));
                    instrs.extend(self.frame.unspill());
                    instrs.extend(Instr::gen_mv(dst, Reg::S1));
                    Ok(instrs)
                }
            }
        }
    }

    // ── Label ──────────────────────────────────────────────────────────────────
    // Get current label count and then increment it by 1
    fn inc_label_counter(&mut self) -> usize {
        let cnt = self.label_counter;
        self.label_counter += 1;
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{Num, Type};
    use crate::token::SyntaxToken;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[track_caller]
    fn assert_cg_expr(expected_instrs: String, expr: &CompExpr) {
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_expr(&expr, Reg::A0).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    // prepend 4 spaces to the first line and swap all newline with newline + 4 spaces
    // append a newline at the end
    fn format_instr_not_in_func(instrs: &str) -> String {
        format!("    {}\n", instrs.replace('\n', "\n    "))
    }

    // ── gen_expr ───────────────────────────────────────────────────────────────
    #[test]
    fn gen_comp_with_lhs_num() {
        let expected_instrs = format_instr_not_in_func(indoc! {"
        li a0, 5"});
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
        let expected_instrs = format_instr_not_in_func(indoc! {"
        li t0, 5
        li t1, 6
        slt a0, t1, t0"});
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
        let expected_instrs = format_instr_not_in_func(indoc! {"
        li t0, 5
        li t1, 6
        xor t0, t0, t1
        seqz a0, t0"});
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
        let expected_instrs = format_instr_not_in_func(indoc! {"
        li t1, 5
        li t2, 7
        add t1, t1, t2
        li t2, 10
        sub t1, t1, t2
        mv a0, t1"});
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
        let expected_instrs = format_instr_not_in_func(indoc! {"
        li t1, 5
        li t2, 8
        add t1, t1, t2
        mv t0, t1
        li t1, 6
        li t2, 7
        add t1, t1, t2
        xor t0, t0, t1
        seqz a0, t0"});
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
        let expected_instrs = format_instr_not_in_func(indoc! {"
        li t1, 1
        sw t0, -4(s0)
        sw t1, -8(s0)
        li t2, 5
        lw t0, -4(s0)
        lw t1, -8(s0)
        add t1, t1, t2
        mv a0, t1"});
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

    // Input: foo()
    #[test]
    fn gen_call_with_no_def() {
        let expr = CompExpr(
            ArithExpr(
                AtomExpr::Call(
                    Id {
                        st: SyntaxToken::default(),
                        name: String::from("foo"),
                    },
                    vec![],
                ),
                vec![],
            ),
            None,
        );
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let err = cg.gen_expr(&expr, Reg::A0).unwrap_err();
        assert!(matches!(err, CGError::UndefinedFunction(..)));
    }

    // ── gen_stmt ──────────────────────────────────────────────────────────────────
    // Input: let a: int := 5 ;
    #[test]
    fn gen_let_stmt() {
        let expected_instrs = format_instr_not_in_func(indoc! {"
            li t0, 5
            sw t0, -4(s0)"});

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
        cg.frame.push_local_scope();
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
    #[test]
    // Input: if (5 > 4) {}
    fn gen_if_only_stmt() {
        let expected_instrs = indoc! {"
            li t0, 5
            li t1, 4
            slt t0, t1, t0
            blez t0, if_end_0
            j if_end_0
        if_end_0:
        "};
        let stmt = Stmt::If(
            Branch(
                CompExpr(
                    ArithExpr(
                        AtomExpr::Num(Num {
                            st: SyntaxToken::default(),
                            name: "5".to_string(),
                        }),
                        vec![],
                    ),
                    Some((
                        CompOp::Grt,
                        ArithExpr(
                            AtomExpr::Num(Num {
                                st: SyntaxToken::default(),
                                name: "4".to_string(),
                            }),
                            vec![],
                        ),
                    )),
                ),
                vec![],
            ),
            vec![], // no elif
            None,   // no else
        );
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_stmt(&stmt).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    #[test]
    // Input: if (5 > 4) {} elif (4 > 5) {}
    fn gen_if_elif_stmt() {
        let expected_instrs = indoc! {"
        li t0, 5
        li t1, 4
        slt t0, t1, t0
        blez t0, elif_1
        j if_end_0
    elif_1:
        li t0, 4
        li t1, 5
        slt t0, t1, t0
        blez t0, if_end_0
        j if_end_0
    if_end_0:
    "};
        let stmt = Stmt::If(
            Branch(
                CompExpr(
                    ArithExpr(
                        AtomExpr::Num(Num {
                            st: SyntaxToken::default(),
                            name: "5".to_string(),
                        }),
                        vec![],
                    ),
                    Some((
                        CompOp::Grt,
                        ArithExpr(
                            AtomExpr::Num(Num {
                                st: SyntaxToken::default(),
                                name: "4".to_string(),
                            }),
                            vec![],
                        ),
                    )),
                ),
                vec![],
            ),
            vec![Branch(
                CompExpr(
                    ArithExpr(
                        AtomExpr::Num(Num {
                            st: SyntaxToken::default(),
                            name: "4".to_string(),
                        }),
                        vec![],
                    ),
                    Some((
                        CompOp::Grt,
                        ArithExpr(
                            AtomExpr::Num(Num {
                                st: SyntaxToken::default(),
                                name: "5".to_string(),
                            }),
                            vec![],
                        ),
                    )),
                ),
                vec![],
            )],
            None, // no else
        );
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_stmt(&stmt).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    #[test]
    // Input: if (5 > 4) {} else {}
    fn gen_if_else_stmt() {
        let expected_instrs = indoc! {"
        li t0, 5
        li t1, 4
        slt t0, t1, t0
        blez t0, else_1
        j if_end_0
    else_1:
    if_end_0:
    "};
        let stmt = Stmt::If(
            Branch(
                CompExpr(
                    ArithExpr(
                        AtomExpr::Num(Num {
                            st: SyntaxToken::default(),
                            name: "5".to_string(),
                        }),
                        vec![],
                    ),
                    Some((
                        CompOp::Grt,
                        ArithExpr(
                            AtomExpr::Num(Num {
                                st: SyntaxToken::default(),
                                name: "4".to_string(),
                            }),
                            vec![],
                        ),
                    )),
                ),
                vec![],
            ),
            vec![],       // no elif
            Some(vec![]), // else {}
        );
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_stmt(&stmt).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    #[test]
    // Input: if (5 > 4) { } elif (4 > 5) {} elif (4 == 5) else {}
    fn gen_branch_stmt() {
        let expected_instrs = indoc! {"
            li t0, 5
            li t1, 4
            slt t0, t1, t0
            blez t0, elif_1
            j if_end_0
        elif_1:
            li t0, 4
            li t1, 5
            slt t0, t1, t0
            blez t0, elif_2
            j if_end_0
        elif_2:
            li t0, 4
            li t1, 5
            xor t0, t0, t1
            seqz t0, t0
            blez t0, else_3
            j if_end_0
        else_3:
        if_end_0:
        "};
        let stmt = Stmt::If(
            // if (5 > 4) {}
            Branch(
                CompExpr(
                    ArithExpr(
                        AtomExpr::Num(Num {
                            st: SyntaxToken::default(),
                            name: "5".to_string(),
                        }),
                        vec![],
                    ),
                    Some((
                        CompOp::Grt,
                        ArithExpr(
                            AtomExpr::Num(Num {
                                st: SyntaxToken::default(),
                                name: "4".to_string(),
                            }),
                            vec![],
                        ),
                    )),
                ),
                vec![], // empty body
            ),
            // elif branches
            vec![
                // elif (4 > 5) {}
                Branch(
                    CompExpr(
                        ArithExpr(
                            AtomExpr::Num(Num {
                                st: SyntaxToken::default(),
                                name: "4".to_string(),
                            }),
                            vec![],
                        ),
                        Some((
                            CompOp::Grt,
                            ArithExpr(
                                AtomExpr::Num(Num {
                                    st: SyntaxToken::default(),
                                    name: "5".to_string(),
                                }),
                                vec![],
                            ),
                        )),
                    ),
                    vec![],
                ),
                // elif (4 == 5) {}
                Branch(
                    CompExpr(
                        ArithExpr(
                            AtomExpr::Num(Num {
                                st: SyntaxToken::default(),
                                name: "4".to_string(),
                            }),
                            vec![],
                        ),
                        Some((
                            CompOp::Equality,
                            ArithExpr(
                                AtomExpr::Num(Num {
                                    st: SyntaxToken::default(),
                                    name: "5".to_string(),
                                }),
                                vec![],
                            ),
                        )),
                    ),
                    vec![],
                ),
            ],
            // else {}
            Some(vec![]),
        );
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_stmt(&stmt).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    // Input: while (1 < 5) {}
    #[test]
    fn gen_while_simple_condition() {
        let expected_instrs = indoc! {"
        while_start_0:
            li t0, 5
            li t1, 1
            slt t0, t1, t0
            blez t0, while_end_0
            j while_start_0
        while_end_0:
        "};

        let stmt = Stmt::While(Branch(
            CompExpr(
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
                            name: String::from("1"),
                        }),
                        vec![],
                    ),
                )),
            ),
            vec![],
        ));

        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_stmt(&stmt).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    // ── gen_params ──────────────────────────────────────────────────────────────────
    // Input: (a: int, b: int)
    #[test]
    fn gen_params_with_two_param() {
        let expected_instrs = format_instr_not_in_func(indoc! {"
            sw a0, -4(s0)
            sw a1, -8(s0)"});

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
        cg.frame.push_local_scope();
        let instrs = CodeGen::gen_code(cg.gen_params(&param).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    // Input: (a: int, b: int, c: int, d: int, e: int, f: int, g: int, h: int, i: int)
    #[test]
    fn gen_params_with_too_many_param() {
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
        cg.frame.push_local_scope();
        let err = cg.gen_params(&param).unwrap_err();
        assert!(matches!(err, CGError::TooManyParam(..)));
    }

    // ── gen_func_def ──────────────────────────────────────────────────────────────────
    // Input: fn foo() -> int { return 0 ; }
    #[test]
    fn gen_func_with_no_param() {
        let expected_instrs = indoc! {"
            foo:
                addi sp, sp, -16
                sw ra, 4(sp)
                sw s1, 8(sp)
                sw s0, 12(sp)
                addi s0, sp, 16
                li a0, 0
                lw ra, -12(s0)
                lw s1, -8(s0)
                lw s0, -4(s0)
                addi sp, sp, 16
                ret
        "}; //note that we need the extra newline at the end when it's a complete function
        let func_def = FuncDef {
            name: Id {
                st: SyntaxToken::default(),
                name: String::from("foo"),
            },
            params: vec![],
            ret: Type::Int,
            body: vec![],
            ret_stmt: ReturnStmt(CompExpr(
                ArithExpr(
                    AtomExpr::Num(Num {
                        st: SyntaxToken::default(),
                        name: String::from("0"),
                    }),
                    vec![],
                ),
                None,
            )),
        };
        let program = Program::default();
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_func_def(&func_def).unwrap());
        assert_eq!(expected_instrs, instrs);
    }

    // ── gen_program ──────────────────────────────────────────────────────────────────
    // Input: fn main() -> int { return 100 ; }
    #[test]
    fn gen_program_with_main() {
        let expected_instrs = indoc! {"
            .text
            .global _start
            _start:
                call main
                li a7, 93
                ecall
            main:
                addi sp, sp, -16
                sw ra, 4(sp)
                sw s1, 8(sp)
                sw s0, 12(sp)
                addi s0, sp, 16
                li a0, 100
                lw ra, -12(s0)
                lw s1, -8(s0)
                lw s0, -4(s0)
                addi sp, sp, 16
                ret
        "};
        let program = Program(vec![FuncDef {
            name: Id {
                st: SyntaxToken::default(),
                name: String::from("main"),
            },
            params: vec![],
            ret: Type::Int,
            body: vec![],
            ret_stmt: ReturnStmt(CompExpr(
                ArithExpr(
                    AtomExpr::Num(Num {
                        st: SyntaxToken::default(),
                        name: String::from("100"),
                    }),
                    vec![],
                ),
                None,
            )),
        }]);
        let mut cg = CodeGen::new(&program);
        let instrs = CodeGen::gen_code(cg.gen_program().unwrap());
        assert_eq!(expected_instrs, instrs);
    }
}

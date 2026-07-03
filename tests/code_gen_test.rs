use indoc::indoc;
use risc_v_compiler::{code_gen::CodeGen, parser::Parser, scanner::Scanner};
use serial_test::serial;
use std::fs;
use std::process::Command;

#[track_caller]
fn compile_and_run(input: &str) -> (i32, String) {
    // 1. Generate assembly string
    let mut sc = Scanner::new(input);
    let sts = sc.scan().expect("scan failed");
    let mut p = Parser::new(&sts);
    let program = p.parse().expect("parse failed");
    let mut cg = CodeGen::new(&program);
    let asm = CodeGen::gen_code(cg.gen_program().expect("code generation failed"));

    // 2. Write assembly to a temp file
    fs::write("/tmp/test.s", &asm)
        .unwrap_or_else(|e| panic!("failed to write assembly: {e}\n--- assembly ---\n{asm}"));

    // 3. Assemble
    let status = Command::new("riscv64-linux-gnu-as")
        .args([
            "-march=rv32im",
            "-mabi=ilp32",
            "-o",
            "/tmp/test.o",
            "/tmp/test.s",
        ])
        .status()
        .unwrap_or_else(|e| panic!("failed to run assembler: {e}\n--- assembly ---\n{asm}"));
    assert!(status.success(), "assembly failed:\n{asm}");

    // 4. Link
    let status = Command::new("riscv64-linux-gnu-ld")
        .args(["-m", "elf32lriscv", "-o", "/tmp/test", "/tmp/test.o"])
        .status()
        .unwrap_or_else(|e| panic!("failed to run linker: {e}\n--- assembly ---\n{asm}"));
    assert!(status.success(), "linking failed");

    // 5. Run and capture exit code
    (
        Command::new("qemu-riscv32")
            .arg("/tmp/test")
            .status()
            .unwrap_or_else(|e| panic!("failed to run qemu: {e}\n--- assembly ---\n{asm}"))
            .code()
            .unwrap_or_else(|| panic!("process killed by signal\n--- assembly ---\n{asm}")),
        asm,
    )
}

#[track_caller]
fn assert_asm_result(expected: i32, input: &str) {
    assert!(expected >= 0);
    assert!(expected <= 255);
    let (result, asm) = compile_and_run(input);
    if expected != result {
        panic!(
            "\nError! Result DON'T MATCH:\nExpected Result: {expected}\nComputed Result: {result}\n--- assembly ---\n{asm}"
        );
    }
}

#[test]
#[serial]
fn gen_return_num() {
    let input = indoc! {"
    fn main ( ) -> int {
        return 183 ;
    }
    "};
    let expected = 183;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_return_id() {
    let input = indoc! {"
    fn main ( ) -> int {
        let a: int := 90;
        return a ;
    }
    "};
    let expected = 90;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_group_expr() {
    let input = indoc! {"
    fn main ( ) -> int {
        let a: int := 255;
        return (a) ;
    }
    "};
    let expected = 255;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_func_call() {
    let input = indoc! {"
    fn another_main(a: int) -> int {return a ;}
    fn main ( ) -> int {
        let a: int := 42;
        return another_main(a) ;
    }
    "};
    let expected = 42;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_grt_num_id() {
    let input = indoc! {"
    fn main ( ) -> int {
        let a: int := 93;
        return 100 > a ;
    }
    "};
    let expected = 1;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_equality_id_num() {
    let input = indoc! {"
    fn main ( ) -> int {
        let a: int := 93;
        return a == 100 ;
    }
    "};
    let expected = 0;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_plus_id_num() {
    let input = indoc! {"
    fn main ( ) -> int {
        let a: int := 22;
        return a + 34 + 3 + 7;
    }
    "};
    let expected = 66;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_sub_id_id() {
    let input = indoc! {"
    fn main ( ) -> int {
        let a: int := 22;
        let b: int := 23;
        return a - b + 2;
    }
    "};
    let expected = 1;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_add_assoc() {
    let input = indoc! {"
    fn add_left_assoc (a: int, b: int, c: int) -> int {return (a + b) + c; }
    fn add_right_assoc (a: int, b: int, c: int) -> int {return a + (b + c); }
    fn main ( ) -> int {
        let a: int := 22;
        let b: int := 23;
        let c: int := 100;
        return add_left_assoc(a, b, c) == add_right_assoc(a,b,c);
    }
    "};
    let expected = 1;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_add_commut() {
    let input = indoc! {"
    fn main ( ) -> int {
        let a: int := 22;
        let b: int := 23;
        return a + b == b + a;
    }
    "};
    let expected = 1;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_add_inverse() {
    let input = indoc! {"
    fn inverse (a: int) -> int {return 0 - a; }
    fn main ( ) -> int {
        let a: int := 22;
        return inverse(a) + a == 0;
    }
    "};
    let expected = 1;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_sub_assoc() {
    let input = indoc! {"
    fn sub_left_assoc (a: int, b: int, c: int) -> int {return (a - b) - c; }
    fn sub_right_assoc (a: int, b: int, c: int)-> int {return a - (b - c); }
    fn main ( ) -> int {
        let a: int := 22;
        let b: int := 23;
        let c: int := 100;
        return sub_left_assoc(a, b, c) == sub_right_assoc(a,b,c);
    }
    "};
    let expected = 0;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_sub_inverse() {
    let input = indoc! {"
    fn inverse (a: int) -> int{return 0 - a; }
    fn main ( ) -> int {
        let a: int := 22;
        return inverse(a) + a == 0;
    }
    "};
    let expected = 1;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_is_under_3() {
    let input = indoc! {"
    fn grt_or_equal( a : int, b : int) -> int {
        let result: int := 0;
        if (a == b) { result := 1; }
        elif (a > b) { result := 1; }
        return result;
    }
    fn main ( ) -> int {
        return grt_or_equal(102, 102);
    }
    "};
    let expected = 1;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_max_two_param() {
    let input = indoc! {"
    fn max ( a : int, b : int) -> int {
        let result: int := 0;
        if (a > b) {result := a; }
        else {result := b; }
        return result;
    }
    fn main ( ) -> int {
        return max(102, 8);
    }
    "};
    let expected = 102;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_max_three_param() {
    let input = indoc! {"
    fn max ( a : int, b : int, c: int) -> int {
        let result: int := 0;
        if (a > b) {
            if (a > c) { result := a; }
            else { result := c; }
        }
        else {
            if (b > c) { result := b; }
            else { result := c; }
        }
        return result;
    }
    fn main ( ) -> int {
        return max(102, 8, 150);
    }
    "};
    let expected = 150;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_less_than() {
    let input = indoc! {"
    fn less_than ( a : int, b : int ) -> int {
        let result: int := 0;
        if (b > a) {
            result := 1;
        }
        return result;
    }
    fn main ( ) -> int {
        return less_than(1, 0);
    }
    "};
    let expected = 0;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_fib_10() {
    let input = indoc! {"
    fn fib ( n : int ) -> int {
        let result : int := 0 ;
        if ( n == 0 ) {
            result := 0 ;
        }
        elif ( n == 1 ) {
            result := 1 ;
        }
        else {
            result := fib ( n - 1 ) + fib ( n - 2 ) ;
        }
        return result ;
    }
    fn main ( ) -> int {
        return fib(10);
    }
    "};
    let expected = 55;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_while_mult_5() {
    let input = indoc! {"
    fn mult_5 ( n : int ) -> int {
        let i: int := 0;
        let result: int := 0;
        while (n - i > 0) {
            result := result + 5;
            i := i + 1; 
        }
        return result;
    }
    fn main ( ) -> int {
        let a: int := mult_5(6);
        let b: int := mult_5(5);
        let c: int := mult_5(9);
        return a + b + c;
    }
    "};
    let expected = 100;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_mult() {
    let input = indoc! {"
    fn multiply ( n : int , m : int ) -> int {
        let product : int := 0 ;
        if ( m == 0 ) { product := 0 ; }
        elif ( m > 0 ) {
            if ( n > 0 ) { product := multiply ( n , m - 1 ) + n ; }
            elif ( n == 0 ) { product := 0 ; }
            else { product := multiply ( n , m - 1 ) - n ; }
            }
        else {
            if ( n > 0 ) { product := multiply ( n , m + 1 ) - n ; }
            elif ( n == 0 ) { product := 0 ; }
            else { product := multiply ( n , m + 1 ) + n ; }
        }
        return product ;
    }
    fn main ( ) -> int {
        let a: int := multiply(5, 3);
        let b: int := multiply(5, 1);
        let c: int := multiply(5, 0);
        let d: int := multiply(35, 4);
        return a + b + c + d;
    }
    "};
    let expected = 160;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn gen_div() {
    let input = indoc! {"
    fn abs ( n : int ) -> int {
        let result : int := 0 ;
        if ( n > 0 ) { result := n ; }
        else { result := 0 - n ; }
        return result ;
    }
    fn divide ( n : int , m : int ) -> int {
        let quotient : int := 0 ;
        if ( abs ( m ) > abs ( n ) ) { quotient := 0 ; }
        else {
            if ( n > 0 ) {
                if ( m > 0 ) { quotient := divide ( n - m , m ) + 1 ; }
                else { quotient := divide ( n + m , m ) - 1 ; }
            }
            else {
                if ( m > 0 ) { quotient := divide ( n + m , m ) - 1 ; }
                else { quotient := divide ( n - m , m ) + 1 ; }
            }
        }
        return  quotient ;
    }

    fn main ( ) -> int {
        let a: int := divide(6, 3);
        let b: int := divide(20, 4);
        let c: int := divide(12, 3);
        return a + b + c;
    }
    "};
    let expected = 11;
    assert_asm_result(expected, input);
}

#[test]
#[serial]
fn parse_mult_iter() {
    let input = indoc! {"
    fn multiply ( n : int , m : int ) -> int {
        let result : int := 0 ;
        let i: int := m;
        while ( n > 0 ) {
            while ( i > 0 ) {
                result := result + 1 ;
                i := i - 1 ;
            }
            i := m;
            n := n - 1 ;
        }
        return result ;
    }
    fn main ( ) -> int {
        let a: int := multiply(5, 3);
        let b: int := multiply(5, 1);
        let c: int := multiply(5, 0);
        return a + b + c;
    }
    "};
    let expected = 20;
    assert_asm_result(expected, input);
}

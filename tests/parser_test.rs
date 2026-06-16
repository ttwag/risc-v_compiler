use risc_v_compiler::{
    parser::{ParseError, Parser},
    scanner::{ScanError, Scanner},
};

use indoc::indoc;
use pretty_assertions::assert_eq;

// The AST would always print one space between each token,
// so we must normalize the input to strip away excessive white space or newline
#[track_caller]
fn assert_ast(input: &str) {
    let mut s = Scanner::new(input);
    let sts = s.scan().unwrap();
    let mut p = Parser::new(&sts);
    let ast = p.parse().unwrap();
    assert_eq!(normalize(input), ast.to_string());
}

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn parse_params() {
    let input = indoc! {"
    fn no_param ( ) -> int { return 0 ; }
    fn one_param ( a : int ) -> int { return a ; }
    fn params ( a : int , b : int , c : int ) -> int { return c ; }
    "};
    assert_ast(input);
}

#[test]
fn parse_assign_stmt() {
    let input = indoc! {"
    fn assign ( a : int ) -> int {
        a := a + 1 ;
        return a ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_lets() {
    let input = indoc! {"
    fn lets ( ) -> int {
        let a : int := 52 + 3 ;
        let b : int := 2 + a - 1 ;
        let c : int := 2 > 0 ;
        return b ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_branch_if() {
    let input = indoc! {"
    fn branch ( flag : int ) -> int {
        let b : int := 0 ;
        if ( flag > 0 ) {
            b := 1 ;
        }
        return b ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_branch_elif() {
    let input = indoc! {"
    fn branch ( flag : int ) -> int {
        let b : int := 0 ;
        if ( flag > 0 ) {
            b := 1 ;
        }
        elif ( flag == 0 ) {
            b := 100 ;
        }
        return b ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_branch_else() {
    let input = indoc! {"
    fn branch ( flag : int ) -> int {
        let b : int := 0 ;
        if ( flag > 0 ) {
            b := 1 ;
        }
        elif ( flag == 0 ) {
            b := 100 ;
        }
        else {
            b := 1000 ;
        }
        return b ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_loop() {
    let input = indoc! {"
    fn loop ( flag : int ) -> int {
        while ( flag > 0 ) {
            flag := flag - 1 ;
        }
        return b ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_nested_loop() {
    let input = indoc! {"
    fn example ( n : int , m : int ) -> int {
        let result : int := 0 ;
        while ( n > 0 ) {
            while ( m > 0 ) {
                result := result + 1 ;
                m := m - 1 ;
            }
            n := n - 1 ;
        }
        return result ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_nested_branch() {
    let input = indoc! {"
    fn example ( n : int ) -> int {
        let result : int := 0 ;
        while ( n > 0 ) {
            if ( n == 1 ) {
                result := result + 1 ;
            }
            n := n - 1 ;
        }
        return result ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_grt() {
    let input = indoc! {"
    fn grt ( left : int , right : int ) -> int {
        return left > right ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_equality() {
    let input = indoc! {"
    fn equality ( left : int , right : int ) -> int {
        return left == right ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_plus() {
    let input = indoc! {"
    fn example ( prime : int ) -> int {
        return 1 + 2 + 3 + prime ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_minus() {
    let input = indoc! {"
    fn example ( prime : int ) -> int {
        return 1 - 2 - 3 - prime ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_group() {
    let input = indoc! {"
    fn example ( ) -> int {
        return 1 > ( 5 + 6 ) ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_func_call_params() {
    let input = indoc! {"
    fn zero ( ) -> int { return 0 ; }
    fn self  ( a : int ) -> int { return a ; }
    fn third  ( a : int , b : int , c : int ) -> int { return c ; }
    fn example ( ) -> int {
        return zero ( ) + self ( 1 ) + third ( 100 , 50 , 20 ) ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_func_call_nested() {
    let input = indoc! {"
    fn self  ( a : int ) -> int { return a ; }
    fn example ( ) -> int {
        return self ( self ( 1 ) ) ;
    }
    "};
    assert_ast(input);
}

#[test]
fn parse_fib() {
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
    "};
    assert_ast(input);
}

#[test]
fn parse_mult() {
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
    "};
    assert_ast(input);
}

#[test]
fn parse_div() {
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
    "};
    assert_ast(input);
}

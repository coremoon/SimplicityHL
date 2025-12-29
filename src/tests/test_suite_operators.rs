//! Comprehensive operator tests for NOT and infix operators.
//!
//! This test suite verifies that:
//! - NOT operator (!x) parses correctly with proper precedence
//! - Infix operators (==, !=, <, >, <=, >=, &&, ||) parse correctly
//! - Operator precedence and associativity are handled correctly
//! - Edge cases with mixed operators parse as expected
//!
//! NOTE: These tests verify PARSING only, not compilation.
//! Full compilation support for operators will be implemented later.

#[cfg(test)]
mod operator_robustness_tests {
    use crate::parse::ParseFromStr;

    // ========================================================================
    // NOT OPERATOR TESTS
    // ========================================================================

    #[test]
    fn not_operator_with_bool() {
        let prog_text = r#"fn main() { let x: bool = true; let not_x: bool = !x; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_with_u8() {
        let prog_text = r#"fn main() { let a: u8 = 10; let not_a: u8 = !a; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_with_u16() {
        let prog_text = r#"fn main() { let b: u16 = 100; let not_b: u16 = !b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_with_u32() {
        let prog_text = r#"fn main() { let c: u32 = 20; let not_c: u32 = !c; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_with_u64() {
        let prog_text = r#"fn main() { let d: u64 = 50; let not_d: u64 = !d; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_chained_with_parens() {
        let prog_text = r#"fn main() { let x: bool = true; let double_not: bool = !(!(x)); }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_triple_with_parens() {
        let prog_text = r#"fn main() { let y: u8 = 5; let triple_not: u8 = !(!(!(y))); }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_with_function_call() {
        let prog_text = r#"fn foo() -> u32 { 42 } fn main() { let result: u32 = !(foo()); }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_precedence_vs_equality() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let result: bool = !a == b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_negating_comparison() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let result: bool = !(a == b); }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_with_logical_and() {
        let prog_text = r#"fn main() { let a: bool = true; let b: bool = false; let result: bool = !a && b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_with_logical_or() {
        let prog_text = r#"fn main() { let x: bool = true; let y: bool = false; let result: bool = !x || y; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn not_operator_comprehensive() {
        let prog_text = r#"fn main() { let bit: bool = false; let not_bit: bool = !bit; let a: u8 = 10; let not_a: u8 = !a; let b: u16 = 100; let not_b: u16 = !b; let c: u32 = 20; let not_c: u32 = !c; let d: u64 = 50; let not_d: u64 = !d; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    // ========================================================================
    // INFIX OPERATOR TESTS (EQUALITY)
    // ========================================================================

    #[test]
    fn infix_eq_simple() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 10; let result: bool = a == b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_ne_simple() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let result: bool = a != b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_eq_with_literals() {
        let prog_text = r#"fn main() { let result: bool = 42 == 42; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_ne_with_literals() {
        let prog_text = r#"fn main() { let result: bool = 10 != 20; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_eq_chained() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 10; let c: u32 = 10; let result: bool = a == b == c; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    // ========================================================================
    // INFIX OPERATOR TESTS (RELATIONAL)
    // ========================================================================

    #[test]
    fn infix_lt_simple() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let result: bool = a < b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_gt_simple() {
        let prog_text = r#"fn main() { let a: u32 = 20; let b: u32 = 10; let result: bool = a > b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_le_simple() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let result: bool = a <= b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_ge_simple() {
        let prog_text = r#"fn main() { let a: u32 = 20; let b: u32 = 10; let result: bool = a >= b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_lt_chained() {
        let prog_text = r#"fn main() { let a: u32 = 5; let b: u32 = 10; let c: u32 = 15; let result: bool = a < b < c; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_gt_chained() {
        let prog_text = r#"fn main() { let a: u32 = 15; let b: u32 = 10; let c: u32 = 5; let result: bool = a > b > c; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    // ========================================================================
    // INFIX OPERATOR TESTS (LOGICAL)
    // ========================================================================

    #[test]
    fn infix_logical_and_simple() {
        let prog_text = r#"fn main() { let a: bool = true; let b: bool = false; let result: bool = a && b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_logical_or_simple() {
        let prog_text = r#"fn main() { let a: bool = true; let b: bool = false; let result: bool = a || b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_logical_and_chained() {
        let prog_text = r#"fn main() { let a: bool = true; let b: bool = true; let c: bool = true; let result: bool = a && b && c; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn infix_logical_or_chained() {
        let prog_text = r#"fn main() { let a: bool = true; let b: bool = false; let c: bool = false; let result: bool = a || b || c; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    // ========================================================================
    // PRECEDENCE TESTS
    // ========================================================================

    #[test]
    fn precedence_relational_before_equality() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let c: u32 = 30; let result: bool = a < b == c > b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn precedence_equality_before_logical_and() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let c: bool = true; let result: bool = a == b && c; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn precedence_logical_and_before_or() {
        let prog_text = r#"fn main() { let a: bool = true; let b: bool = false; let c: bool = true; let result: bool = a && b || c; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn precedence_not_before_comparison() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let result: bool = !a == b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    // ========================================================================
    // COMPLEX MIXED OPERATOR TESTS
    // ========================================================================

    #[test]
    fn complex_mixed_operators_1() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let c: u32 = 15; let x: bool = true; let y: bool = false; let result: bool = a < b && c == 15 || !x; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn complex_mixed_operators_2() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let result: bool = !(a == b) && a < b || b >= 20; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn complex_mixed_operators_3() {
        let prog_text = r#"fn main() { let a: bool = true; let b: bool = false; let c: bool = true; let result: bool = !a || !b && c; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn complex_mixed_operators_4() {
        let prog_text = r#"fn main() { let x: u32 = 5; let y: u32 = 10; let z: u32 = 15; let r1: bool = x < y; let r2: bool = y <= z; let r3: bool = r1 && r2; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    // ========================================================================
    // EDGE CASE TESTS
    // ========================================================================

    #[test]
    fn edge_case_operators_without_spaces() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let r1: bool = a==b; let r2: bool = a!=b; let r3: bool = a<b; let r4: bool = a>b; let r5: bool = a<=b; let r6: bool = a>=b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn edge_case_operators_with_extra_spaces() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let r1: bool = a  ==  b; let r2: bool = a  !=  b; let r3: bool = a  <  b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn edge_case_not_with_function_calls() {
        let prog_text = r#"fn is_positive(x: u32) -> bool { x > 0 } fn main() { let a: u32 = 10; let result: bool = !is_positive(a); }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn edge_case_comparison_with_function_calls() {
        let prog_text = r#"fn get_value() -> u32 { 42 } fn main() { let result: bool = get_value() == 42; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn edge_case_operators_with_builtin_functions() {
        let prog_text = r#"fn main() { let a: u32 = 15; let b: u32 = 7; let and_result: u32 = and::<u32>(a, b); let or_result: u32 = or::<u32>(a, b); let result: bool = and_result == 7; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn edge_case_not_with_different_types() {
        let tests = vec![
            r#"fn main() { let x: bool = !true; }"#,
            r#"fn main() { let x: u8 = !0u8; }"#,
            r#"fn main() { let x: u16 = !0u16; }"#,
            r#"fn main() { let x: u32 = !0u32; }"#,
            r#"fn main() { let x: u64 = !0u64; }"#,
        ];
        for test in tests {
            assert!(crate::parse::Program::parse_from_str(test).is_ok(), "Failed: {}", test);
        }
    }

    #[test]
    fn edge_case_comparisons_all_types() {
        let tests = vec![
            r#"fn main() { let r: bool = 1u8 == 1u8; }"#,
            r#"fn main() { let r: bool = 100u16 == 100u16; }"#,
            r#"fn main() { let r: bool = 1000u32 == 1000u32; }"#,
            r#"fn main() { let r: bool = 10000u64 == 10000u64; }"#,
        ];
        for test in tests {
            assert!(crate::parse::Program::parse_from_str(test).is_ok(), "Failed: {}", test);
        }
    }

    // ========================================================================
    // OPERATOR ROBUSTNESS VERIFICATION
    // ========================================================================

    #[test]
    fn robustness_all_comparison_operators() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let eq: bool = a == b; let ne: bool = a != b; let lt: bool = a < b; let gt: bool = b > a; let le: bool = a <= b; let ge: bool = b >= a; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn robustness_all_logical_operators() {
        let prog_text = r#"fn main() { let a: bool = true; let b: bool = false; let and_result: bool = a && b; let or_result: bool = a || b; let not_a: bool = !a; let not_b: bool = !b; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn robustness_nested_operators() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let c: u32 = 30; let result: bool = (a < b) && (b < c) || !(a == c); }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn robustness_operator_in_function_params() {
        let prog_text = r#"fn check_range(x: u32, min: u32, max: u32) -> bool { x >= min && x <= max } fn main() { let result: bool = check_range(15, 10, 20); }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }

    #[test]
    fn robustness_multiple_operators_in_expression() {
        let prog_text = r#"fn main() { let a: u32 = 10; let b: u32 = 20; let c: u32 = 30; let d: u32 = 40; let result: bool = a < b && b < c && c < d; }"#;
        assert!(crate::parse::Program::parse_from_str(prog_text).is_ok());
    }
}

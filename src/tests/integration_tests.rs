//! Integration tests for SimplicityHL programs.
//!
//! These tests verify that SimplicityHL can parse, compile, and execute
//! real-world contract examples from the examples/ directory.

use super::TestCase;
use crate::*;

#[test]
fn cat() {
    TestCase::program_file("./examples/cat.simf")
        .with_witness_values(WitnessValues::default())
        .assert_run_success();
}

#[test]
fn ctv() {
    TestCase::program_file("./examples/ctv.simf")
        .with_witness_values(WitnessValues::default())
        .assert_run_success();
}

#[test]
fn regression_153() {
    TestCase::program_file("./examples/array_fold_2n.simf")
        .with_witness_values(WitnessValues::default())
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn sighash_non_interactive_fee_bump() {
    let mut t = TestCase::program_file("./examples/non_interactive_fee_bump.simf")
        .with_witness_file("./examples/non_interactive_fee_bump.wit");
    t.sequence = elements::Sequence::ENABLE_LOCKTIME_NO_RBF;
    t.lock_time = elements::LockTime::from_time(1734967235 + 600).unwrap();
    t.include_fee_output = true;
    t.assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn escrow_with_delay_timeout() {
    TestCase::program_file("./examples/escrow_with_delay.simf")
        .with_sequence(1000)
        .print_sighash_all()
        .with_witness_file("./examples/escrow_with_delay.timeout.wit")
        .assert_run_success();
}

#[test]
fn hash_loop() {
    TestCase::program_file("./examples/hash_loop.simf")
        .with_witness_values(WitnessValues::default())
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn hodl_vault() {
    TestCase::program_file("./examples/hodl_vault.simf")
        .with_lock_time(1000)
        .print_sighash_all()
        .with_witness_file("./examples/hodl_vault.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn htlc_complete() {
    TestCase::program_file("./examples/htlc.simf")
        .print_sighash_all()
        .with_witness_file("./examples/htlc.complete.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn last_will_inherit() {
    TestCase::program_file("./examples/last_will.simf")
        .with_sequence(25920)
        .print_sighash_all()
        .with_witness_file("./examples/last_will.inherit.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn p2ms() {
    TestCase::program_file("./examples/p2ms.simf")
        .print_sighash_all()
        .with_witness_file("./examples/p2ms.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn p2pk() {
    TestCase::template_file("./examples/p2pk.simf")
        .with_argument_file("./examples/p2pk.args")
        .print_sighash_all()
        .with_witness_file("./examples/p2pk.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn p2pkh() {
    TestCase::program_file("./examples/p2pkh.simf")
        .print_sighash_all()
        .with_witness_file("./examples/p2pkh.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn presigned_vault_complete() {
    TestCase::program_file("./examples/presigned_vault.simf")
        .with_sequence(1000)
        .print_sighash_all()
        .with_witness_file("./examples/presigned_vault.complete.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn sighash_all_anyonecanpay() {
    TestCase::program_file("./examples/sighash_all_anyonecanpay.simf")
        .with_witness_file("./examples/sighash_all_anyonecanpay.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn sighash_all_anyprevout() {
    TestCase::program_file("./examples/sighash_all_anyprevout.simf")
        .with_witness_file("./examples/sighash_all_anyprevout.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn sighash_all_anyprevoutanyscript() {
    TestCase::program_file("./examples/sighash_all_anyprevoutanyscript.simf")
        .with_witness_file("./examples/sighash_all_anyprevoutanyscript.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn sighash_none() {
    TestCase::program_file("./examples/sighash_none.simf")
        .with_witness_file("./examples/sighash_none.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn sighash_single() {
    TestCase::program_file("./examples/sighash_single.simf")
        .with_witness_file("./examples/sighash_single.wit")
        .assert_run_success();
}

#[test]
#[cfg(feature = "serde")]
fn transfer_with_timeout_transfer() {
    TestCase::program_file("./examples/transfer_with_timeout.simf")
        .print_sighash_all()
        .with_witness_file("./examples/transfer_with_timeout.transfer.wit")
        .assert_run_success();
}

#[test]
fn redefined_variable() {
    let prog_text = r#"fn main() {
    let beefbabe: (u16, u16) = (0xbeef, 0xbabe);
    let beefbabe: u32 = <(u16, u16)>::into(beefbabe);
}
"#;
    TestCase::program_text(std::borrow::Cow::Borrowed(prog_text))
        .with_witness_values(WitnessValues::default())
        .assert_run_success();
}

#[test]
fn empty_function_body_nonempty_return() {
    let prog_text = r#"fn my_true() -> bool {
    // function body is empty, although function must return `bool`
}

fn main() {
    assert!(my_true());
}
"#;
    match SatisfiedProgram::new(
        prog_text,
        Arguments::default(),
        WitnessValues::default(),
        false,
    ) {
        Ok(_) => panic!("Accepted faulty program"),
        Err(error) => {
            assert!(
                error.contains("Expected expression of type `bool`, found type `()`"),
                "Unexpected error: {error}",
            );
        }
    }
}

#[test]
fn fuzz_regression_2() {
    parse::Program::parse_from_str("fn dbggscas(h: bool, asyxhaaaa: a) {\nfalse}\n\n").unwrap();
}

#[test]
#[ignore]
fn fuzz_slow_unit_1() {
    parse::Program::parse_from_str("fn fnnfn(MMet:(((sssss,((((((sssss,ssssss,ss,((((((sssss,ss,((((((sssss,ssssss,ss,((((((sssss,ssssss,((((((sssss,sssssssss,(((((((sssss,sssssssss,(((((ssss,((((((sssss,sssssssss,(((((((sssss,ssss,((((((sssss,ss,((((((sssss,ssssss,ss,((((((sssss,ssssss,((((((sssss,sssssssss,(((((((sssss,sssssssss,(((((ssss,((((((sssss,sssssssss,(((((((sssss,sssssssssssss,(((((((((((u|(").unwrap_err();
}

#[test]
fn type_alias() {
    let prog_text = r#"type MyAlias = u32;

fn main() {
    let x: MyAlias = 32;
    assert!(jet::eq_32(x, 32));
}"#;
    TestCase::program_text(std::borrow::Cow::Borrowed(prog_text))
        .with_witness_values(WitnessValues::default())
        .assert_run_success();
}

#[test]
fn type_error_regression() {
    let prog_text = r#"fn main() {
    let (a, b): (u32, u32) = (0, 1);
    assert!(jet::eq_32(a, 0));

    let (c, d): (u32, u32) = (2, 3);
    assert!(jet::eq_32(c, 2));
    assert!(jet::eq_32(d, 3));
}"#;
    TestCase::program_text(std::borrow::Cow::Borrowed(prog_text))
        .with_witness_values(WitnessValues::default())
        .assert_run_success();
}

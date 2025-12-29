//! TestCase struct and helper implementations for test infrastructure.

use base64::display::Base64Display;
use base64::engine::general_purpose::STANDARD;
use simplicity::BitMachine;
use std::borrow::Cow;
use std::path::Path;

use crate::*;

/// Test case infrastructure for running SimplicityHL programs.
pub struct TestCase<T> {
    pub program: T,
    pub lock_time: elements::LockTime,
    pub sequence: elements::Sequence,
    pub include_fee_output: bool,
}

impl TestCase<TemplateProgram> {
    pub fn template_file<P: AsRef<Path>>(program_file_path: P) -> Self {
        let program_text = std::fs::read_to_string(program_file_path).unwrap();
        Self::template_text(Cow::Owned(program_text))
    }

    pub fn template_text(program_text: Cow<str>) -> Self {
        let program = match TemplateProgram::new(program_text.as_ref()) {
            Ok(x) => x,
            Err(error) => panic!("{error}"),
        };
        Self {
            program,
            lock_time: elements::LockTime::ZERO,
            sequence: elements::Sequence::MAX,
            include_fee_output: false,
        }
    }

    #[cfg(feature = "serde")]
    pub fn with_argument_file<P: AsRef<Path>>(
        self,
        arguments_file_path: P,
    ) -> TestCase<CompiledProgram> {
        let arguments_text = std::fs::read_to_string(arguments_file_path).unwrap();
        let arguments = match serde_json::from_str::<Arguments>(&arguments_text) {
            Ok(x) => x,
            Err(error) => panic!("{error}"),
        };
        self.with_arguments(arguments)
    }

    pub fn with_arguments(self, arguments: Arguments) -> TestCase<CompiledProgram> {
        let program = match self.program.instantiate(arguments, true) {
            Ok(x) => x,
            Err(error) => panic!("{error}"),
        };
        TestCase {
            program,
            lock_time: self.lock_time,
            sequence: self.sequence,
            include_fee_output: self.include_fee_output,
        }
    }
}

impl TestCase<CompiledProgram> {
    pub fn program_file<P: AsRef<Path>>(program_file_path: P) -> Self {
        TestCase::<TemplateProgram>::template_file(program_file_path)
            .with_arguments(Arguments::default())
    }

    pub fn program_text(program_text: Cow<str>) -> Self {
        TestCase::<TemplateProgram>::template_text(program_text)
            .with_arguments(Arguments::default())
    }

    #[cfg(feature = "serde")]
    pub fn with_witness_file<P: AsRef<Path>>(
        self,
        witness_file_path: P,
    ) -> TestCase<SatisfiedProgram> {
        let witness_text = std::fs::read_to_string(witness_file_path).unwrap();
        let witness_values = match serde_json::from_str::<WitnessValues>(&witness_text) {
            Ok(x) => x,
            Err(error) => panic!("{error}"),
        };
        self.with_witness_values(witness_values)
    }

    pub fn with_witness_values(
        self,
        witness_values: WitnessValues,
    ) -> TestCase<SatisfiedProgram> {
        let program = match self.program.satisfy(witness_values) {
            Ok(x) => x,
            Err(error) => panic!("{error}"),
        };
        TestCase {
            program,
            lock_time: self.lock_time,
            sequence: self.sequence,
            include_fee_output: self.include_fee_output,
        }
    }
}

impl<T> TestCase<T> {
    #[allow(dead_code)]
    pub fn with_lock_time(mut self, height: u32) -> Self {
        let height = elements::locktime::Height::from_consensus(height).unwrap();
        self.lock_time = elements::LockTime::Blocks(height);
        if self.sequence.is_final() {
            self.sequence = elements::Sequence::ENABLE_LOCKTIME_NO_RBF;
        }
        self
    }

    #[allow(dead_code)]
    pub fn with_sequence(mut self, distance: u16) -> Self {
        self.sequence = elements::Sequence::from_height(distance);
        self
    }

    #[allow(dead_code)]
    pub fn print_sighash_all(self) -> Self {
        let env = dummy_env::dummy_with(self.lock_time, self.sequence, self.include_fee_output);
        dbg!(env.c_tx_env().sighash_all());
        self
    }
}

impl TestCase<SatisfiedProgram> {
    #[allow(dead_code)]
    pub fn print_encoding(self) -> Self {
        let (program_bytes, witness_bytes) = self.program.redeem().to_vec_with_witness();
        println!(
            "Program:\n{}",
            Base64Display::new(&program_bytes, &STANDARD)
        );
        println!(
            "Witness:\n{}",
            Base64Display::new(&witness_bytes, &STANDARD)
        );
        self
    }

    fn run(self) -> Result<(), simplicity::bit_machine::ExecutionError> {
        let env = dummy_env::dummy_with(self.lock_time, self.sequence, self.include_fee_output);
        let pruned = self.program.redeem().prune(&env)?;
        let mut mac = BitMachine::for_program(&pruned)
            .expect("program should be within reasonable bounds");
        mac.exec(&pruned, &env).map(|_| ())
    }

    pub fn assert_run_success(self) {
        match self.run() {
            Ok(()) => {}
            Err(error) => panic!("Unexpected error: {error}"),
        }
    }
}

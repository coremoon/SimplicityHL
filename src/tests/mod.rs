//! Integration tests module for SimplicityHL.
//!
//! Contains:
//! - `test_case`: TestCase struct and test infrastructure
//! - `test_suite_operators`: Comprehensive operator robustness tests
//! - `integration_tests`: Real-world contract examples

#[cfg(test)]
pub mod test_case;

#[cfg(test)]
pub mod test_suite_operators;

#[cfg(test)]
pub mod integration_tests;

// Re-export TestCase for use in other crates (especially compile/builtins.rs)
#[cfg(test)]
pub use test_case::TestCase;

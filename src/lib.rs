//! Library for parsing and compiling SimplicityHL

pub mod array;
pub mod ast;
pub mod compile;
pub mod debug;
pub mod dummy_env;
pub mod error;
pub mod jet;
pub mod named;
pub mod num;
pub mod parse;
pub mod pattern;
#[cfg(feature = "serde")]
mod serde;
pub mod str;
pub mod tests;
pub mod tracker;
pub mod types;
pub mod value;
mod witness;

use std::sync::Arc;

use simplicity::jet::elements::ElementsEnv;
use simplicity::{jet::Elements, CommitNode, RedeemNode};

pub extern crate either;
pub extern crate simplicity;
pub use simplicity::elements;

use crate::debug::DebugSymbols;
use crate::error::WithFile;
use crate::parse::ParseFromStr;
pub use crate::types::ResolvedType;
pub use crate::value::Value;
pub use crate::witness::{Arguments, Parameters, WitnessTypes, WitnessValues};

/// The template of a SimplicityHL program.
///
/// A template has parameterized values that need to be supplied with arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemplateProgram {
    simfony: ast::Program,
    file: Arc<str>,
}

impl TemplateProgram {
    /// Parse the template of a SimplicityHL program.
    ///
    /// ## Errors
    ///
    /// The string is not a valid SimplicityHL program.
    pub fn new<Str: Into<Arc<str>>>(s: Str) -> Result<Self, String> {
        let file = s.into();
        let parse_program = parse::Program::parse_from_str(&file)?;
        let ast_program = ast::Program::analyze(&parse_program).with_file(Arc::clone(&file))?;
        Ok(Self {
            simfony: ast_program,
            file,
        })
    }

    /// Access the parameters of the program.
    pub fn parameters(&self) -> &Parameters {
        self.simfony.parameters()
    }

    /// Instantiate the template program with the given `arguments`.
    ///
    /// ## Errors
    ///
    /// The arguments are not consistent with the parameters of the program.
    /// Use [`TemplateProgram::parameters`] to see which parameters the program has.
    pub fn instantiate(
        &self,
        arguments: Arguments,
        include_debug_symbols: bool,
    ) -> Result<CompiledProgram, String> {
        arguments
            .is_consistent(self.simfony.parameters())
            .map_err(|error| error.to_string())?;

        let commit = self
            .simfony
            .compile(arguments, include_debug_symbols)
            .with_file(Arc::clone(&self.file))?;

        Ok(CompiledProgram {
            debug_symbols: self.simfony.debug_symbols(self.file.as_ref()),
            simplicity: commit,
            witness_types: self.simfony.witness_types().shallow_clone(),
        })
    }
}

/// A SimplicityHL program, compiled to Simplicity.
#[derive(Clone, Debug)]
pub struct CompiledProgram {
    simplicity: Arc<named::CommitNode<Elements>>,
    witness_types: WitnessTypes,
    debug_symbols: DebugSymbols,
}

impl CompiledProgram {
    /// Parse and compile a SimplicityHL program from the given string.
    ///
    /// ## See
    ///
    /// - [`TemplateProgram::new`]
    /// - [`TemplateProgram::instantiate`]
    pub fn new<Str: Into<Arc<str>>>(
        s: Str,
        arguments: Arguments,
        include_debug_symbols: bool,
    ) -> Result<Self, String> {
        TemplateProgram::new(s)
            .and_then(|template| template.instantiate(arguments, include_debug_symbols))
    }

    /// Access the debug symbols for the Simplicity target code.
    pub fn debug_symbols(&self) -> &DebugSymbols {
        &self.debug_symbols
    }

    /// Access the Simplicity target code, without witness data.
    pub fn commit(&self) -> Arc<CommitNode<Elements>> {
        named::forget_names(&self.simplicity)
    }

    /// Satisfy the SimplicityHL program with the given `witness_values`.
    ///
    /// ## Errors
    ///
    /// - Witness values have a different type than declared in the SimplicityHL program.
    /// - There are missing witness values.
    pub fn satisfy(&self, witness_values: WitnessValues) -> Result<SatisfiedProgram, String> {
        self.satisfy_with_env(witness_values, None)
    }

    /// Satisfy the SimplicityHL program with the given `witness_values`.
    /// If `env` is `None`, the program is not pruned, otherwise it is pruned with the given environment.
    ///
    /// ## Errors
    ///
    /// - Witness values have a different type than declared in the SimplicityHL program.
    /// - There are missing witness values.
    pub fn satisfy_with_env(
        &self,
        witness_values: WitnessValues,
        env: Option<&ElementsEnv<Arc<elements::Transaction>>>,
    ) -> Result<SatisfiedProgram, String> {
        witness_values
            .is_consistent(&self.witness_types)
            .map_err(|e| e.to_string())?;

        let mut simplicity_redeem = named::populate_witnesses(&self.simplicity, witness_values)?;
        if let Some(env) = env {
            simplicity_redeem = simplicity_redeem.prune(env).map_err(|e| e.to_string())?;
        }
        Ok(SatisfiedProgram {
            simplicity: simplicity_redeem,
            debug_symbols: self.debug_symbols.clone(),
        })
    }
}

/// A SimplicityHL program, compiled to Simplicity and satisfied with witness data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SatisfiedProgram {
    simplicity: Arc<RedeemNode<Elements>>,
    debug_symbols: DebugSymbols,
}

impl SatisfiedProgram {
    /// Parse, compile and satisfy a SimplicityHL program from the given string.
    ///
    /// ## See
    ///
    /// - [`TemplateProgram::new`]
    /// - [`TemplateProgram::instantiate`]
    /// - [`CompiledProgram::satisfy`]
    pub fn new<Str: Into<Arc<str>>>(
        s: Str,
        arguments: Arguments,
        witness_values: WitnessValues,
        include_debug_symbols: bool,
    ) -> Result<Self, String> {
        let compiled = CompiledProgram::new(s, arguments, include_debug_symbols)?;
        compiled.satisfy(witness_values)
    }

    /// Access the Simplicity target code, including witness data.
    pub fn redeem(&self) -> &Arc<RedeemNode<Elements>> {
        &self.simplicity
    }

    /// Access the debug symbols for the Simplicity target code.
    pub fn debug_symbols(&self) -> &DebugSymbols {
        &self.debug_symbols
    }
}

/// Recursively implement [`PartialEq`], [`Eq`] and [`std::hash::Hash`]
/// using selected members of a given type. The type must have a getter
/// method for each selected member.
#[macro_export]
macro_rules! impl_eq_hash {
    ($ty: ident; $($member: ident),*) => {
        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                true $(&& self.$member() == other.$member())*
            }
        }

        impl Eq for $ty {}

        impl std::hash::Hash for $ty {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                $(self.$member().hash(state);)*
            }
        }
    };
}

/// Helper trait for implementing [`arbitrary::Arbitrary`] for recursive structures.
///
/// [`ArbitraryRec::arbitrary_rec`] allows the caller to set a budget that is decreased every time
/// the generated structure gets deeper. The maximum depth of the generated structure is equal to
/// the initial budget. The budget prevents the generated structure from becoming too deep, which
/// could cause issues in the code that processes these structures.
///
/// <https://github.com/rust-fuzz/arbitrary/issues/78>
#[cfg(feature = "arbitrary")]
trait ArbitraryRec: Sized {
    /// Generate a recursive structure from unstructured data.
    ///
    /// Generate leaves or parents when the budget is positive.
    /// Generate only leaves when the budget is zero.
    ///
    /// ## Implementation
    ///
    /// Recursive calls of [`arbitrary_rec`] must decrease the budget by one.
    fn arbitrary_rec(u: &mut arbitrary::Unstructured, budget: usize) -> arbitrary::Result<Self>;
}

/// Helper trait for implementing [`arbitrary::Arbitrary`] for typed structures.
///
/// [`arbitrary::Arbitrary`] is intended to produce well-formed values.
/// Structures with an internal type should be generated in a well-typed fashion.
///
/// [`arbitrary::Arbitrary`] can be implemented for a typed structure as follows:
/// 1. Generate the type via [`arbitrary::Arbitrary`].
/// 2. Generate the structure via [`ArbitraryOfType::arbitrary_of_type`].
#[cfg(feature = "arbitrary")]
pub trait ArbitraryOfType: Sized {
    /// Internal type of the structure.
    type Type;

    /// Generate a structure of the given type.
    fn arbitrary_of_type(
        u: &mut arbitrary::Unstructured,
        ty: &Self::Type,
    ) -> arbitrary::Result<Self>;
}

// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/codegen.rs

use std::io;

use crate::{
    DERIVES, GENERATORS,
    derives::Derive,
    generators::Generator,
    output::{Output, RawOutput},
    parse::attr::{AttrLocation, AttrPart},
    schema::Schema,
};

/// Runner trait.
///
/// This is the super-trait of [`Derive`] and [`Generator`].
pub trait Runner {
    fn file_path(&self) -> &'static str;

    /// Run the runner on the schema.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema breaks a rule the runner relies on.
    fn run(&self, schema: &Schema) -> io::Result<Vec<Output>>;
}

/// Get all runners (generators and derives).
#[must_use]
pub fn get_runners() -> Vec<GeneratorOrDerive> {
    GENERATORS
        .iter()
        .map(|&generator| GeneratorOrDerive::Generator(generator))
        .chain(DERIVES.iter().map(|&derive| GeneratorOrDerive::Derive(derive)))
        .collect()
}

/// A `Generator` or a `Derive`.
///
/// Provides a single interface for running either.
#[derive(Clone, Copy)]
pub enum GeneratorOrDerive {
    Generator(&'static (dyn Generator + Sync)),
    Derive(&'static (dyn Derive + Sync)),
}

impl GeneratorOrDerive {
    /// Get the names of the attributes this [`Generator`] or [`Derive`] owns.
    #[must_use]
    pub fn attrs(self) -> &'static [&'static str] {
        match self {
            Self::Generator(generator) => generator.attrs(),
            Self::Derive(derive) => derive.attrs(),
        }
    }

    /// Parse an attribute this [`Generator`] or [`Derive`] owns.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute is not legal in this position.
    pub fn parse_attr(
        self,
        attr_name: &str,
        location: AttrLocation<'_>,
        part: AttrPart<'_>,
    ) -> io::Result<()> {
        match self {
            Self::Generator(generator) => generator.parse_attr(attr_name, location, part),
            Self::Derive(derive) => derive.parse_attr(attr_name, location, part),
        }
    }

    /// Execute `prepare` method on a [`Generator`] or [`Derive`].
    pub fn prepare(self, schema: &mut Schema) {
        match self {
            Self::Generator(generator) => generator.prepare(schema),
            Self::Derive(derive) => derive.prepare(schema),
        }
    }

    /// Run the [`Generator`] or [`Derive`].
    ///
    /// # Errors
    ///
    /// Returns an error if the schema breaks a rule the runner relies on, or
    /// an output does not parse or cannot be formatted.
    pub fn run(self, schema: &Schema) -> io::Result<Vec<RawOutput>> {
        let (runner_path, outputs) = match self {
            Self::Generator(generator) => (generator.file_path(), generator.run(schema)?),
            Self::Derive(derive) => (derive.file_path(), derive.run(schema)?),
        };
        outputs.into_iter().map(|output| output.into_raw(runner_path)).collect()
    }
}

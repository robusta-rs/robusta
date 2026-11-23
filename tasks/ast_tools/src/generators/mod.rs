// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/generators/mod.rs

use std::io;

use crate::{
    codegen::Runner,
    invalid,
    output::Output,
    parse::attr::{AttrLocation, AttrPart},
    schema::Schema,
};

mod assert_layouts;
mod ast_builder;
mod ast_kind;
mod formatter;
mod get_id;
mod inherit_variants;
mod visit;

pub use assert_layouts::AssertLayoutsGenerator;
pub use ast_builder::AstBuilderGenerator;
pub use ast_kind::AstKindGenerator;
pub use formatter::{FormatterAstNodesGenerator, FormatterFormatGenerator};
pub use get_id::GetIdGenerator;
pub use inherit_variants::InheritVariantsGenerator;
pub use visit::VisitGenerator;

/// Trait to define a generator.
pub trait Generator: Runner {
    /// Get the names of the attributes this generator owns.
    fn attrs(&self) -> &[&'static str] {
        &[]
    }

    /// Parse an attribute this generator owns.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute is not legal in this position.
    fn parse_attr(
        &self,
        attr_name: &str,
        location: AttrLocation<'_>,
        _part: AttrPart<'_>,
    ) -> io::Result<()> {
        Err(invalid(format!("{location}: bad #[{attr_name}]")))
    }

    /// Prepare for generation, modifying schema.
    ///
    /// Runs before any `generate` or `derive` method runs.
    fn prepare(&self, _schema: &mut Schema) {}

    /// Generate outputs.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema breaks a rule the generator relies on.
    fn generate(&self, schema: &Schema) -> io::Result<Vec<Output>>;
}

/// Macro to implement [`Runner`] for a [`Generator`].
///
/// Must be used on every [`Generator`].
macro_rules! define_generator {
    ($ident:ident) => {
        const _: () = {
            use std::io;

            use $crate::{codegen::Runner, output::Output, schema::Schema};

            impl Runner for $ident {
                fn file_path(&self) -> &'static str {
                    file!()
                }

                fn run(&self, schema: &Schema) -> io::Result<Vec<Output>> {
                    self.generate(schema)
                }
            }
        };
    };
}
pub(crate) use define_generator;

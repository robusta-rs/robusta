// Directory adapted from https://github.com/oxc-project/oxc/tree/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/derives

use std::io;

use proc_macro2::TokenStream;
use quote::quote;
use rustc_hash::{FxHashMap, FxHashSet};
use syn::{Path, parse_str};

use crate::{
    codegen::Runner,
    invalid,
    output::{Output, output_path},
    parse::attr::{AttrLocation, AttrPart},
    schema::{FileId, Schema, StructOrEnum},
    utils::snake_case,
};

mod content_eq;
mod get_span;

pub use content_eq::DeriveContentEq;
pub use get_span::DeriveGetSpan;

/// Trait to define a derive.
pub trait Derive: Runner {
    // Methods which can/must be defined by implementer.

    /// Get trait name.
    fn trait_name(&self) -> &'static str;

    /// Get if trait has lifetime.
    fn trait_has_lifetime(&self) -> bool {
        false
    }

    /// Get crate trait is defined in.
    fn crate_name(&self) -> &'static str;

    /// Get the names of the attributes this derive owns.
    fn attrs(&self) -> &[&'static str] {
        &[]
    }

    /// Parse an attribute this derive owns.
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

    /// Generate prelude to be output at top of generated files.
    fn prelude(&self) -> TokenStream {
        quote!()
    }

    /// Generate trait implementation for a type.
    fn derive(&self, type_def: StructOrEnum<'_>, schema: &Schema) -> TokenStream;

    // Standard methods. Should not be overridden.

    /// Run derive on all types which derive the trait, and compile into 1 file per crate.
    fn output(&self, schema: &Schema) -> Vec<Output> {
        #[derive(Default)]
        struct CrateContent {
            import_file_ids: FxHashSet<FileId>,
            output: TokenStream,
        }

        // Run derive on all types which has `#[generate_derive]` attr for this trait.
        // Store results in a hash map indexed by crate name.
        let mut crate_contents = FxHashMap::<&str, CrateContent>::default();
        for type_def in schema.structs_and_enums() {
            if !type_def.generates_derive(self.trait_name()) {
                continue;
            }
            let derived = self.derive(type_def, schema);
            let file_id = type_def.file_id();

            let content = crate_contents.entry(schema.files[file_id].krate()).or_default();
            content.import_file_ids.insert(file_id);

            content.output.extend(quote! {
                ///@@line_break
                #derived
            });
        }

        // Generate an output for each crate.
        // Wrap each output in template with `use` statements to import types which were derived.
        let filename = format!("derive_{}.rs", snake_case(self.trait_name()));
        crate_contents
            .into_iter()
            .map(|(krate, content)| {
                let mut import_paths = content
                    .import_file_ids
                    .into_iter()
                    .map(|file_id| schema.files[file_id].import_path())
                    .collect::<Vec<_>>();
                import_paths.sort_unstable();

                let tokens = self.template(&import_paths, content.output);
                Output::from_tokens(output_path(&format!("crates/{krate}"), &filename), &tokens)
            })
            .collect()
    }

    /// Wrap derived output for a crate in template.
    /// Add prelude, and `use` statements to import types which were derived.
    fn template(&self, import_paths: &[&str], impls: TokenStream) -> TokenStream {
        let prelude = self.prelude();

        let use_modules = import_paths.iter().map(|import_path| {
            if import_path.is_empty() {
                quote! {
                    use crate::*;
                }
            } else {
                // `::ast::java` -> `use crate::ast::java::*;`
                let import_path: Path = parse_str(import_path).expect("a module path");
                quote!( use crate #import_path ::*; )
            }
        });

        quote! {
            #prelude

            ///@@line_break
            #(#use_modules)*

            ///@@line_break
            #impls
        }
    }
}

/// Macro to implement [`Runner`] for a [`Derive`].
///
/// Must be used on every [`Derive`].
macro_rules! define_derive {
    ($ident:ident) => {
        const _: () = {
            use std::io;

            use $crate::{codegen::Runner, output::Output, schema::Schema};

            impl Runner for $ident {
                fn file_path(&self) -> &'static str {
                    file!()
                }

                fn run(&self, schema: &Schema) -> io::Result<Vec<Output>> {
                    Ok(self.output(schema))
                }
            }
        };
    };
}
pub(crate) use define_derive;

// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/main.rs

//! Generation of the sources derived from the AST types.
//!
//! The `#[ast]` types are the single source of truth for the AST. [`parse`]
//! reads them into a [`Schema`], and every derive and generator reads that
//! schema. `robusta_ast_macros::ast` reads what this crate generates for it,
//! so this crate must not depend on the types it reads.
//!
//! Output goes through `prettyplease` and the workspace's `rustfmt`, so
//! generators emit plain Rust and leave layout to the formatter.

pub mod codegen;
pub mod derives;
pub mod find;
pub mod generators;
pub mod output;
pub mod parse;
pub mod schema;
mod utils;

use std::path::{Path, PathBuf};
use std::{fs, io};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use codegen::GeneratorOrDerive;
use derives::Derive;
use generators::Generator;
use output::{Output, RawOutput, output_path};
use schema::Schema;

/// Path to `robusta_ast` crate.
const AST_CRATE_PATH: &str = "crates/robusta_ast";

/// Path to `robusta_ast_macros` crate.
const AST_MACROS_CRATE_PATH: &str = "crates/robusta_ast_macros";

/// Path to `robusta_ast_visit` crate.
const AST_VISIT_CRATE_PATH: &str = "crates/robusta_ast_visit";

/// Derives (for use with `#[generate_derive]`).
const DERIVES: &[&(dyn Derive + Sync)] = &[&derives::DeriveContentEq, &derives::DeriveGetSpan];

/// Attributes on structs and enums (not including those defined by derives/generators).
const ATTRIBUTES: [&str; 2] = ["generate_derive", "plural"];

/// Code generators.
const GENERATORS: &[&(dyn Generator + Sync)] = &[
    &generators::AssertLayoutsGenerator,
    &generators::AstBuilderGenerator,
    &generators::AstKindGenerator,
    &generators::GetIdGenerator,
    &generators::InheritVariantsGenerator,
    &generators::VisitGenerator,
    &generators::FormatterAstNodesGenerator,
    &generators::FormatterFormatGenerator,
];

/// The workspace root, derived from this crate's manifest location.
#[must_use]
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Render every generated source, formatted, with its path relative to the
/// workspace root.
///
/// # Errors
///
/// Returns an error if the schema breaks a rule a generator relies on, or a
/// generated source does not parse or cannot be formatted.
pub fn generate(mut schema: Schema) -> io::Result<Vec<RawOutput>> {
    let runners = codegen::get_runners();
    for runner in &runners {
        runner.prepare(&mut schema);
    }
    let mut outputs = Vec::new();
    for runner in &runners {
        outputs.extend(runner.run(&schema)?);
    }
    outputs.push(generate_proc_macro().into_raw(file!())?);
    outputs.push(generate_updated_proc_macro(&runners)?);
    outputs.sort_unstable_by(|output1, output2| output1.path.cmp(&output2.path));
    Ok(outputs)
}

/// Generate function for proc macro in `robusta_ast_macros` crate.
///
/// This function translates trait name to path to the trait and any generic params.
fn generate_proc_macro() -> Output {
    let match_arms = DERIVES.iter().map(|derive| {
        let trait_name = derive.trait_name();
        let trait_ident = format_ident!("{trait_name}");
        let crate_ident = format_ident!("{}", derive.crate_name());
        if derive.trait_has_lifetime() {
            quote!( #trait_name => (quote!(::#crate_ident::#trait_ident), quote!( <'static> )) )
        } else {
            quote!( #trait_name => (quote!(::#crate_ident::#trait_ident), TokenStream::new()) )
        }
    });

    let tokens: TokenStream = quote! {
        use proc_macro2::TokenStream;
        use quote::quote;

        ///@@line_break
        pub fn get_trait_crate_and_generics(trait_name: &str) -> Option<(TokenStream, TokenStream)> {
            let res = match trait_name {
                #(#match_arms,)*
                _ => return None,
            };
            Some(res)
        }
    };

    Output::from_tokens(output_path(AST_MACROS_CRATE_PATH, "derived_traits.rs"), &tokens)
}

/// Update the list of helper attributes for `Ast` derive proc macro in `robusta_ast_macros` crate
/// to include all attrs which generators/derives utilize.
///
/// Unfortunately we can't add a separate generated file for this, as proc macros can only be declared
/// in the main `lib.rs` of a proc macro crate. So we have to edit the existing file.
fn generate_updated_proc_macro(runners: &[GeneratorOrDerive]) -> io::Result<RawOutput> {
    // Get all attrs which derives/generators use
    let mut attrs = runners.iter().flat_map(|runner| runner.attrs()).copied().collect::<Vec<_>>();
    attrs.extend(ATTRIBUTES);
    attrs.sort_unstable();
    let attrs = attrs.join(", ");

    // Load `robusta_ast_macros` crate's `lib.rs` file.
    // Substitute list of used attrs into `#[proc_macro_derive(Ast, attributes(...))]`.
    let path = format!("{AST_MACROS_CRATE_PATH}/src/lib.rs");
    let code = fs::read_to_string(workspace_root().join(&path))?;
    let bad_macro = || invalid(format!("{path}: no `Ast` derive to update"));
    let (start, end) = code.split_once("#[proc_macro_derive(").ok_or_else(bad_macro)?;
    let (_, end) = end.split_once(")]").ok_or_else(bad_macro)?;
    assert!(end.trim_start().starts_with("pub fn ast_derive("));
    let code = format!("{start}#[proc_macro_derive(Ast, attributes({attrs}))]{end}");

    RawOutput::from_rust_string(path, &code)
}

/// Write every generated source under `workspace_root`.
///
/// # Errors
///
/// Returns an error if the AST sources cannot be read, an output cannot be
/// formatted, or an output cannot be written.
pub fn write_generated(workspace_root: &Path) -> io::Result<()> {
    let schema = parse::parse_files(&find::source_paths(workspace_root), workspace_root)?;
    for output in generate(schema)? {
        output.write_to_file(workspace_root)?;
    }
    Ok(())
}

pub(crate) fn invalid(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[cfg(test)]
mod tests {
    use cow_utils::CowUtils;

    use crate::{
        generators::{AssertLayoutsGenerator, AstKindGenerator, Generator},
        parse::{parse_files, parse_sources},
        schema::{Def, StructDef, Visibility},
    };

    use super::{Schema, find::source_paths, generate, utils::snake_case, workspace_root};

    fn schema() -> Schema {
        let root = workspace_root();
        parse_files(&source_paths(&root), &root).expect("AST sources should parse")
    }

    #[test]
    fn every_visited_type_reaches_all_generated_sources() {
        let mut schema = schema();
        AstKindGenerator.prepare(&mut schema);
        let names = schema.visited().map(|item| item.name().to_string()).collect::<Vec<_>>();
        assert!(!names.is_empty(), "the AST should declare types");
        let kinds = schema
            .structs()
            .filter(|item| item.kind.has_kind)
            .map(|item| item.name.clone())
            .collect::<Vec<_>>();
        let generated = generate(schema).expect("generated sources should format");
        let source = |path: &str| {
            &generated.iter().find(|output| output.path.ends_with(path)).expect("an output").content
        };
        let ast_kind = source("ast_kind.rs");
        let visit = source("visit.rs");
        let ast_nodes = source("ast_nodes.rs");
        let format = source("format.rs");
        for name in &names {
            for source in [ast_nodes, format] {
                assert!(
                    source
                        .cow_replace("AstNode<'_, 'a,", "AstNode<'parent, 'a,")
                        .cow_replace("AstNode<'_, '_,", "AstNode<'parent, 'a,")
                        .contains(&format!("AstNode<'parent, 'a, {name}")),
                    "{name} is missing from a formatter output"
                );
            }
            assert!(
                visit.contains(&format!("fn walk_{}<", snake_case(name))),
                "{name} is marked #[ast(visit)] but has no traversal"
            );
        }
        for name in &kinds {
            assert!(ast_kind.contains(&format!("    {name},\n")), "{name} has no AstType variant");
        }
    }

    #[test]
    fn the_marker_separates_visited_syntax_from_other_generator_known_types() {
        let source = "
            #[ast] pub struct Span { pub start: u32, pub end: u32 }
            #[ast] pub struct NodeId { pub index: u32 }
            #[ast] pub struct Comment { pub span: Span }
            #[ast(visit)]
            pub struct Node<'a> { pub node_id: Cell<NodeId>, pub span: Span, pub comments: Vec<'a, Comment>, pub child: Option<Box<'a, Node<'a>>> }
            #[ast(visit)]
            pub struct Keyword { pub span: Span }
        ";
        let mut schema = parse_sources(vec![syn::parse_file(source).unwrap()]).unwrap();
        AstKindGenerator.prepare(&mut schema);
        let visited = schema.visited().map(|item| item.name().to_string()).collect::<Vec<_>>();
        assert_eq!(visited, ["Node", "Keyword"]);
        let node = schema.type_by_name("Node").as_struct().unwrap();
        let fields =
            node.visited_fields(&schema).map(|field| field.name.as_str()).collect::<Vec<_>>();
        assert_eq!(fields, ["child"], "a walk has no use for a span or a comment");
        assert!(node.kind.has_kind);
        assert!(!schema.type_by_name("Comment").as_struct().unwrap().kind.has_kind);
        assert!(
            !schema.type_by_name("Keyword").as_struct().unwrap().kind.has_kind,
            "a visited struct with no `node_id` has no kind"
        );
    }

    #[test]
    fn every_node_but_the_modifiers_has_a_kind_and_an_id_after_its_span() {
        let mut schema = schema();
        AstKindGenerator.prepare(&mut schema);
        AssertLayoutsGenerator.prepare(&mut schema);
        let without_kind = schema
            .structs()
            .filter(|item| item.visit.has_visitor() && !item.kind.has_kind)
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(without_kind, ["RequiresModifier", "KeywordModifier"]);
        for name in ["OrdinaryCompilationUnit", "CompactCompilationUnit", "ModularCompilationUnit"]
        {
            assert!(schema.type_by_name(name).as_struct().unwrap().kind.has_kind, "{name}");
        }

        let offset = |item: &StructDef, name: &str| {
            let field = item.fields.iter().find(|field| field.name == name).expect("a field");
            (field.offset.offset_64, field.offset.offset_32)
        };
        for item in schema.structs().filter(|item| item.kind.has_kind) {
            assert_eq!(offset(item, "span"), (0, 0), "{}", item.name);
            assert_eq!(offset(item, "node_id"), (8, 8), "{}", item.name);
        }
    }

    #[test]
    fn an_index_type_is_known_through_the_ast_marker_inside_its_macro() {
        let source = "
            define_nonmax_u32_index_type! { #[ast] pub struct NodeId; }
            define_nonmax_u32_index_type! { pub struct ScopeId; }
            other_macro! { #[ast] pub struct OtherId; }
            #[ast(foreign = NonMaxU32)] struct NonMaxU32Alias(NonZeroU32);
            #[ast] pub struct Node { pub node_id: Cell<NodeId>, pub next: Option<NodeId> }
        ";
        let mut schema = parse_sources(vec![syn::parse_file(source).unwrap()]).unwrap();
        AssertLayoutsGenerator.prepare(&mut schema);
        assert!(!schema.type_names.contains_key("ScopeId"));
        assert!(!schema.type_names.contains_key("OtherId"));

        let node_id = schema.type_by_name("NodeId").as_struct().unwrap();
        assert!(!node_id.is_foreign && !node_id.visit.has_visitor());
        assert_eq!(node_id.fields[0].visibility, Visibility::Private);
        assert_eq!((node_id.layout.layout_64.size, node_id.layout.layout_64.align), (4, 4));
        assert_eq!((node_id.layout.layout_32.size, node_id.layout.layout_32.align), (4, 4));

        // `Option<NodeId>` uses the niche of `NonMaxU32`.
        let node = schema.type_by_name("Node").as_struct().unwrap();
        assert_eq!((node.layout.layout_64.size, node.layout.layout_32.size), (8, 8));
    }

    #[test]
    fn a_marker_or_type_the_generator_does_not_know_is_an_error() {
        for (source, message) in [
            ("#[ast(unknown)] struct A;", "A: unknown #[ast] marker `unknown`"),
            ("#[ast] #[scope] struct A;", "A: bad #[scope]"),
            ("#[ast(visit)] #[scope(other)] struct A;", "A: bad #[scope]"),
            ("#[ast(visit)] #[scope] enum A { B = 0 }", "A: bad #[scope]"),
            ("#[ast(visit)] struct A { #[scope(enter_before)] b: u8 }", "A::b: bad #[scope]"),
            ("#[ast(visit)] #[scope] struct A { #[scope] b: u8 }", "A::b: bad #[scope]"),
            ("#[ast] #[content_eq] struct A;", "A: bad #[content_eq]"),
            ("#[ast] #[content_eq(other)] enum A { B = 0 }", "A: bad #[content_eq]"),
            ("#[ast] struct A { #[content_eq] b: u8 }", "A::b: bad #[content_eq]"),
            ("#[ast(content_eq)] struct A;", "A: bad #[content_eq]"),
            ("#[ast] enum A { #[content_eq(skip)] B = 0 }", "A::B: bad #[content_eq]"),
            ("#[ast(visit)] enum A { B = 0, #[scope] C = 1 }", "A::C: bad #[scope]"),
            ("#[ast] #[generate_derive(Unknown)] struct A;", "A: unknown derive trait `Unknown`"),
            ("#[ast] struct A { b: Unknown }", "A: unknown type `Unknown`"),
            ("#[ast] struct A(pub u8);", "A: a node has named fields"),
            ("#[ast] struct A; #[ast] enum A { B = 0 }", "2 types with same name: A"),
            ("#[ast] enum A { B }", "A::B: all enum variants must have explicit discriminants"),
            ("#[ast] enum A { INHERIT }", "A: `INHERIT` must name an enum"),
            ("#[ast] enum A { INHERIT(B) }", "A: `INHERIT` must name an enum"),
            ("#[ast] struct B; #[ast] enum A { INHERIT(B) }", "A: `INHERIT` must name an enum"),
            (
                "#[ast] enum A { B { c: u8 } }",
                "A::B: only enum variants with a single unnamed field are supported",
            ),
        ] {
            let file = syn::parse_file(source).unwrap();
            let error = parse_sources(vec![file]).err().expect(source);
            assert_eq!(error.to_string(), message, "{source}");
        }
    }
}

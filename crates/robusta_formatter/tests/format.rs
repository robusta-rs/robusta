//! End-to-end formatter tests organized by Java grammar and layout ownership.
//!
//! Large golden cases use explicit paired fixtures under `fixtures/java`: each
//! `<case>.java` input is paired with `<case>.expected.java`. The static test
//! declarations below keep every case visible in `cargo test`, while the
//! formatter-contract test rejects missing, duplicate, or unvisited fixtures.
//! Small focused regressions stay inline in their owning module.

mod fixtures;

macro_rules! fixture_tests {
    ($($test:ident => $fixture:literal;)+) => {
        pub const FIXTURES: &[&str] = &[$($fixture),+];

        $(
            #[test]
            fn $test() {
                $crate::fixtures::assert_java_fixture(
                    $fixture,
                    include_str!(concat!("../fixtures/java/", $fixture, ".java")),
                    include_str!(concat!("../fixtures/java/", $fixture, ".expected.java")),
                );
            }
        )+
    };
}

#[path = "format/comment_policy.rs"]
mod comment_policy;
#[path = "format/comments_trivia.rs"]
mod comments_trivia;
#[path = "format/compilation_units.rs"]
mod compilation_units;
#[path = "format/configuration.rs"]
mod configuration;
#[path = "format/contract.rs"]
mod contract;
#[path = "format/control_boundary_comments.rs"]
mod control_boundary_comments;
#[path = "format/declarations.rs"]
mod declarations;
#[path = "format/expressions.rs"]
mod expressions;
#[path = "format/javadoc.rs"]
mod javadoc;
#[path = "format/layout_width.rs"]
mod layout_width;
#[path = "format/statements.rs"]
mod statements;
#[path = "format/type_declaration_headers.rs"]
mod type_declaration_headers;
#[path = "format/types_annotations.rs"]
mod types_annotations;

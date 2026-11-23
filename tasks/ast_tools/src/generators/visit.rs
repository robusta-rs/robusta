// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/generators/visit.rs

//! Generator for the `Visit` trait.

use std::io;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    AST_VISIT_CRATE_PATH, invalid,
    output::{Output, output_path},
    parse::attr::{AttrLocation, AttrPart},
    schema::{
        Def, EnumDef, FieldDef, OptionDef, Schema, StructDef, TypeDef, VecDef,
        extensions::visit::{Scope, VisitorNames},
    },
    utils::snake_case,
};

use super::{Generator, define_generator};

/// Visitors with less than this number of fields/variants will be marked `#[inline]`.
const INLINE_LIMIT: usize = 5;

/// Generator for the `Visit` trait.
pub struct VisitGenerator;

define_generator!(VisitGenerator);

impl Generator for VisitGenerator {
    /// Own `#[ast(visit)]` on structs or enums, and `#[scope]` on structs or struct fields.
    fn attrs(&self) -> &[&'static str] {
        &["visit", "scope"]
    }

    /// Parse `#[ast(visit)]` and `#[scope]` attrs.
    fn parse_attr(
        &self,
        attr_name: &str,
        location: AttrLocation<'_>,
        part: AttrPart<'_>,
    ) -> io::Result<()> {
        match attr_name {
            "visit" => parse_visit_attr(location, &part),
            "scope" => parse_scope_attr(location, &part),
            _ => unreachable!(),
        }
    }

    /// Create names for `visit_*` methods and `walk_*` functions for all `Vec`s
    /// whose inner type has a visitor.
    fn prepare(&self, schema: &mut Schema) {
        for type_id in schema.types.indices() {
            let Some(vec_def) = schema.types[type_id].as_vec() else { continue };

            let inner_type = vec_def.inner_type(schema);
            let plural_snake_name = match inner_type {
                TypeDef::Struct(struct_def) if struct_def.visit.has_visitor() => {
                    struct_def.plural_snake_name()
                }
                TypeDef::Enum(enum_def) if enum_def.visit.has_visitor() => {
                    enum_def.plural_snake_name()
                }
                _ => continue,
            };

            let visitor_names = VisitorNames::from_snake_name(&plural_snake_name);
            let vec_def = schema.types[type_id].as_vec_mut().expect("type is a `Vec`");
            vec_def.visit.visitor_names = Some(visitor_names);
        }
    }

    fn generate(&self, schema: &Schema) -> io::Result<Vec<Output>> {
        let tokens = generate_output(schema);
        Ok(vec![Output::from_tokens(output_path(AST_VISIT_CRATE_PATH, "visit.rs"), &tokens)])
    }
}

/// Parse `#[ast(visit)]` attr.
fn parse_visit_attr(location: AttrLocation<'_>, part: &AttrPart<'_>) -> io::Result<()> {
    match (part, location) {
        (AttrPart::None, AttrLocation::StructAstAttr(struct_def)) => {
            let names = VisitorNames::from_snake_name(&struct_def.snake_name());
            struct_def.visit.visitor_names = Some(names);
            Ok(())
        }
        (AttrPart::None, AttrLocation::EnumAstAttr(enum_def)) => {
            let names = VisitorNames::from_snake_name(&enum_def.snake_name());
            enum_def.visit.visitor_names = Some(names);
            Ok(())
        }
        (_, location) => Err(invalid(format!("{location}: bad #[visit]"))),
    }
}

/// Parse `#[scope]` attr.
fn parse_scope_attr(location: AttrLocation<'_>, part: &AttrPart<'_>) -> io::Result<()> {
    match (part, location) {
        // `#[scope]` on struct
        (AttrPart::None, AttrLocation::Struct(struct_def)) if struct_def.visit.has_visitor() => {
            let exit_before_index = struct_def.fields.len();
            struct_def.visit.scope = Some(Scope { enter_before_index: 0, exit_before_index });
            Ok(())
        }
        // `#[scope(enter_before)]` on struct field
        (AttrPart::Tag("enter_before"), AttrLocation::StructField(struct_def, field_index))
            if struct_def.visit.scope.is_some() =>
        {
            struct_def.visit.scope.as_mut().unwrap().enter_before_index = field_index;
            Ok(())
        }
        // `#[scope(exit_before)]` on struct field
        (AttrPart::Tag("exit_before"), AttrLocation::StructField(struct_def, field_index))
            if struct_def.visit.scope.is_some() =>
        {
            struct_def.visit.scope.as_mut().unwrap().exit_before_index = field_index;
            Ok(())
        }
        (_, location) => Err(invalid(format!("{location}: bad #[scope]"))),
    }
}

/// Generate output for the `Visit` trait.
fn generate_output(schema: &Schema) -> TokenStream {
    let mut builder = VisitBuilder::new(schema);
    builder.generate();
    let expression_chain = generate_expression_chain(&builder.chains, schema);
    let VisitBuilder { visit_methods, walk_fns, .. } = builder;

    quote! {
        //! Visitor Pattern
        //!
        //! See:
        //! * [visitor pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
        //! * [rustc visitor](https://github.com/rust-lang/rust/blob/1.82.0/compiler/rustc_ast/src/visit.rs)

        //!@@line_break
        #![expect(unused_variables, clippy::semicolon_if_nothing_returned)]
        #![allow(clippy::needless_pass_by_ref_mut)]

        ///@@line_break
        use std::cell::Cell;

        ///@@line_break
        use robusta_allocator::Vec;
        use robusta_ast::*;
        use robusta_syntax::scope::ScopeId;
        use smallvec::SmallVec;

        ///@@line_break
        use walk::*;

        ///@@line_break
        /// Syntax tree traversal
        ///
        /// A chain the parser builds without recursion, such as `a.b().c`, is walked without
        /// recursion too. `visit_*` runs for the chain's outermost node only, while `enter_node`
        /// and `leave_node` run for all of its nodes.
        pub trait Visit<'a>: Sized {
            #[inline]
            fn enter_node(&mut self, kind: AstKind<'a>) {}
            #[inline]
            fn leave_node(&mut self, kind: AstKind<'a>) {}

            ///@@line_break
            #[inline]
            fn enter_scope(&mut self, scope_id: &Cell<Option<ScopeId>>) {}
            #[inline]
            fn leave_scope(&mut self) {}

            #visit_methods
        }

        ///@@line_break
        pub mod walk {
            use super::*;

            ///@@line_break
            #walk_fns

            #expression_chain
        }
    }
}

/// Generator of `visit_*` methods and `walk_*` functions for `Visit`.
struct VisitBuilder<'s> {
    schema: &'s Schema,
    chains: [Chain; 7],
    /// `visit_*` methods for `Visit`
    visit_methods: TokenStream,
    /// `walk_*` functions for `Visit`
    walk_fns: TokenStream,
}

impl<'s> VisitBuilder<'s> {
    /// Create new [`VisitBuilder`].
    fn new(schema: &'s Schema) -> Self {
        Self { schema, chains: chains(schema), visit_methods: quote!(), walk_fns: quote!() }
    }

    /// Generate `visit_*` methods and `walk_*` functions for `Visit`.
    fn generate(&mut self) {
        for type_def in &self.schema.types {
            match type_def {
                TypeDef::Struct(struct_def) => self.generate_struct_visitor(struct_def),
                TypeDef::Enum(enum_def) => self.generate_enum_visitor(enum_def),
                TypeDef::Vec(vec_def) => self.generate_vec_visitor(vec_def),
                _ => {}
            }
        }
    }

    /// Generate the `visit_*` method which calls a `walk_*` function.
    fn generate_visit_method(&mut self, names: &VisitorNames, ty: &TokenStream) {
        let visit_fn_ident = names.visitor_ident();
        let walk_fn_ident = names.walk_ident();
        self.visit_methods.extend(quote! {
            ///@@line_break
            #[inline]
            fn #visit_fn_ident(&mut self, it: &'a #ty) {
                #walk_fn_ident(self, it);
            }
        });
    }

    /// Generate `visit_*` method and `walk_*` function for a struct.
    fn generate_struct_visitor(&mut self, struct_def: &StructDef) {
        // Exit if this struct is not visited
        let Some(visitor_names) = &struct_def.visit.visitor_names else { return };

        let struct_ty = struct_def.ty(self.schema);
        self.generate_visit_method(visitor_names, &struct_ty);

        let walk_fn_ident = visitor_names.walk_ident();
        let (maybe_inline_attr, body) = self.generate_struct_walk_body(struct_def);
        self.walk_fns.extend(quote! {
            ///@@line_break
            #maybe_inline_attr
            pub fn #walk_fn_ident<'a, V: Visit<'a>>(visitor: &mut V, it: &'a #struct_ty) {
                #body
            }
        });
    }

    /// Generate the body of a struct's `walk_*` function, and its `#[inline]` attr if it has one.
    fn generate_struct_walk_body(&self, struct_def: &StructDef) -> (TokenStream, TokenStream) {
        if self.chains.iter().any(|chain| chain.node == struct_def.name) {
            let struct_ident = struct_def.ident();
            let body =
                quote!( walk_expression_chain(visitor, ExpressionChain::#struct_ident(it)); );
            return (quote!( #[inline] ), body);
        }
        if let Some(&(_, choice, first)) =
            NAME_CHAINS.iter().find(|(node, ..)| *node == struct_def.name)
        {
            return (quote!(), generate_name_chain(struct_def, choice, first, self.schema));
        }

        // Generate `enter_node` and `leave_node` calls (if this struct has an `AstKind`)
        let (enter_node, leave_node) =
            generate_enter_and_leave_node(&struct_def.ident(), struct_def.kind.has_kind);

        // Generate `enter_scope` and `leave_scope` calls (if this struct has a scope).
        // They will be inserted before the relevant fields.
        let (mut scope_entry, mut scope_exit) = struct_def.visit.scope.as_ref().map_or(
            (None, None),
            |&Scope { enter_before_index, exit_before_index }| {
                (
                    Some((enter_before_index, quote!( visitor.enter_scope(&it.scope_id); ))),
                    Some((exit_before_index, quote!( visitor.leave_scope(); ))),
                )
            },
        );

        // Generate `visit_*` calls for struct fields
        let mut field_visits_count = 0usize;
        let mut field_visits = struct_def
            .fields
            .iter()
            .enumerate()
            .filter_map(|(field_index, field)| {
                let visit = generate_field_visit(field, &quote!(it), self.schema)?;
                field_visits_count += 1;

                // We handle exiting scope first, to create correct output if entering and exiting
                // on same field.
                let visit = match scope_exit.take_if(|(index, _)| *index <= field_index) {
                    Some((_, leave_scope)) => quote!( #leave_scope #visit ),
                    None => visit,
                };
                let visit = match scope_entry.take_if(|(index, _)| *index <= field_index) {
                    Some((_, enter_scope)) => quote!( #enter_scope #visit ),
                    None => visit,
                };
                Some(visit)
            })
            .collect::<TokenStream>();

        // If didn't enter or exit scope already, enter/exit after last field
        if let Some((_, enter_scope)) = scope_entry {
            field_visits.extend(enter_scope);
        }
        if let Some((_, leave_scope)) = scope_exit {
            field_visits.extend(leave_scope);
        }

        // `#[inline]` if there are `INLINE_LIMIT` or less fields visited
        let maybe_inline_attr =
            if field_visits_count <= INLINE_LIMIT { quote!( #[inline] ) } else { quote!() };

        let body = quote! {
            #enter_node
            #field_visits
            #leave_node
        };
        (maybe_inline_attr, body)
    }

    /// Generate `visit_*` method and `walk_*` function for an enum.
    fn generate_enum_visitor(&mut self, enum_def: &EnumDef) {
        // Exit if this enum is not visited
        let Some(visitor_names) = &enum_def.visit.visitor_names else { return };

        let enum_ty = enum_def.ty(self.schema);
        self.generate_visit_method(visitor_names, &enum_ty);

        let walk_fn_ident = visitor_names.walk_ident();
        let enum_ident = enum_def.ident();

        let mut match_arm_count = 0usize;
        let variant_match_arms = enum_def
            .variants
            .iter()
            .filter_map(|variant| {
                let variant_type = variant.field_type(self.schema)?;
                let visit = generate_visit_type(
                    variant_type,
                    &Target::Reference(quote!(it)),
                    &quote!(it),
                    false,
                    self.schema,
                )?;

                match_arm_count += 1;

                let variant_ident = variant.ident();
                Some(quote!( #enum_ident::#variant_ident(it) => #visit, ))
            })
            .collect::<TokenStream>();

        let inherits_match_arms = enum_def
            .inherits
            .iter()
            .map(|&inherits_id| {
                let inherits_type = self.schema.enum_def(inherits_id);
                let Some(inner_visitor_names) = &inherits_type.visit.visitor_names else {
                    panic!(
                        "When an enum inherits variants from another enum and the inheritor has a visitor, \
                        the inherited enum must also have a visitor: `{}` inheriting from `{}`",
                        enum_def.name(),
                        inherits_type.name(),
                    );
                };
                let inner_visit_fn_ident = inner_visitor_names.visitor_ident();

                match_arm_count += 1;

                let inherits_snake_name = inherits_type.snake_name();
                let match_ident = format_ident!("match_{inherits_snake_name}");
                let to_fn_ident = format_ident!("to_{inherits_snake_name}");
                quote! {
                    #match_ident!(#enum_ident) => visitor.#inner_visit_fn_ident(it.#to_fn_ident()),
                }
            })
            .collect::<TokenStream>();

        // Add catch-all match arm if not all variants are visited
        let catch_all_match_arm =
            if match_arm_count < enum_def.variants.len() + enum_def.inherits.len() {
                quote!( _ => {} )
            } else {
                quote!()
            };

        // `#[inline]` if there are `INLINE_LIMIT` or less fields visited
        let maybe_inline_attr =
            if match_arm_count <= INLINE_LIMIT { quote!( #[inline] ) } else { quote!() };

        self.walk_fns.extend(quote! {
            ///@@line_break
            #maybe_inline_attr
            pub fn #walk_fn_ident<'a, V: Visit<'a>>(visitor: &mut V, it: &'a #enum_ty) {
                match it {
                    #variant_match_arms
                    #inherits_match_arms
                    #catch_all_match_arm
                }
            }
        });
    }

    /// Generate `visit_*` method and `walk_*` function for a `Vec`.
    fn generate_vec_visitor(&mut self, vec_def: &VecDef) {
        // Exit if this `Vec` does not have its own visitor
        let Some(visitor_names) = &vec_def.visit.visitor_names else { return };

        let vec_ty = vec_def.ty(self.schema);
        self.generate_visit_method(visitor_names, &vec_ty);

        let walk_fn_ident = visitor_names.walk_ident();
        let inner_visit_fn_ident = match vec_def.inner_type(self.schema) {
            TypeDef::Struct(struct_def) => visitor_ident(struct_def.visit.visitor_names.as_ref()),
            TypeDef::Enum(enum_def) => visitor_ident(enum_def.visit.visitor_names.as_ref()),
            _ => unreachable!(),
        };

        self.walk_fns.extend(quote! {
            ///@@line_break
            #[inline]
            pub fn #walk_fn_ident<'a, V: Visit<'a>>(visitor: &mut V, it: &'a #vec_ty) {
                for el in it {
                    visitor.#inner_visit_fn_ident(el);
                }
            }
        });
    }
}

/// Get the visitor method name of a type which has a visitor.
fn visitor_ident(visitor_names: Option<&VisitorNames>) -> Ident {
    visitor_names.expect("type has a visitor").visitor_ident()
}

// ----------------------
// Generate visitor calls
// ----------------------

/// Generate visitor call for a struct field of `owner`.
fn generate_field_visit(
    field: &FieldDef,
    owner: &TokenStream,
    schema: &Schema,
) -> Option<TokenStream> {
    let field_ident = field.ident();
    generate_visit_type(
        field.type_def(schema),
        &Target::Property(quote!( #owner.#field_ident )),
        &quote!(#field_ident),
        true,
        schema,
    )
}

/// Generate visitor calls for a type.
///
/// e.g.:
/// * `visitor.visit_type(&it.ty)`
/// * `if let Some(ty) = &it.ty { visitor.visit_type(ty); }`.
///
/// Returns `None` if this type is not visited.
///
/// * `target` is the expression for the type, represented by a [`Target`].
///   e.g. `it.ty` in first example above, or `ty` in the 2nd.
///
/// * `field_ident` is the name for the field. Is used in output for `Option`s.
///
/// * `trailing_semicolon` indicates if a semicolon postfix is needed.
///   This is `true` for struct fields, `false` for enum variants.
fn generate_visit_type(
    type_def: &TypeDef,
    target: &Target,
    field_ident: &TokenStream,
    trailing_semicolon: bool,
    schema: &Schema,
) -> Option<TokenStream> {
    let visit_fn_ident = match type_def {
        TypeDef::Struct(struct_def) => struct_def.visit.visitor_names.as_ref()?.visitor_ident(),
        TypeDef::Enum(enum_def) => enum_def.visit.visitor_names.as_ref()?.visitor_ident(),
        TypeDef::Vec(vec_def) => vec_def.visit.visitor_names.as_ref()?.visitor_ident(),
        TypeDef::Option(option_def) => {
            return generate_visit_option(option_def, target, field_ident, schema);
        }
        // `Box`es can be treated as transparent, as auto-deref handles it
        TypeDef::Box(box_def) => {
            return generate_visit_type(
                box_def.inner_type(schema),
                target,
                field_ident,
                trailing_semicolon,
                schema,
            );
        }
        // Primitives and `Cell`s are not visited
        TypeDef::Primitive(_) | TypeDef::Cell(_) => return None,
    };

    let target_ref = target.generate_ref();
    let semicolon = trailing_semicolon.then(|| quote!(;));
    Some(quote!( visitor.#visit_fn_ident(#target_ref) #semicolon ))
}

/// Generate visitor calls for an `Option`.
///
/// e.g.:
/// ```ignore
/// if let Some(ty) = &it.ty {
///     visitor.visit_type(ty);
/// }
/// ```
///
/// Returns `None` if inner type is not visited.
fn generate_visit_option(
    option_def: &OptionDef,
    target: &Target,
    field_ident: &TokenStream,
    schema: &Schema,
) -> Option<TokenStream> {
    let inner_visit = generate_visit_type(
        option_def.inner_type(schema),
        &Target::Reference(field_ident.clone()),
        field_ident,
        true,
        schema,
    )?;
    let target_ref = target.generate_ref();
    Some(quote! {
        if let Some(#field_ident) = #target_ref {
            #inner_visit
        }
    })
}

/// Target for a visit function call.
///
/// * `Target::Reference` represents a variable which is already a reference.
///   e.g. `ty` in `if let Some(ty) = &it.ty {}`
///
/// * `Target::Property` represents an object property e.g. `it.ty`.
///   Needs `&` prepended to it when using it.
enum Target {
    Reference(TokenStream),
    Property(TokenStream),
}

impl Target {
    /// Prepend target with `&` if required.
    fn generate_ref(&self) -> TokenStream {
        match self {
            Self::Reference(ident) => ident.clone(),
            Self::Property(prop) => quote!( &#prop ),
        }
    }
}

/// Generate code for `enter_node` and `leave_node`.
///
/// If the type has no `AstKind`, returns a comment for enter, and empty token stream for exit.
fn generate_enter_and_leave_node(
    type_ident: &Ident,
    has_kind: bool,
) -> (/* enter_node */ TokenStream, /* leave_node */ TokenStream) {
    if has_kind {
        let enter_node = quote! {
            let kind = AstKind::#type_ident(it);
            visitor.enter_node(kind);
        };
        (enter_node, quote!( visitor.leave_node(kind); ))
    } else {
        (quote!( #![doc = "@ No `AstKind` for this type"] ), quote!())
    }
}

// ----------------------
// Generate chains
// ----------------------

/// A node of an operand or receiver chain, which the parser builds without recursion.
struct Chain {
    node: &'static str,
    /// The `Expression` variant which holds the node.
    variant: &'static str,
    /// The field which holds the next node of the chain.
    member: &'static str,
    /// Yields the next node, for the field which holds it.
    next: TokenStream,
    /// What the node visits on the way back, before the fields after `member`.
    unwind: TokenStream,
}

impl Chain {
    fn new(node: &'static str, variant: &'static str, member: &'static str) -> Self {
        let member_ident = format_ident!("{member}");
        Self {
            node,
            variant,
            member,
            next: quote!( Some(&it.#member_ident) ),
            unwind: TokenStream::new(),
        }
    }
}

fn chains(schema: &Schema) -> [Chain; 7] {
    let method_member =
        schema.type_by_name("MethodMember").as_struct().expect("`MethodMember` is a struct");
    let member_fields = method_member
        .visited_fields(schema)
        .filter(|field| field.name != "target")
        .filter_map(|field| generate_field_visit(field, &quote!(member), schema))
        .collect::<TokenStream>();
    [
        Chain::new("FieldAccess", "FieldAccess", "target"),
        Chain {
            next: quote! {
                match &it.callee {
                    Callee::Member(member) => {
                        visitor.enter_node(AstKind::MethodMember(member));
                        Some(&member.target)
                    }
                    Callee::Identifier(_) => {
                        visitor.visit_callee(&it.callee);
                        None
                    }
                }
            },
            unwind: quote! {
                if let Callee::Member(member) = &it.callee {
                    #member_fields
                    visitor.leave_node(AstKind::MethodMember(member));
                }
            },
            ..Chain::new("MethodCall", "MethodCall", "callee")
        },
        Chain::new("QualifiedClassInstanceCreation", "QualifiedClassInstanceCreation", "qualifier"),
        Chain::new("ArrayAccess", "ArrayAccess", "array"),
        Chain::new("InstanceofExpression", "Instanceof", "operand"),
        Chain::new("BinaryExpression", "Binary", "lhs"),
        Chain {
            next: quote! {
                if let Some(expression) = it.target.as_expression() {
                    Some(expression)
                } else {
                    visitor.visit_method_reference_target(&it.target);
                    None
                }
            },
            ..Chain::new("MethodReference", "MethodReference", "target")
        },
    ]
}

// The parser builds dotted names without recursion too, each holding the name to
// its left in `qualifier`. These give the choice that holds it, and the variant
// that starts the name.
const NAME_CHAINS: [(&str, &str, &str); 3] = [
    ("QualifiedName", "Name", "Identifier"),
    ("QualifiedTypeName", "TypeName", "Identifier"),
    ("QualifiedClassType", "ClassType", "Simple"),
];

/// Generate a walk that keeps the names to its right on a list, in place of the stack.
fn generate_name_chain(
    struct_def: &StructDef,
    choice: &str,
    first: &str,
    schema: &Schema,
) -> TokenStream {
    let node = struct_def.ident();
    let choice_ident = format_ident!("{choice}");
    let first_ident = format_ident!("{first}");
    let visit_choice = format_ident!("visit_{}", snake_case(choice));
    let field_visits = struct_def
        .visited_fields(schema)
        .filter(|field| field.name != "qualifier")
        .filter_map(|field| generate_field_visit(field, &quote!(node), schema))
        .collect::<TokenStream>();
    quote! {
        let mut node = it;
        let mut parents = SmallVec::<[&'a #node<'a>; 4]>::new();
        loop {
            visitor.enter_node(AstKind::#node(node));
            match &node.qualifier {
                #choice_ident::Qualified(next) => {
                    parents.push(node);
                    node = next;
                }
                qualifier @ #choice_ident::#first_ident(_) => {
                    visitor.#visit_choice(qualifier);
                    break;
                }
            }
        }
        loop {
            #field_visits
            visitor.leave_node(AstKind::#node(node));
            let Some(parent) = parents.pop() else { break };
            node = parent;
        }
    }
}

/// Generate the walk that keeps the nodes of a chain on a list, in place of the stack.
fn generate_expression_chain(chains: &[Chain], schema: &Schema) -> TokenStream {
    let variants = chains.iter().map(|chain| {
        let node = format_ident!("{}", chain.node);
        quote!( #node(&'a #node<'a>), )
    });
    let from_expression = chains.iter().map(|chain| {
        let node = format_ident!("{}", chain.node);
        let variant = format_ident!("{}", chain.variant);
        quote!( Expression::#variant(it) => Some(Self::#node(it)), )
    });
    let enter = chains.iter().map(|chain| {
        let node = format_ident!("{}", chain.node);
        let (before, _) = chain_field_visits(chain, schema);
        let next = &chain.next;
        quote! {
            ExpressionChain::#node(it) => {
                visitor.enter_node(AstKind::#node(it));
                #before
                #next
            }
        }
    });
    let leave = chains.iter().map(|chain| {
        let node = format_ident!("{}", chain.node);
        let (_, after) = chain_field_visits(chain, schema);
        let unwind = &chain.unwind;
        quote! {
            ExpressionChain::#node(it) => {
                #unwind
                #after
                visitor.leave_node(AstKind::#node(it));
            }
        }
    });
    quote! {
        ///@@line_break
        #[derive(Clone, Copy)]
        enum ExpressionChain<'a> {
            #(#variants)*
        }

        ///@@line_break
        impl<'a> ExpressionChain<'a> {
            fn from_expression(expression: &'a Expression<'a>) -> Option<Self> {
                match expression {
                    #(#from_expression)*
                    _ => None,
                }
            }
        }

        ///@@line_break
        fn walk_expression_chain<'a, V: Visit<'a>>(visitor: &mut V, mut node: ExpressionChain<'a>) {
            let mut parents = SmallVec::<[ExpressionChain<'a>; 4]>::new();
            loop {
                let next = match node {
                    #(#enter)*
                };
                let Some(next) = next else { break };
                if let Some(next) = ExpressionChain::from_expression(next) {
                    parents.push(node);
                    node = next;
                } else {
                    visitor.visit_expression(next);
                    break;
                }
            }
            loop {
                match node {
                    #(#leave)*
                }
                let Some(parent) = parents.pop() else { break };
                node = parent;
            }
        }
    }
}

/// Generate the visitor calls for the fields before and after a chain's `member`.
fn chain_field_visits(chain: &Chain, schema: &Schema) -> (TokenStream, TokenStream) {
    let item = schema.type_by_name(chain.node).as_struct().expect("expression chain is a struct");
    assert!(item.kind.has_kind, "expression node kind");
    let mut before = TokenStream::new();
    let mut after = TokenStream::new();
    let mut found = false;
    for field in item.visited_fields(schema) {
        if field.name == chain.member {
            found = true;
        } else if let Some(visit) = generate_field_visit(field, &quote!(it), schema) {
            if found { after.extend(visit) } else { before.extend(visit) }
        }
    }
    assert!(found, "chain operand");
    (before, after)
}

#[cfg(test)]
mod tests {
    use crate::{
        generators::{AstKindGenerator, Generator, VisitGenerator},
        parse::parse_sources,
    };

    const CHAIN_TYPES: &str = "
        #[ast(visit)] pub struct FieldAccess<'a> { pub node_id: Cell<NodeId>, pub target: Expression<'a> }
        #[ast(visit)] pub struct MethodCall<'a> { pub node_id: Cell<NodeId>, pub callee: Callee<'a> }
        #[ast(visit)] pub struct MethodMember<'a> { pub node_id: Cell<NodeId>, pub target: Expression<'a> }
        #[ast(visit)] pub struct Identifier { pub node_id: Cell<NodeId> }
        #[ast(visit)] pub enum Callee<'a> { Identifier(Identifier) = 0, Member(MethodMember<'a>) = 1 }
        #[ast(visit)] pub struct QualifiedClassInstanceCreation<'a> { pub node_id: Cell<NodeId>, pub qualifier: Expression<'a> }
        #[ast(visit)] pub struct ArrayAccess<'a> { pub node_id: Cell<NodeId>, pub array: Expression<'a> }
        #[ast(visit)] pub struct InstanceofExpression<'a> { pub node_id: Cell<NodeId>, pub operand: Expression<'a> }
        #[ast(visit)] pub struct BinaryExpression<'a> { pub node_id: Cell<NodeId>, pub lhs: Expression<'a> }
        #[ast(visit)] pub struct MethodReference<'a> { pub node_id: Cell<NodeId>, pub target: Expression<'a> }
        #[ast(visit)] pub enum Expression<'a> { Identifier(Identifier) = 0 }
        #[ast] pub struct NodeId { pub index: u32 }
        #[ast] pub struct ScopeId { pub index: u32 }
    ";

    fn generate(source: &str) -> String {
        let source = format!("{source}{CHAIN_TYPES}");
        let mut schema = parse_sources(vec![syn::parse_file(&source).unwrap()]).unwrap();
        AstKindGenerator.prepare(&mut schema);
        VisitGenerator.prepare(&mut schema);
        let outputs = VisitGenerator.generate(&schema).unwrap();
        outputs[0].code.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn body<'c>(code: &'c str, walk: &str) -> &'c str {
        let body = &code[code.find(&format!("pub fn {walk} <")).expect("a walk")..];
        &body[..body[1..].find("pub fn ").map_or(body.len(), |end| end + 1)]
    }

    #[test]
    fn scope_markers_place_the_scope_events_between_the_fields() {
        let code = generate(
            "
            #[ast(visit)] #[scope] pub struct Whole<'a> {
                pub node_id: Cell<NodeId>,
                pub first: Identifier,
                pub scope_id: Cell<Option<ScopeId>>,
            }
            #[ast(visit)] #[scope] pub struct Late<'a> {
                pub first: Identifier,
                #[scope(enter_before)] pub second: Identifier,
                pub scope_id: Cell<Option<ScopeId>>,
            }
            #[ast(visit)] #[scope] pub struct Early<'a> {
                pub first: Identifier,
                #[scope(exit_before)] pub second: Identifier,
                pub third: Identifier,
                pub scope_id: Cell<Option<ScopeId>>,
            }
            ",
        );
        let enter = "visitor . enter_scope (& it . scope_id) ;";
        let leave = "visitor . leave_scope () ;";
        let first = "visitor . visit_identifier (& it . first) ;";
        let second = "visitor . visit_identifier (& it . second) ;";
        let third = "visitor . visit_identifier (& it . third) ;";
        assert!(body(&code, "walk_whole").contains(&format!("{enter} {first} {leave}")), "{code}");
        assert!(body(&code, "walk_late").contains(&format!("{first} {enter} {second} {leave}")));
        assert!(
            body(&code, "walk_early")
                .contains(&format!("{enter} {first} {leave} {second} {third}"))
        );
    }

    #[test]
    fn a_walk_enters_a_node_around_its_fields_and_a_choice_dispatches_through_inherited_choices() {
        let code = generate(
            "
            #[ast(visit)] pub struct Node<'a> { pub node_id: Cell<NodeId>, pub items: Vec<'a, Item<'a>> }
            #[ast(visit)] pub struct NoKind { pub first: Identifier }
            #[ast(visit)] pub enum Item<'a> { Node(Box<'a, Node<'a>>) = 0, INHERIT(Inner) }
            #[ast(visit)] pub enum Inner { NoKind(NoKind) = 1 }
            ",
        );
        let node = body(&code, "walk_node");
        assert!(node.contains("visitor . enter_node (kind) ; visitor . visit_items (& it . items) ; visitor . leave_node (kind) ;"), "{code}");
        assert!(
            body(&code, "walk_items").contains("for el in it { visitor . visit_item (el) ; }"),
            "{code}"
        );
        assert!(!body(&code, "walk_no_kind").contains("enter_node"), "{code}");
        let item = body(&code, "walk_item");
        assert!(item.contains("Item :: Node (it) => visitor . visit_node (it) ,"), "{code}");
        assert!(
            item.contains("match_inner ! (Item) => visitor . visit_inner (it . to_inner ()) ,"),
            "{code}"
        );
    }
}

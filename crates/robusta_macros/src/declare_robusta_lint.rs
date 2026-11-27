// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_macros/src/declare_oxc_lint.rs

use convert_case::{Boundary, Case, Converter};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Error, Expr, ExprLit, Ident, Lit, LitStr, Meta, MetaNameValue, Result, Token, Type,
    braced, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Bracket,
};

pub struct LintRuleMeta {
    /// The doc comments, kept on the rule's struct.
    attributes: Vec<Attribute>,
    /// The doc comments as one string, a line per comment without its leading space.
    documentation: String,
    name: Ident,
    id_const: Ident,
    instance: Ident,
    summary: LitStr,
    category: Ident,
    fix: Expr,
    /// The node kinds `run` matches on, or `None` for every node.
    node_types: Option<Vec<Ident>>,
    /// The settings type and its default registry value.
    settings: Option<(Type, Expr)>,
}

impl Parse for LintRuleMeta {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let mut documentation = String::new();
        for attr in &attributes {
            let Some(line) = parse_doc_attr(attr) else {
                return Err(Error::new_spanned(attr, "unexpected attribute"));
            };
            documentation.push_str(line.strip_prefix(' ').unwrap_or(&line));
            documentation.push('\n');
        }

        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let mut id_const = None;
        let mut instance = None;
        let mut summary = None;
        let mut category = None;
        let mut fix = None;
        let mut node_types = None;
        let mut settings = None;
        let mut keys = Vec::new();

        // Each field is `key: value,`.
        while !content.is_empty() {
            let key: Ident = content.parse()?;
            if keys.contains(&key) {
                return Err(Error::new_spanned(key, "key already specified in rule declaration"));
            }
            content.parse::<Token![:]>()?;
            match key.to_string().as_str() {
                "id" => id_const = Some(content.parse()?),
                "rule" => instance = Some(content.parse()?),
                "summary" => summary = Some(content.parse()?),
                "category" => category = Some(content.parse()?),
                "fix" => fix = Some(content.parse()?),
                "node_types" => node_types = Some(parse_node_types(&content)?),
                "settings" => settings = Some(assignment(&content)?),
                _ => return Err(Error::new_spanned(key, "unexpected key in rule declaration")),
            }
            content.parse::<Token![,]>()?;
            keys.push(key);
        }

        let span = name.span();
        let missing = |key| Error::new(span, format!("missing `{key}` in `declare_robusta_lint!`"));
        Ok(Self {
            attributes,
            documentation,
            name,
            id_const: id_const.ok_or_else(|| missing("id"))?,
            instance: instance.ok_or_else(|| missing("rule"))?,
            summary: summary.ok_or_else(|| missing("summary"))?,
            category: category.ok_or_else(|| missing("category"))?,
            fix: fix.ok_or_else(|| missing("fix"))?,
            node_types: node_types.ok_or_else(|| missing("node_types"))?,
            settings,
        })
    }
}

pub fn rule_name_converter() -> Converter {
    Converter::new().remove_boundary(Boundary::LowerDigit).to_case(Case::Kebab)
}

pub fn declare_robusta_lint(metadata: LintRuleMeta) -> TokenStream {
    let LintRuleMeta {
        attributes,
        documentation,
        name,
        id_const,
        instance,
        summary,
        category,
        fix,
        node_types,
        settings,
    } = metadata;

    let id = rule_name_converter().convert(name.to_string());
    let (fields, value) = match &settings {
        Some((settings, default)) => {
            (quote! { { settings: #settings } }, quote! { #name { settings: #default } })
        }
        None => (quote! { ; }, quote! { #name }),
    };
    let has_config = settings.is_some();
    let settings_schema = settings.map(|(settings, _)| {
        quote! {
            fn settings_schema(
                &self,
                generator: &mut ::schemars::SchemaGenerator,
            ) -> Option<::schemars::schema::Schema> {
                Some(generator.subschema_for::<#settings>())
            }
        }
    });
    let node_types = if let Some(kinds) = node_types {
        quote! {{
            static NODE_TYPES: ::robusta_semantic::AstTypesBitset =
                ::robusta_semantic::AstTypesBitset::from_types(&[
                    #(::robusta_ast::AstType::#kinds),*
                ]);
            ::core::option::Option::Some(&NODE_TYPES)
        }}
    } else {
        quote! { ::core::option::Option::None }
    };

    let output = quote! {
        /// Stable, user-facing rule ID.
        pub const #id_const: crate::RuleId = crate::RuleId::new(#id);

        #(#attributes)*
        #[derive(Debug, Default)]
        pub struct #name #fields

        /// Registry instance dispatched for the rule's declared node kinds.
        pub static #instance: #name = #value;

        impl crate::DeclaredRule for #name {
            #[cfg(feature = "ruledocs")]
            fn documentation(&self) -> Option<&'static str> {
                Some(#documentation)
            }

            fn meta(&self) -> &'static crate::RuleMeta {
                static META: crate::RuleMeta = crate::RuleMeta {
                    id: #id_const,
                    summary: #summary,
                    category: crate::RuleCategory::#category,
                    fix: #fix,
                };
                &META
            }

            fn node_types(&self) -> Option<&'static ::robusta_semantic::AstTypesBitset> {
                #node_types
            }

            fn has_config(&self) -> bool {
                #has_config
            }

            #settings_schema

            fn configure(
                &self,
                value: ::serde_json::Value,
            ) -> Result<::std::sync::Arc<dyn crate::Rule>, ::serde_json::Error> {
                let rule = <#name as crate::Rule>::from_configuration(value)?;
                Ok(::std::sync::Arc::new(rule))
            }
        }
    };

    TokenStream::from(output)
}

/// Parses `left = right`.
fn assignment<L: Parse, R: Parse>(input: ParseStream<'_>) -> Result<(L, R)> {
    let left = input.parse()?;
    input.parse::<Token![=]>()?;
    Ok((left, input.parse()?))
}

/// Parses `any` or a bracketed, non-empty list of node kinds.
fn parse_node_types(input: ParseStream<'_>) -> Result<Option<Vec<Ident>>> {
    if !input.peek(Bracket) {
        let any: Ident = input.parse()?;
        if any != "any" {
            return Err(Error::new_spanned(any, "expected `any` or a list of node kinds"));
        }
        return Ok(None);
    }
    let content;
    bracketed!(content in input);
    let kinds = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;
    if kinds.is_empty() {
        return Err(Error::new(content.span(), "expected at least one node kind"));
    }
    Ok(Some(kinds.into_iter().collect()))
}

/// The text of a `#[doc = "..."]` attribute.
fn parse_doc_attr(attr: &Attribute) -> Option<String> {
    match &attr.meta {
        Meta::NameValue(MetaNameValue {
            path,
            value: Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }),
            ..
        }) if path.is_ident("doc") => Some(lit.value()),
        _ => None,
    }
}

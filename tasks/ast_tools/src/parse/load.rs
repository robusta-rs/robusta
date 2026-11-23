use std::{fs, io, path::Path};

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Attribute, Ident, Item, ItemMacro, Meta, Token, Visibility, parse::ParseStream, parse_quote,
    punctuated::Punctuated,
};

use crate::{invalid, schema::FileId};

use super::skeleton::{EnumSkeleton, Skeleton, StructSkeleton};

/// Read a Rust source file and parse it with `syn`.
pub fn load_file(path: &Path) -> io::Result<syn::File> {
    let source = fs::read_to_string(path)?;
    syn::parse_file(&source).map_err(|error| invalid(format!("{}: {error}", path.display())))
}

/// Get the [`Skeleton`] of a struct or enum with an `#[ast]` attribute.
pub fn parse_item(item: Item, file_id: FileId) -> io::Result<Option<Skeleton>> {
    match item {
        Item::Struct(item) => {
            let Some(attr) = parse_ast_attr(&item.attrs, &item.ident)? else { return Ok(None) };
            let is_foreign = attr.foreign.is_some();
            let name = attr.foreign.unwrap_or_else(|| item.ident.to_string());
            Ok(Some(Skeleton::Struct(StructSkeleton { name, file_id, is_foreign, item })))
        }
        Item::Enum(item) => {
            let Some(attr) = parse_ast_attr(&item.attrs, &item.ident)? else { return Ok(None) };
            if attr.foreign.is_some() {
                return Err(invalid(format!("{}: only a struct can be foreign", item.ident)));
            }
            let name = item.ident.to_string();
            Ok(Some(Skeleton::Enum(EnumSkeleton { name, file_id, item })))
        }
        Item::Macro(item) => parse_macro(&item, file_id),
        _ => Ok(None),
    }
}

/// Get the [`Skeleton`] of an index type whose macro holds an `#[ast]` attribute.
fn parse_macro(item: &ItemMacro, file_id: FileId) -> io::Result<Option<Skeleton>> {
    if !item.mac.path.is_ident("define_nonmax_u32_index_type") {
        return Ok(None);
    }
    let (attrs, vis, ident) = item
        .mac
        .parse_body_with(|input: ParseStream<'_>| {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis = input.parse::<Visibility>()?;
            input.parse::<Token![struct]>()?;
            let ident = input.parse::<Ident>()?;
            input.parse::<Token![;]>()?;
            // Ignore optional macro configuration after the type declaration.
            input.parse::<TokenStream>()?;
            Ok((attrs, vis, ident))
        })
        .map_err(|error| invalid(format!("bad index type macro: {error}")))?;
    let item = parse_quote! {
        #(#attrs)*
        #vis struct #ident (NonMaxU32);
    };
    parse_item(Item::Struct(item), file_id)
}

/// What an `#[ast]` attribute says about a type's skeleton.
struct AstAttr {
    /// `#[ast(foreign = Type)]`: the name of the type this one describes.
    foreign: Option<String>,
}

fn parse_ast_attr(attrs: &[Attribute], ident: &Ident) -> io::Result<Option<AstAttr>> {
    let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident("ast")) else {
        return Ok(None);
    };
    if matches!(attr.meta, Meta::Path(_)) {
        return Ok(Some(AstAttr { foreign: None }));
    }
    let metas = attr
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        .map_err(|error| invalid(format!("{ident}: bad #[ast] attribute: {error}")))?;
    let foreign = metas.iter().find_map(|meta| match meta {
        Meta::NameValue(name_value) if name_value.path.is_ident("foreign") => {
            Some(name_value.value.to_token_stream().to_string())
        }
        _ => None,
    });
    Ok(Some(AstAttr { foreign }))
}

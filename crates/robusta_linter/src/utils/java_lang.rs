use robusta_ast::{AstKind, ClassType, IdentifierReference, Type, TypeDeclarationKind};
use robusta_semantic::{ReferenceId, ScopeId, Semantic, Symbol, SymbolKind};
use robusta_str::Ident;

use super::imports_java_lang_type;

/// Whether the simple type name `name` names `java.lang`'s type of that name,
/// as far as this file shows.
///
/// A type the file declares or imports from elsewhere takes the name where it
/// is in scope (JLS §6.4.1), and so does a member type that an enclosing class
/// inherits from a supertype in the file (JLS §8.5). Types from other files are
/// not resolved.
pub fn names_java_lang_type(semantic: &Semantic<'_>, name: &IdentifierReference<'_>) -> bool {
    type_symbol(semantic, name.reference_id(), &mut Vec::new()).is_none_or(|symbol| {
        matches!(
            semantic.nodes().kind(symbol.declaration_node_id()),
            AstKind::ImportDeclaration(import) if imports_java_lang_type(import)
        )
    })
}

/// The symbol in this file that the simple type name of `reference_id` names.
fn type_symbol<'s, 'a>(
    semantic: &'s Semantic<'a>,
    reference_id: ReferenceId,
    lookups: &mut Vec<(ScopeId, Ident<'a>)>,
) -> Option<&'s Symbol<'a>> {
    let scoping = semantic.scoping();
    let reference = scoping.reference(reference_id);
    let binding = reference.symbol_id().map(|symbol_id| scoping.symbol(symbol_id));
    // A class between the name and its binding may inherit a member type that
    // hides the binding (JLS §6.4.1).
    let binding_scope = binding.map(Symbol::scope_id);
    scoping
        .scope_ancestors(reference.scope_id())
        .take_while(|&scope_id| Some(scope_id) != binding_scope)
        .find_map(|scope_id| inherited_member_type(semantic, scope_id, reference.ident(), lookups))
        .or(binding)
}

/// The member type named `name` that the type with body `body` declares or
/// inherits.
fn member_type<'s, 'a>(
    semantic: &'s Semantic<'a>,
    body: ScopeId,
    name: Ident<'a>,
    lookups: &mut Vec<(ScopeId, Ident<'a>)>,
) -> Option<&'s Symbol<'a>> {
    semantic
        .scoping()
        .symbols_for_name(body, name)
        .map(|(_, symbol)| symbol)
        .find(|symbol| symbol.kind() == SymbolKind::Type)
        .or_else(|| inherited_member_type(semantic, body, name, lookups))
}

/// The member type named `name` that the type with body `body` inherits from
/// a supertype declared in this file. Private member types are not inherited.
///
/// `lookups` holds the lookups in progress, so that cyclic inheritance, which
/// does not compile, still ends.
fn inherited_member_type<'s, 'a>(
    semantic: &'s Semantic<'a>,
    body: ScopeId,
    name: Ident<'a>,
    lookups: &mut Vec<(ScopeId, Ident<'a>)>,
) -> Option<&'s Symbol<'a>> {
    if lookups.contains(&(body, name)) {
        return None;
    }
    lookups.push((body, name));
    let member = supertypes(semantic, body).find_map(|supertype| {
        let supertype = type_body(semantic, supertype, lookups)?;
        member_type(semantic, supertype, name, lookups).filter(|member| !member.is_private())
    });
    lookups.pop();
    member
}

/// The direct supertypes written for the type with body `body`. An enum
/// constant's body has none of its own: the enum's body encloses it.
fn supertypes<'a>(
    semantic: &Semantic<'a>,
    body: ScopeId,
) -> impl Iterator<Item = &'a ClassType<'a>> {
    let scope = semantic.scoping().scope(body);
    let (superclass, interfaces, created): (_, &[Type<'a>], _) = if scope.is_type() {
        match semantic.nodes().parent_kind(scope.owner_node_id()) {
            AstKind::ClassDeclaration(class) => {
                (class.superclass.as_ref(), &class.interfaces, None)
            }
            AstKind::InterfaceDeclaration(interface) => (None, &interface.extends, None),
            AstKind::EnumDeclaration(declaration) => (None, &declaration.interfaces, None),
            AstKind::RecordDeclaration(record) => (None, &record.interfaces, None),
            AstKind::ClassInstanceCreation(creation) => (None, &[], Some(&creation.ty)),
            _ => (None, &[], None),
        }
    } else {
        (None, &[], None)
    };
    superclass.into_iter().chain(interfaces).filter_map(Type::as_class_type).chain(created)
}

/// The body of the type declared in this file that `ty` names.
fn type_body<'a>(
    semantic: &Semantic<'a>,
    ty: &ClassType<'a>,
    lookups: &mut Vec<(ScopeId, Ident<'a>)>,
) -> Option<ScopeId> {
    let symbol = match ty {
        ClassType::Simple(simple) => type_symbol(semantic, simple.name.reference_id(), lookups)?,
        ClassType::Qualified(qualified) => {
            let qualifier = type_body(semantic, &qualified.qualifier, lookups)?;
            member_type(semantic, qualifier, Ident::from(qualified.name.name), lookups)?
        }
    };
    if symbol.kind() != SymbolKind::Type {
        return None;
    }
    let body_scope_id =
        match semantic.nodes().kind(symbol.declaration_node_id()).as_type_declaration_kind()? {
            TypeDeclarationKind::Class(declaration) => &declaration.body.scope_id,
            TypeDeclarationKind::Interface(declaration) => &declaration.body.scope_id,
            TypeDeclarationKind::Enum(declaration) => &declaration.body.scope_id,
            TypeDeclarationKind::Record(declaration) => &declaration.body.scope_id,
            TypeDeclarationKind::AnnotationInterface(declaration) => &declaration.body.scope_id,
        };
    body_scope_id.get()
}

// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_semantic/src/scoping.rs

use std::{fmt, marker::PhantomData};

use oxc_index::IndexVec;
use robusta_allocator::{Allocator, HashMap as ArenaHashMap, Vec as ArenaVec, hash_map::Entry};
use robusta_ast::ImportKind;
use robusta_span::Span;
use robusta_str::{ArenaIdentHashMap, Ident};
use robusta_syntax::{reference::ReferenceId, scope::ScopeId, symbol::SymbolId};
use self_cell::self_cell;

use crate::NodeId;

/// Same-name symbols in one scope, in declaration order.
/// Java's type, method, and variable namespaces (JLS §6.5) and positional
/// visibility parts require a candidate list.
type ScopeBindings<'cell> = ArenaIdentHashMap<'cell, ArenaVec<'cell, SymbolId>>;

self_cell!(
    pub(crate) struct ScopingCell<'a> {
        // The dependent borrows names from the AST as well as arena storage.
        owner: (Allocator, PhantomData<&'a str>),
        #[covariant]
        dependent: ScopingInner,
    }
);

impl fmt::Debug for ScopingCell<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.borrow_dependent().fmt(f)
    }
}

#[derive(Debug)]
pub struct ScopingInner<'cell> {
    scope_children: ArenaVec<'cell, ArenaVec<'cell, ScopeId>>,
    symbol_parts: ArenaVec<'cell, ArenaVec<'cell, Span>>,
    resolved_references: ArenaVec<'cell, ArenaVec<'cell, ReferenceId>>,
    pub(crate) bindings: IndexVec<ScopeId, ScopeBindings<'cell>>,
    /// Redeclarations of a symbol.
    ///
    /// Once a symbol is redeclared, there are at least two entries here. The first
    /// entry is the original declaration, and the rest are redeclarations.
    symbol_redeclarations: ArenaHashMap<'cell, SymbolId, ArenaVec<'cell, Redeclaration>>,
    declaration_symbols: ArenaHashMap<'cell, NodeId, ArenaVec<'cell, SymbolId>>,
}

// These relationship families have complete flat owners before indexing.
// Size each list exactly so arena growth does not retain abandoned buffers.
fn relationship_lists<'cell, T: Copy + 'cell>(
    allocator: &'cell Allocator,
    owners: usize,
    relations: impl Iterator<Item = (usize, T)> + Clone,
) -> ArenaVec<'cell, ArenaVec<'cell, T>> {
    let mut counts = vec![0; owners];
    for (owner, _) in relations.clone() {
        counts[owner] += 1;
    }
    let mut lists = ArenaVec::from_iter_in(
        counts.into_iter().map(|count| ArenaVec::with_capacity_in(count, &allocator)),
        &allocator,
    );
    for (owner, id) in relations {
        lists[owner].push(id);
    }
    lists
}

impl<'cell> ScopingInner<'cell> {
    fn new(allocator: &'cell Allocator) -> Self {
        Self {
            scope_children: ArenaVec::new_in(&allocator),
            symbol_parts: ArenaVec::new_in(&allocator),
            resolved_references: ArenaVec::new_in(&allocator),
            bindings: IndexVec::new(),
            symbol_redeclarations: ArenaHashMap::new_in(allocator),
            declaration_symbols: ArenaHashMap::new_in(allocator),
        }
    }

    fn index_scopes(&mut self, allocator: &'cell Allocator, scopes: &IndexVec<ScopeId, Scope>) {
        self.scope_children = relationship_lists(
            allocator,
            scopes.len(),
            scopes
                .iter_enumerated()
                .filter_map(|(id, scope)| scope.parent_id.map(|parent| (parent.index(), id))),
        );
    }

    fn index_symbols(
        &mut self,
        allocator: &'cell Allocator,
        symbols: &IndexVec<SymbolId, Symbol<'cell>>,
        parts: &[(SymbolId, Span)],
    ) {
        self.symbol_parts = relationship_lists(
            allocator,
            symbols.len(),
            parts.iter().map(|&(symbol_id, part)| (symbol_id.index(), part)),
        );
        let declared =
            symbols.iter_enumerated().map(|(id, symbol)| (symbol.declaration_node_id, id));
        let redeclared = self.symbol_redeclarations.iter().flat_map(|(&id, redeclarations)| {
            redeclarations[1..].iter().map(move |redeclaration| (redeclaration.declaration, id))
        });
        self.declaration_symbols =
            ArenaHashMap::with_capacity_in(symbols.len() + redeclared.clone().count(), allocator);
        for (node_id, id) in declared.chain(redeclared) {
            self.declaration_symbols
                .entry(node_id)
                .or_insert_with(|| ArenaVec::with_capacity_in(1, &allocator))
                .push(id);
        }
    }

    fn index_references(
        &mut self,
        allocator: &'cell Allocator,
        symbols: usize,
        references: &IndexVec<ReferenceId, Reference<'cell>>,
    ) {
        self.resolved_references = relationship_lists(
            allocator,
            symbols,
            references.iter_enumerated().filter_map(|(id, reference)| {
                reference.symbol_id.map(|symbol| (symbol.index(), id))
            }),
        );
    }
}

/// One lexical region and its relationship to the surrounding scope tree.
#[derive(Debug)]
pub struct Scope {
    pub(crate) parent_id: Option<ScopeId>,
    pub(crate) owner_node_id: NodeId,
    /// The owner's span, which can hold fields outside the scope.
    pub(crate) span: Span,
    pub(crate) is_type: bool,
}

impl Scope {
    pub const fn parent_id(&self) -> Option<ScopeId> {
        self.parent_id
    }

    pub const fn owner_node_id(&self) -> NodeId {
        self.owner_node_id
    }

    pub const fn span(&self) -> Span {
        self.span
    }

    pub const fn is_type(&self) -> bool {
        self.is_type
    }
}

/// The declaration classes needed by file-local lint rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Import(ImportKind),
    Type,
    TypeParameter,
    Method,
    Field,
    RecordComponent,
    EnumConstant,
    Parameter,
    LocalVariable,
    EnhancedForVariable,
    Resource,
    CatchParameter,
    PatternVariable,
}

impl SymbolKind {
    pub const fn is_local_variable(self) -> bool {
        matches!(
            self,
            Self::LocalVariable
                | Self::EnhancedForVariable
                | Self::Resource
                | Self::PatternVariable
        )
    }

    pub(crate) const fn is_variable(self) -> bool {
        matches!(
            self,
            Self::Field
                | Self::RecordComponent
                | Self::EnumConstant
                | Self::Parameter
                | Self::LocalVariable
                | Self::EnhancedForVariable
                | Self::Resource
                | Self::CatchParameter
                | Self::PatternVariable
        )
    }
}

/// One binding, usable by its simple name within [`Scoping::symbol_parts`].
///
/// The methods of one name in a class share a symbol, since only argument types
/// choose between them. The symbol's name span and declaration node are the first
/// method's, and [`Scoping::symbol_declarations`] gives them all.
#[derive(Debug)]
pub struct Symbol<'a> {
    pub(crate) name: Ident<'a>,
    pub(crate) name_span: Span,
    pub(crate) kind: SymbolKind,
    pub(crate) scope_id: ScopeId,
    pub(crate) declaration_node_id: NodeId,
    pub(crate) is_private: bool,
    pub(crate) shadows: Option<SymbolId>,
}

impl<'a> Symbol<'a> {
    pub fn name(&self) -> &'a str {
        self.name.as_str()
    }

    /// The name with its precomputed hash, for [`Scoping::symbols_for_name`].
    pub const fn ident(&self) -> Ident<'a> {
        self.name
    }

    pub const fn name_span(&self) -> Span {
        self.name_span
    }

    pub const fn kind(&self) -> SymbolKind {
        self.kind
    }

    pub const fn scope_id(&self) -> ScopeId {
        self.scope_id
    }

    pub const fn declaration_node_id(&self) -> NodeId {
        self.declaration_node_id
    }

    /// Whether every declaration of the symbol is private.
    pub const fn is_private(&self) -> bool {
        self.is_private
    }

    pub const fn shadows(&self) -> Option<SymbolId> {
        self.shadows
    }
}

/// One of the declarations a symbol shares.
#[derive(Debug, Clone, Copy)]
pub struct Redeclaration {
    pub span: Span,
    pub declaration: NodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceKind {
    Read,
    Write,
    ReadWrite,
    Type,
    Method,
}

impl ReferenceKind {
    pub const fn is_read(self) -> bool {
        !matches!(self, Self::Write)
    }
}

/// One simple-name use and its file-local resolution, if any.
#[derive(Debug)]
pub struct Reference<'a> {
    pub(crate) name: Ident<'a>,
    pub(crate) span: Span,
    pub(crate) kind: ReferenceKind,
    pub(crate) scope_id: ScopeId,
    pub(crate) node_id: NodeId,
    pub(crate) symbol_id: Option<SymbolId>,
}

impl<'a> Reference<'a> {
    pub fn name(&self) -> &'a str {
        self.name.as_str()
    }

    pub const fn span(&self) -> Span {
        self.span
    }

    pub const fn kind(&self) -> ReferenceKind {
        self.kind
    }

    pub const fn scope_id(&self) -> ScopeId {
        self.scope_id
    }

    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub const fn symbol_id(&self) -> Option<SymbolId> {
        self.symbol_id
    }
}

/// Scope tree, symbol table, and resolved references for one compilation unit.
///
/// Alongside the tables, two lookup indexes answer the queries that name
/// resolution and the lint rules ask repeatedly, so neither of them scans a whole
/// table: `bindings` maps a scope's simple names to their declared symbols, and
/// `declaration_symbols` maps a declaration node to the symbols it introduces.
#[derive(Debug)]
pub struct Scoping<'a> {
    pub(crate) scopes: IndexVec<ScopeId, Scope>,
    pub(crate) symbols: IndexVec<SymbolId, Symbol<'a>>,
    pub(crate) references: IndexVec<ReferenceId, Reference<'a>>,
    pub(crate) cell: ScopingCell<'a>,
}

impl Default for Scoping<'_> {
    fn default() -> Self {
        Self {
            scopes: IndexVec::new(),
            symbols: IndexVec::new(),
            references: IndexVec::new(),
            cell: ScopingCell::new((Allocator::new(), PhantomData), |(allocator, _)| {
                ScopingInner::new(allocator)
            }),
        }
    }
}

impl<'a> Scoping<'a> {
    pub fn scopes(&self) -> &[Scope] {
        self.scopes.as_raw_slice()
    }

    pub fn symbols(&self) -> &[Symbol<'a>] {
        self.symbols.as_raw_slice()
    }

    pub fn symbols_enumerated(
        &self,
    ) -> impl ExactSizeIterator<Item = (SymbolId, &Symbol<'a>)> + '_ {
        self.symbols.iter_enumerated()
    }

    pub fn references(&self) -> &[Reference<'a>] {
        self.references.as_raw_slice()
    }

    pub fn scope(&self, id: ScopeId) -> &Scope {
        &self.scopes[id]
    }

    pub fn symbol(&self, id: SymbolId) -> &Symbol<'a> {
        &self.symbols[id]
    }

    pub fn reference(&self, id: ReferenceId) -> &Reference<'a> {
        &self.references[id]
    }

    /// Child scopes in source order.
    pub fn scope_children(&self, id: ScopeId) -> &[ScopeId] {
        &self.cell.borrow_dependent().scope_children[id.index()]
    }

    /// The parts of its scope where a symbol's simple name refers to it, in source order.
    pub fn symbol_parts(&self, id: SymbolId) -> &[Span] {
        &self.cell.borrow_dependent().symbol_parts[id.index()]
    }

    /// References resolved to a symbol, in source order.
    pub fn resolved_reference_ids(&self, id: SymbolId) -> &[ReferenceId] {
        &self.cell.borrow_dependent().resolved_references[id.index()]
    }

    /// Get all redeclarations associated with a symbol.
    pub fn symbol_redeclarations(&self, id: SymbolId) -> &[Redeclaration] {
        self.cell
            .borrow_dependent()
            .symbol_redeclarations
            .get(&id)
            .map_or(&[], |redeclarations| &redeclarations[..])
    }

    /// Get all declaration node IDs for a symbol, in source order.
    pub fn symbol_declarations(&self, id: SymbolId) -> impl Iterator<Item = NodeId> + '_ {
        let redeclarations = self.symbol_redeclarations(id);
        std::iter::once(self.symbols[id].declaration_node_id)
            .filter(move |_| redeclarations.is_empty())
            .chain(redeclarations.iter().map(|redeclaration| redeclaration.declaration))
    }

    pub fn symbols_for_declaration(
        &self,
        node_id: NodeId,
    ) -> impl Iterator<Item = (SymbolId, &Symbol<'a>)> + '_ {
        self.cell
            .borrow_dependent()
            .declaration_symbols
            .get(&node_id)
            .into_iter()
            .flatten()
            .map(move |&symbol_id| (symbol_id, &self.symbols[symbol_id]))
    }

    /// Same-name symbols in a scope, in source order.
    pub fn symbols_for_name(
        &self,
        scope_id: ScopeId,
        name: Ident<'a>,
    ) -> impl Iterator<Item = (SymbolId, &Symbol<'a>)> + Clone + '_ {
        self.cell.borrow_dependent().bindings[scope_id]
            .get(&name)
            .into_iter()
            .flatten()
            .map(move |&symbol_id| (symbol_id, &self.symbols[symbol_id]))
    }

    /// The references resolved to `symbol_id`, in source order.
    pub fn references_for_symbol(
        &self,
        symbol_id: SymbolId,
    ) -> impl DoubleEndedIterator<Item = &Reference<'a>> + '_ {
        self.resolved_reference_ids(symbol_id)
            .iter()
            .map(|&reference_id| self.reference(reference_id))
    }

    /// A plain assignment does not make a local's value observed.
    pub fn symbol_is_read(&self, symbol_id: SymbolId) -> bool {
        self.references_for_symbol(symbol_id).any(|reference| reference.kind.is_read())
    }

    pub fn symbol_is_referenced(&self, symbol_id: SymbolId) -> bool {
        !self.resolved_reference_ids(symbol_id).is_empty()
    }

    /// The scope and its ancestors, innermost first.
    pub(crate) fn scope_ancestors(&self, scope_id: ScopeId) -> impl Iterator<Item = ScopeId> + '_ {
        std::iter::successors(Some(scope_id), |&id| self.scope(id).parent_id)
    }

    pub(crate) fn add_scope(
        &mut self,
        parent_id: Option<ScopeId>,
        node_id: NodeId,
        span: Span,
        is_type: bool,
    ) -> ScopeId {
        self.cell.with_dependent_mut(|(allocator, _), inner| {
            inner.bindings.push(ArenaHashMap::new_in(allocator));
        });
        self.scopes.push(Scope { parent_id, owner_node_id: node_id, span, is_type })
    }

    pub(crate) fn create_symbol_with_binding(&mut self, symbol: Symbol<'a>) -> SymbolId {
        let (name, scope_id) = (symbol.name, symbol.scope_id);
        let symbol_id = self.symbols.push(symbol);
        self.cell.with_dependent_mut(|(allocator, _), inner| {
            inner.bindings[scope_id]
                .entry(name)
                .or_insert_with(|| ArenaVec::with_capacity_in(1, &allocator))
                .push(symbol_id);
        });
        symbol_id
    }

    /// Record a redeclaration for an existing symbol.
    pub(crate) fn add_symbol_redeclaration(
        &mut self,
        symbol_id: SymbolId,
        declaration: NodeId,
        span: Span,
    ) {
        let symbol = &self.symbols[symbol_id];
        let first =
            Redeclaration { span: symbol.name_span, declaration: symbol.declaration_node_id };
        self.cell.with_dependent_mut(|(allocator, _), inner| {
            let redeclaration = Redeclaration { span, declaration };
            match inner.symbol_redeclarations.entry(symbol_id) {
                Entry::Occupied(occupied) => occupied.into_mut().push(redeclaration),
                Entry::Vacant(vacant) => {
                    vacant.insert(ArenaVec::from_iter_in([first, redeclaration], &allocator));
                }
            }
        });
    }

    pub(crate) fn create_reference(&mut self, reference: Reference<'a>) -> ReferenceId {
        self.references.push(reference)
    }

    pub(crate) fn index_scopes(&mut self) {
        self.cell.with_dependent_mut(|(allocator, _), inner| {
            inner.index_scopes(allocator, &self.scopes);
        });
    }

    pub(crate) fn index_symbols(&mut self, parts: &[(SymbolId, Span)]) {
        self.cell.with_dependent_mut(|(allocator, _), inner| {
            inner.index_symbols(allocator, &self.symbols, parts);
        });
    }

    pub(crate) fn pop_symbol_part(&mut self, id: SymbolId) {
        self.cell.with_dependent_mut(|_, inner| {
            inner.symbol_parts[id.index()].pop();
        });
    }

    pub(crate) fn index_references(&mut self) {
        self.cell.with_dependent_mut(|(allocator, _), inner| {
            inner.index_references(allocator, self.symbols.len(), &self.references);
        });
    }
}

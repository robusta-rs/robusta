// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/get_id.rs`.

#![expect(clippy::inline_always)]

use robusta_syntax::{node::NodeId, reference::ReferenceId, scope::ScopeId, symbol::SymbolId};

use crate::ast::*;

impl CompilationUnit<'_> {
    /// Get [`NodeId`] of [`CompilationUnit`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`CompilationUnit`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`CompilationUnit`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`CompilationUnit`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl OrdinaryCompilationUnit<'_> {
    /// Get [`NodeId`] of [`OrdinaryCompilationUnit`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`OrdinaryCompilationUnit`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ModularCompilationUnit<'_> {
    /// Get [`NodeId`] of [`ModularCompilationUnit`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ModularCompilationUnit`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl PackageDeclaration<'_> {
    /// Get [`NodeId`] of [`PackageDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`PackageDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ImportDeclaration<'_> {
    /// Get [`NodeId`] of [`ImportDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ImportDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl CompactCompilationUnit<'_> {
    /// Get [`NodeId`] of [`CompactCompilationUnit`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`CompactCompilationUnit`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`CompactCompilationUnit`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`CompactCompilationUnit`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl ModuleDeclaration<'_> {
    /// Get [`NodeId`] of [`ModuleDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ModuleDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl RequiresDirective<'_> {
    /// Get [`NodeId`] of [`RequiresDirective`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`RequiresDirective`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl PackageDirective<'_> {
    /// Get [`NodeId`] of [`PackageDirective`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`PackageDirective`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl UsesDirective<'_> {
    /// Get [`NodeId`] of [`UsesDirective`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`UsesDirective`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ProvidesDirective<'_> {
    /// Get [`NodeId`] of [`ProvidesDirective`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ProvidesDirective`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ClassDeclaration<'_> {
    /// Get [`NodeId`] of [`ClassDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ClassDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`ClassDeclaration`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`ClassDeclaration`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl ClassBody<'_> {
    /// Get [`NodeId`] of [`ClassBody`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ClassBody`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`ClassBody`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`ClassBody`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl InterfaceDeclaration<'_> {
    /// Get [`NodeId`] of [`InterfaceDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`InterfaceDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`InterfaceDeclaration`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`InterfaceDeclaration`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl InterfaceBody<'_> {
    /// Get [`NodeId`] of [`InterfaceBody`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`InterfaceBody`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`InterfaceBody`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`InterfaceBody`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl EnumDeclaration<'_> {
    /// Get [`NodeId`] of [`EnumDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`EnumDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl EnumBody<'_> {
    /// Get [`NodeId`] of [`EnumBody`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`EnumBody`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`EnumBody`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`EnumBody`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl EnumConstant<'_> {
    /// Get [`NodeId`] of [`EnumConstant`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`EnumConstant`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl RecordDeclaration<'_> {
    /// Get [`NodeId`] of [`RecordDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`RecordDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`RecordDeclaration`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`RecordDeclaration`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl RecordHeader<'_> {
    /// Get [`NodeId`] of [`RecordHeader`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`RecordHeader`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl RecordComponent<'_> {
    /// Get [`NodeId`] of [`RecordComponent`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`RecordComponent`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl AnnotationInterfaceDeclaration<'_> {
    /// Get [`NodeId`] of [`AnnotationInterfaceDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`AnnotationInterfaceDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl AnnotationInterfaceBody<'_> {
    /// Get [`NodeId`] of [`AnnotationInterfaceBody`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`AnnotationInterfaceBody`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`AnnotationInterfaceBody`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`AnnotationInterfaceBody`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl AnnotationInterfaceElementDeclaration<'_> {
    /// Get [`NodeId`] of [`AnnotationInterfaceElementDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`AnnotationInterfaceElementDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl InitializerBlock<'_> {
    /// Get [`NodeId`] of [`InitializerBlock`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`InitializerBlock`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl FieldDeclaration<'_> {
    /// Get [`NodeId`] of [`FieldDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`FieldDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl MethodDeclaration<'_> {
    /// Get [`NodeId`] of [`MethodDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`MethodDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`MethodDeclaration`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`MethodDeclaration`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl ConstructorDeclaration<'_> {
    /// Get [`NodeId`] of [`ConstructorDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ConstructorDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`ConstructorDeclaration`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`ConstructorDeclaration`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl CompactConstructorDeclaration<'_> {
    /// Get [`NodeId`] of [`CompactConstructorDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`CompactConstructorDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`CompactConstructorDeclaration`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`CompactConstructorDeclaration`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl ReceiverParameter<'_> {
    /// Get [`NodeId`] of [`ReceiverParameter`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ReceiverParameter`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl FormalParameters<'_> {
    /// Get [`NodeId`] of [`FormalParameters`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`FormalParameters`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl FormalParameter<'_> {
    /// Get [`NodeId`] of [`FormalParameter`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`FormalParameter`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl Block<'_> {
    /// Get [`NodeId`] of [`Block`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`Block`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`Block`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`Block`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl EmptyStatement {
    /// Get [`NodeId`] of [`EmptyStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`EmptyStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl IfStatement<'_> {
    /// Get [`NodeId`] of [`IfStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`IfStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl SwitchStatement<'_> {
    /// Get [`NodeId`] of [`SwitchStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`SwitchStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`SwitchStatement`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`SwitchStatement`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl SwitchRule<'_> {
    /// Get [`NodeId`] of [`SwitchRule`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`SwitchRule`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl SwitchCase<'_> {
    /// Get [`NodeId`] of [`SwitchCase`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`SwitchCase`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl CaseLabel<'_> {
    /// Get [`NodeId`] of [`CaseLabel`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`CaseLabel`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl CasePatternLabel<'_> {
    /// Get [`NodeId`] of [`CasePatternLabel`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`CasePatternLabel`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl WhileStatement<'_> {
    /// Get [`NodeId`] of [`WhileStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`WhileStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl DoStatement<'_> {
    /// Get [`NodeId`] of [`DoStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`DoStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ForStatement<'_> {
    /// Get [`NodeId`] of [`ForStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ForStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`ForStatement`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`ForStatement`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl StatementExpressionList<'_> {
    /// Get [`NodeId`] of [`StatementExpressionList`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`StatementExpressionList`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl EnhancedForStatement<'_> {
    /// Get [`NodeId`] of [`EnhancedForStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`EnhancedForStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`EnhancedForStatement`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`EnhancedForStatement`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl BreakStatement<'_> {
    /// Get [`NodeId`] of [`BreakStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`BreakStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ContinueStatement<'_> {
    /// Get [`NodeId`] of [`ContinueStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ContinueStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ThrowStatement<'_> {
    /// Get [`NodeId`] of [`ThrowStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ThrowStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl AssertStatement<'_> {
    /// Get [`NodeId`] of [`AssertStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`AssertStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl TryStatement<'_> {
    /// Get [`NodeId`] of [`TryStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`TryStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`TryStatement`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`TryStatement`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl CatchClause<'_> {
    /// Get [`NodeId`] of [`CatchClause`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`CatchClause`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`CatchClause`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`CatchClause`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl CatchParameter<'_> {
    /// Get [`NodeId`] of [`CatchParameter`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`CatchParameter`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl UnionType<'_> {
    /// Get [`NodeId`] of [`UnionType`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`UnionType`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl SynchronizedStatement<'_> {
    /// Get [`NodeId`] of [`SynchronizedStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`SynchronizedStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl LabeledStatement<'_> {
    /// Get [`NodeId`] of [`LabeledStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`LabeledStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ConstructorInvocation<'_> {
    /// Get [`NodeId`] of [`ConstructorInvocation`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ConstructorInvocation`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ExpressionStatement<'_> {
    /// Get [`NodeId`] of [`ExpressionStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ExpressionStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl LocalVariableDeclaration<'_> {
    /// Get [`NodeId`] of [`LocalVariableDeclaration`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`LocalVariableDeclaration`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ReturnStatement<'_> {
    /// Get [`NodeId`] of [`ReturnStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ReturnStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl YieldStatement<'_> {
    /// Get [`NodeId`] of [`YieldStatement`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`YieldStatement`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl VariableDeclarator<'_> {
    /// Get [`NodeId`] of [`VariableDeclarator`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`VariableDeclarator`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ArrayInitializer<'_> {
    /// Get [`NodeId`] of [`ArrayInitializer`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ArrayInitializer`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl FieldAccess<'_> {
    /// Get [`NodeId`] of [`FieldAccess`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`FieldAccess`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl MethodCall<'_> {
    /// Get [`NodeId`] of [`MethodCall`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`MethodCall`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl MethodMember<'_> {
    /// Get [`NodeId`] of [`MethodMember`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`MethodMember`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ClassLiteral<'_> {
    /// Get [`NodeId`] of [`ClassLiteral`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ClassLiteral`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ThisExpression {
    /// Get [`NodeId`] of [`ThisExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ThisExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl Super {
    /// Get [`NodeId`] of [`Super`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`Super`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl QualifiedThis<'_> {
    /// Get [`NodeId`] of [`QualifiedThis`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`QualifiedThis`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl QualifiedSuper<'_> {
    /// Get [`NodeId`] of [`QualifiedSuper`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`QualifiedSuper`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ClassInstanceCreation<'_> {
    /// Get [`NodeId`] of [`ClassInstanceCreation`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ClassInstanceCreation`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl QualifiedClassInstanceCreation<'_> {
    /// Get [`NodeId`] of [`QualifiedClassInstanceCreation`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`QualifiedClassInstanceCreation`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ArrayCreation<'_> {
    /// Get [`NodeId`] of [`ArrayCreation`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ArrayCreation`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ArrayDimension<'_> {
    /// Get [`NodeId`] of [`ArrayDimension`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ArrayDimension`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ArrayAccess<'_> {
    /// Get [`NodeId`] of [`ArrayAccess`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ArrayAccess`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl UpdateExpression<'_> {
    /// Get [`NodeId`] of [`UpdateExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`UpdateExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl UnaryExpression<'_> {
    /// Get [`NodeId`] of [`UnaryExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`UnaryExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl CastExpression<'_> {
    /// Get [`NodeId`] of [`CastExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`CastExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl InstanceofExpression<'_> {
    /// Get [`NodeId`] of [`InstanceofExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`InstanceofExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl TypePattern<'_> {
    /// Get [`NodeId`] of [`TypePattern`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`TypePattern`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl RecordPattern<'_> {
    /// Get [`NodeId`] of [`RecordPattern`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`RecordPattern`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl MatchAllPattern {
    /// Get [`NodeId`] of [`MatchAllPattern`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`MatchAllPattern`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl BinaryExpression<'_> {
    /// Get [`NodeId`] of [`BinaryExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`BinaryExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl AssignmentExpression<'_> {
    /// Get [`NodeId`] of [`AssignmentExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`AssignmentExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ConditionalExpression<'_> {
    /// Get [`NodeId`] of [`ConditionalExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ConditionalExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl SwitchExpression<'_> {
    /// Get [`NodeId`] of [`SwitchExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`SwitchExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`SwitchExpression`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`SwitchExpression`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl ParenthesizedExpression<'_> {
    /// Get [`NodeId`] of [`ParenthesizedExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ParenthesizedExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl LambdaExpression<'_> {
    /// Get [`NodeId`] of [`LambdaExpression`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`LambdaExpression`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ScopeId`] of [`LambdaExpression`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `scope_id` is [`None`].
    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id.get().unwrap()
    }

    /// Set [`ScopeId`] of [`LambdaExpression`].
    #[inline]
    pub fn set_scope_id(&self, scope_id: ScopeId) {
        self.scope_id.set(Some(scope_id));
    }
}

impl MethodReference<'_> {
    /// Get [`NodeId`] of [`MethodReference`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`MethodReference`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl TypeParameters<'_> {
    /// Get [`NodeId`] of [`TypeParameters`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`TypeParameters`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl TypeParameter<'_> {
    /// Get [`NodeId`] of [`TypeParameter`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`TypeParameter`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl VarType {
    /// Get [`NodeId`] of [`VarType`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`VarType`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl VoidType {
    /// Get [`NodeId`] of [`VoidType`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`VoidType`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl PrimitiveType<'_> {
    /// Get [`NodeId`] of [`PrimitiveType`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`PrimitiveType`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl SimpleClassType<'_> {
    /// Get [`NodeId`] of [`SimpleClassType`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`SimpleClassType`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl QualifiedClassType<'_> {
    /// Get [`NodeId`] of [`QualifiedClassType`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`QualifiedClassType`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ArrayType<'_> {
    /// Get [`NodeId`] of [`ArrayType`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ArrayType`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ArrayTypeDimension<'_> {
    /// Get [`NodeId`] of [`ArrayTypeDimension`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ArrayTypeDimension`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl TypeArguments<'_> {
    /// Get [`NodeId`] of [`TypeArguments`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`TypeArguments`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl WildcardTypeArgument<'_> {
    /// Get [`NodeId`] of [`WildcardTypeArgument`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`WildcardTypeArgument`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl WildcardBound<'_> {
    /// Get [`NodeId`] of [`WildcardBound`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`WildcardBound`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl IntLiteral<'_> {
    /// Get [`NodeId`] of [`IntLiteral`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`IntLiteral`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl FloatLiteral<'_> {
    /// Get [`NodeId`] of [`FloatLiteral`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`FloatLiteral`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl CharLiteral<'_> {
    /// Get [`NodeId`] of [`CharLiteral`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`CharLiteral`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl StringLiteral<'_> {
    /// Get [`NodeId`] of [`StringLiteral`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`StringLiteral`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl BooleanLiteral {
    /// Get [`NodeId`] of [`BooleanLiteral`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`BooleanLiteral`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl NullLiteral {
    /// Get [`NodeId`] of [`NullLiteral`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`NullLiteral`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl IdentifierName<'_> {
    /// Get [`NodeId`] of [`IdentifierName`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`IdentifierName`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl IdentifierReference<'_> {
    /// Get [`NodeId`] of [`IdentifierReference`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`IdentifierReference`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`ReferenceId`] of [`IdentifierReference`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `reference_id` is [`None`].
    #[inline]
    pub fn reference_id(&self) -> ReferenceId {
        self.reference_id.get().unwrap()
    }

    /// Set [`ReferenceId`] of [`IdentifierReference`].
    #[inline]
    pub fn set_reference_id(&self, reference_id: ReferenceId) {
        self.reference_id.set(Some(reference_id));
    }
}

impl BindingIdentifier<'_> {
    /// Get [`NodeId`] of [`BindingIdentifier`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`BindingIdentifier`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }

    /// Get [`SymbolId`] of [`BindingIdentifier`].
    ///
    /// Only use this method after semantic analysis.
    ///
    /// # Panics
    /// Panics if `symbol_id` is [`None`].
    #[inline]
    pub fn symbol_id(&self) -> SymbolId {
        self.symbol_id.get().unwrap()
    }

    /// Set [`SymbolId`] of [`BindingIdentifier`].
    #[inline]
    pub fn set_symbol_id(&self, symbol_id: SymbolId) {
        self.symbol_id.set(Some(symbol_id));
    }
}

impl LabelIdentifier<'_> {
    /// Get [`NodeId`] of [`LabelIdentifier`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`LabelIdentifier`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl QualifiedName<'_> {
    /// Get [`NodeId`] of [`QualifiedName`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`QualifiedName`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl QualifiedTypeName<'_> {
    /// Get [`NodeId`] of [`QualifiedTypeName`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`QualifiedTypeName`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl Annotation<'_> {
    /// Get [`NodeId`] of [`Annotation`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`Annotation`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ElementValuePair<'_> {
    /// Get [`NodeId`] of [`ElementValuePair`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ElementValuePair`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl ElementValueArrayInitializer<'_> {
    /// Get [`NodeId`] of [`ElementValueArrayInitializer`].
    #[inline]
    pub fn node_id(&self) -> NodeId {
        self.node_id.get()
    }

    /// Set [`NodeId`] of [`ElementValueArrayInitializer`].
    #[inline]
    pub fn set_node_id(&self, node_id: NodeId) {
        self.node_id.set(node_id);
    }
}

impl CompilationUnitContent<'_> {
    /// Get [`NodeId`] of [`CompilationUnitContent`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Ordinary(it) => it.node_id(),
            Self::Compact(it) => it.node_id(),
            Self::Modular(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`CompilationUnitContent`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Ordinary(it) => it.set_node_id(node_id),
            Self::Compact(it) => it.set_node_id(node_id),
            Self::Modular(it) => it.set_node_id(node_id),
        }
    }
}

impl ModuleDirective<'_> {
    /// Get [`NodeId`] of [`ModuleDirective`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Requires(it) => it.node_id(),
            Self::Package(it) => it.node_id(),
            Self::Uses(it) => it.node_id(),
            Self::Provides(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`ModuleDirective`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Requires(it) => it.set_node_id(node_id),
            Self::Package(it) => it.set_node_id(node_id),
            Self::Uses(it) => it.set_node_id(node_id),
            Self::Provides(it) => it.set_node_id(node_id),
        }
    }
}

impl TypeDeclaration<'_> {
    /// Get [`NodeId`] of [`TypeDeclaration`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Empty(it) => it.node_id(),
            Self::AnnotationInterface(it) => it.node_id(),
            Self::Class(it) => it.node_id(),
            Self::Interface(it) => it.node_id(),
            Self::Enum(it) => it.node_id(),
            Self::Record(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`TypeDeclaration`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Empty(it) => it.set_node_id(node_id),
            Self::AnnotationInterface(it) => it.set_node_id(node_id),
            Self::Class(it) => it.set_node_id(node_id),
            Self::Interface(it) => it.set_node_id(node_id),
            Self::Enum(it) => it.set_node_id(node_id),
            Self::Record(it) => it.set_node_id(node_id),
        }
    }
}

impl LocalTypeDeclaration<'_> {
    /// Get [`NodeId`] of [`LocalTypeDeclaration`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Class(it) => it.node_id(),
            Self::Interface(it) => it.node_id(),
            Self::Enum(it) => it.node_id(),
            Self::Record(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`LocalTypeDeclaration`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Class(it) => it.set_node_id(node_id),
            Self::Interface(it) => it.set_node_id(node_id),
            Self::Enum(it) => it.set_node_id(node_id),
            Self::Record(it) => it.set_node_id(node_id),
        }
    }
}

impl AnnotationInterfaceMember<'_> {
    /// Get [`NodeId`] of [`AnnotationInterfaceMember`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Element(it) => it.node_id(),
            Self::Field(it) => it.node_id(),
            Self::Empty(it) => it.node_id(),
            Self::AnnotationInterface(it) => it.node_id(),
            Self::Class(it) => it.node_id(),
            Self::Interface(it) => it.node_id(),
            Self::Enum(it) => it.node_id(),
            Self::Record(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`AnnotationInterfaceMember`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Element(it) => it.set_node_id(node_id),
            Self::Field(it) => it.set_node_id(node_id),
            Self::Empty(it) => it.set_node_id(node_id),
            Self::AnnotationInterface(it) => it.set_node_id(node_id),
            Self::Class(it) => it.set_node_id(node_id),
            Self::Interface(it) => it.set_node_id(node_id),
            Self::Enum(it) => it.set_node_id(node_id),
            Self::Record(it) => it.set_node_id(node_id),
        }
    }
}

impl ClassMember<'_> {
    /// Get [`NodeId`] of [`ClassMember`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Field(it) => it.node_id(),
            Self::Method(it) => it.node_id(),
            Self::Constructor(it) => it.node_id(),
            Self::CompactConstructor(it) => it.node_id(),
            Self::Initializer(it) => it.node_id(),
            Self::Empty(it) => it.node_id(),
            Self::AnnotationInterface(it) => it.node_id(),
            Self::Class(it) => it.node_id(),
            Self::Interface(it) => it.node_id(),
            Self::Enum(it) => it.node_id(),
            Self::Record(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`ClassMember`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Field(it) => it.set_node_id(node_id),
            Self::Method(it) => it.set_node_id(node_id),
            Self::Constructor(it) => it.set_node_id(node_id),
            Self::CompactConstructor(it) => it.set_node_id(node_id),
            Self::Initializer(it) => it.set_node_id(node_id),
            Self::Empty(it) => it.set_node_id(node_id),
            Self::AnnotationInterface(it) => it.set_node_id(node_id),
            Self::Class(it) => it.set_node_id(node_id),
            Self::Interface(it) => it.set_node_id(node_id),
            Self::Enum(it) => it.set_node_id(node_id),
            Self::Record(it) => it.set_node_id(node_id),
        }
    }
}

impl InterfaceMember<'_> {
    /// Get [`NodeId`] of [`InterfaceMember`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Constant(it) => it.node_id(),
            Self::Method(it) => it.node_id(),
            Self::Empty(it) => it.node_id(),
            Self::AnnotationInterface(it) => it.node_id(),
            Self::Class(it) => it.node_id(),
            Self::Interface(it) => it.node_id(),
            Self::Enum(it) => it.node_id(),
            Self::Record(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`InterfaceMember`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Constant(it) => it.set_node_id(node_id),
            Self::Method(it) => it.set_node_id(node_id),
            Self::Empty(it) => it.set_node_id(node_id),
            Self::AnnotationInterface(it) => it.set_node_id(node_id),
            Self::Class(it) => it.set_node_id(node_id),
            Self::Interface(it) => it.set_node_id(node_id),
            Self::Enum(it) => it.set_node_id(node_id),
            Self::Record(it) => it.set_node_id(node_id),
        }
    }
}

impl Statement<'_> {
    /// Get [`NodeId`] of [`Statement`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Block(it) => it.node_id(),
            Self::Empty(it) => it.node_id(),
            Self::ConstructorInvocation(it) => it.node_id(),
            Self::LocalVariable(it) => it.node_id(),
            Self::Expression(it) => it.node_id(),
            Self::If(it) => it.node_id(),
            Self::Switch(it) => it.node_id(),
            Self::While(it) => it.node_id(),
            Self::Do(it) => it.node_id(),
            Self::For(it) => it.node_id(),
            Self::EnhancedFor(it) => it.node_id(),
            Self::Break(it) => it.node_id(),
            Self::Continue(it) => it.node_id(),
            Self::Throw(it) => it.node_id(),
            Self::Assert(it) => it.node_id(),
            Self::Try(it) => it.node_id(),
            Self::Synchronized(it) => it.node_id(),
            Self::Return(it) => it.node_id(),
            Self::Yield(it) => it.node_id(),
            Self::Labeled(it) => it.node_id(),
            Self::Class(it) => it.node_id(),
            Self::Interface(it) => it.node_id(),
            Self::Enum(it) => it.node_id(),
            Self::Record(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`Statement`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Block(it) => it.set_node_id(node_id),
            Self::Empty(it) => it.set_node_id(node_id),
            Self::ConstructorInvocation(it) => it.set_node_id(node_id),
            Self::LocalVariable(it) => it.set_node_id(node_id),
            Self::Expression(it) => it.set_node_id(node_id),
            Self::If(it) => it.set_node_id(node_id),
            Self::Switch(it) => it.set_node_id(node_id),
            Self::While(it) => it.set_node_id(node_id),
            Self::Do(it) => it.set_node_id(node_id),
            Self::For(it) => it.set_node_id(node_id),
            Self::EnhancedFor(it) => it.set_node_id(node_id),
            Self::Break(it) => it.set_node_id(node_id),
            Self::Continue(it) => it.set_node_id(node_id),
            Self::Throw(it) => it.set_node_id(node_id),
            Self::Assert(it) => it.set_node_id(node_id),
            Self::Try(it) => it.set_node_id(node_id),
            Self::Synchronized(it) => it.set_node_id(node_id),
            Self::Return(it) => it.set_node_id(node_id),
            Self::Yield(it) => it.set_node_id(node_id),
            Self::Labeled(it) => it.set_node_id(node_id),
            Self::Class(it) => it.set_node_id(node_id),
            Self::Interface(it) => it.set_node_id(node_id),
            Self::Enum(it) => it.set_node_id(node_id),
            Self::Record(it) => it.set_node_id(node_id),
        }
    }
}

impl SwitchItem<'_> {
    /// Get [`NodeId`] of [`SwitchItem`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Rule(it) => it.node_id(),
            Self::Case(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`SwitchItem`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Rule(it) => it.set_node_id(node_id),
            Self::Case(it) => it.set_node_id(node_id),
        }
    }
}

impl SwitchRuleBody<'_> {
    /// Get [`NodeId`] of [`SwitchRuleBody`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Block(it) => it.node_id(),
            Self::Throw(it) => it.node_id(),
            Self::IntLiteral(it) => it.node_id(),
            Self::FloatLiteral(it) => it.node_id(),
            Self::CharLiteral(it) => it.node_id(),
            Self::StringLiteral(it) => it.node_id(),
            Self::BooleanLiteral(it) => it.node_id(),
            Self::NullLiteral(it) => it.node_id(),
            Self::This(it) => it.node_id(),
            Self::Super(it) => it.node_id(),
            Self::QualifiedThis(it) => it.node_id(),
            Self::QualifiedSuper(it) => it.node_id(),
            Self::Identifier(it) => it.node_id(),
            Self::FieldAccess(it) => it.node_id(),
            Self::MethodCall(it) => it.node_id(),
            Self::ClassLiteral(it) => it.node_id(),
            Self::ClassInstanceCreation(it) => it.node_id(),
            Self::QualifiedClassInstanceCreation(it) => it.node_id(),
            Self::ArrayCreation(it) => it.node_id(),
            Self::ArrayAccess(it) => it.node_id(),
            Self::Update(it) => it.node_id(),
            Self::Unary(it) => it.node_id(),
            Self::Cast(it) => it.node_id(),
            Self::Instanceof(it) => it.node_id(),
            Self::Binary(it) => it.node_id(),
            Self::Assignment(it) => it.node_id(),
            Self::Conditional(it) => it.node_id(),
            Self::Switch(it) => it.node_id(),
            Self::Parenthesized(it) => it.node_id(),
            Self::Lambda(it) => it.node_id(),
            Self::MethodReference(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`SwitchRuleBody`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Block(it) => it.set_node_id(node_id),
            Self::Throw(it) => it.set_node_id(node_id),
            Self::IntLiteral(it) => it.set_node_id(node_id),
            Self::FloatLiteral(it) => it.set_node_id(node_id),
            Self::CharLiteral(it) => it.set_node_id(node_id),
            Self::StringLiteral(it) => it.set_node_id(node_id),
            Self::BooleanLiteral(it) => it.set_node_id(node_id),
            Self::NullLiteral(it) => it.set_node_id(node_id),
            Self::This(it) => it.set_node_id(node_id),
            Self::Super(it) => it.set_node_id(node_id),
            Self::QualifiedThis(it) => it.set_node_id(node_id),
            Self::QualifiedSuper(it) => it.set_node_id(node_id),
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::FieldAccess(it) => it.set_node_id(node_id),
            Self::MethodCall(it) => it.set_node_id(node_id),
            Self::ClassLiteral(it) => it.set_node_id(node_id),
            Self::ClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::QualifiedClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::ArrayCreation(it) => it.set_node_id(node_id),
            Self::ArrayAccess(it) => it.set_node_id(node_id),
            Self::Update(it) => it.set_node_id(node_id),
            Self::Unary(it) => it.set_node_id(node_id),
            Self::Cast(it) => it.set_node_id(node_id),
            Self::Instanceof(it) => it.set_node_id(node_id),
            Self::Binary(it) => it.set_node_id(node_id),
            Self::Assignment(it) => it.set_node_id(node_id),
            Self::Conditional(it) => it.set_node_id(node_id),
            Self::Switch(it) => it.set_node_id(node_id),
            Self::Parenthesized(it) => it.set_node_id(node_id),
            Self::Lambda(it) => it.set_node_id(node_id),
            Self::MethodReference(it) => it.set_node_id(node_id),
        }
    }
}

impl SwitchLabel<'_> {
    /// Get [`NodeId`] of [`SwitchLabel`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Case(it) => it.node_id(),
            Self::Pattern(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`SwitchLabel`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Case(it) => it.set_node_id(node_id),
            Self::Pattern(it) => it.set_node_id(node_id),
        }
    }
}

impl ForInit<'_> {
    /// Get [`NodeId`] of [`ForInit`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Declaration(it) => it.node_id(),
            Self::Expressions(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`ForInit`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Declaration(it) => it.set_node_id(node_id),
            Self::Expressions(it) => it.set_node_id(node_id),
        }
    }
}

impl Resource<'_> {
    /// Get [`NodeId`] of [`Resource`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Declaration(it) => it.node_id(),
            Self::Identifier(it) => it.node_id(),
            Self::Field(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`Resource`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Declaration(it) => it.set_node_id(node_id),
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::Field(it) => it.set_node_id(node_id),
        }
    }
}

impl ResourceAccess<'_> {
    /// Get [`NodeId`] of [`ResourceAccess`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Identifier(it) => it.node_id(),
            Self::Field(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`ResourceAccess`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::Field(it) => it.set_node_id(node_id),
        }
    }
}

impl CatchType<'_> {
    /// Get [`NodeId`] of [`CatchType`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Union(it) => it.node_id(),
            Self::Simple(it) => it.node_id(),
            Self::Qualified(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`CatchType`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Union(it) => it.set_node_id(node_id),
            Self::Simple(it) => it.set_node_id(node_id),
            Self::Qualified(it) => it.set_node_id(node_id),
        }
    }
}

impl VariableInitializer<'_> {
    /// Get [`NodeId`] of [`VariableInitializer`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Array(it) => it.node_id(),
            Self::IntLiteral(it) => it.node_id(),
            Self::FloatLiteral(it) => it.node_id(),
            Self::CharLiteral(it) => it.node_id(),
            Self::StringLiteral(it) => it.node_id(),
            Self::BooleanLiteral(it) => it.node_id(),
            Self::NullLiteral(it) => it.node_id(),
            Self::This(it) => it.node_id(),
            Self::Super(it) => it.node_id(),
            Self::QualifiedThis(it) => it.node_id(),
            Self::QualifiedSuper(it) => it.node_id(),
            Self::Identifier(it) => it.node_id(),
            Self::FieldAccess(it) => it.node_id(),
            Self::MethodCall(it) => it.node_id(),
            Self::ClassLiteral(it) => it.node_id(),
            Self::ClassInstanceCreation(it) => it.node_id(),
            Self::QualifiedClassInstanceCreation(it) => it.node_id(),
            Self::ArrayCreation(it) => it.node_id(),
            Self::ArrayAccess(it) => it.node_id(),
            Self::Update(it) => it.node_id(),
            Self::Unary(it) => it.node_id(),
            Self::Cast(it) => it.node_id(),
            Self::Instanceof(it) => it.node_id(),
            Self::Binary(it) => it.node_id(),
            Self::Assignment(it) => it.node_id(),
            Self::Conditional(it) => it.node_id(),
            Self::Switch(it) => it.node_id(),
            Self::Parenthesized(it) => it.node_id(),
            Self::Lambda(it) => it.node_id(),
            Self::MethodReference(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`VariableInitializer`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Array(it) => it.set_node_id(node_id),
            Self::IntLiteral(it) => it.set_node_id(node_id),
            Self::FloatLiteral(it) => it.set_node_id(node_id),
            Self::CharLiteral(it) => it.set_node_id(node_id),
            Self::StringLiteral(it) => it.set_node_id(node_id),
            Self::BooleanLiteral(it) => it.set_node_id(node_id),
            Self::NullLiteral(it) => it.set_node_id(node_id),
            Self::This(it) => it.set_node_id(node_id),
            Self::Super(it) => it.set_node_id(node_id),
            Self::QualifiedThis(it) => it.set_node_id(node_id),
            Self::QualifiedSuper(it) => it.set_node_id(node_id),
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::FieldAccess(it) => it.set_node_id(node_id),
            Self::MethodCall(it) => it.set_node_id(node_id),
            Self::ClassLiteral(it) => it.set_node_id(node_id),
            Self::ClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::QualifiedClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::ArrayCreation(it) => it.set_node_id(node_id),
            Self::ArrayAccess(it) => it.set_node_id(node_id),
            Self::Update(it) => it.set_node_id(node_id),
            Self::Unary(it) => it.set_node_id(node_id),
            Self::Cast(it) => it.set_node_id(node_id),
            Self::Instanceof(it) => it.set_node_id(node_id),
            Self::Binary(it) => it.set_node_id(node_id),
            Self::Assignment(it) => it.set_node_id(node_id),
            Self::Conditional(it) => it.set_node_id(node_id),
            Self::Switch(it) => it.set_node_id(node_id),
            Self::Parenthesized(it) => it.set_node_id(node_id),
            Self::Lambda(it) => it.set_node_id(node_id),
            Self::MethodReference(it) => it.set_node_id(node_id),
        }
    }
}

impl Expression<'_> {
    /// Get [`NodeId`] of [`Expression`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::IntLiteral(it) => it.node_id(),
            Self::FloatLiteral(it) => it.node_id(),
            Self::CharLiteral(it) => it.node_id(),
            Self::StringLiteral(it) => it.node_id(),
            Self::BooleanLiteral(it) => it.node_id(),
            Self::NullLiteral(it) => it.node_id(),
            Self::This(it) => it.node_id(),
            Self::Super(it) => it.node_id(),
            Self::QualifiedThis(it) => it.node_id(),
            Self::QualifiedSuper(it) => it.node_id(),
            Self::Identifier(it) => it.node_id(),
            Self::FieldAccess(it) => it.node_id(),
            Self::MethodCall(it) => it.node_id(),
            Self::ClassLiteral(it) => it.node_id(),
            Self::ClassInstanceCreation(it) => it.node_id(),
            Self::QualifiedClassInstanceCreation(it) => it.node_id(),
            Self::ArrayCreation(it) => it.node_id(),
            Self::ArrayAccess(it) => it.node_id(),
            Self::Update(it) => it.node_id(),
            Self::Unary(it) => it.node_id(),
            Self::Cast(it) => it.node_id(),
            Self::Instanceof(it) => it.node_id(),
            Self::Binary(it) => it.node_id(),
            Self::Assignment(it) => it.node_id(),
            Self::Conditional(it) => it.node_id(),
            Self::Switch(it) => it.node_id(),
            Self::Parenthesized(it) => it.node_id(),
            Self::Lambda(it) => it.node_id(),
            Self::MethodReference(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`Expression`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::IntLiteral(it) => it.set_node_id(node_id),
            Self::FloatLiteral(it) => it.set_node_id(node_id),
            Self::CharLiteral(it) => it.set_node_id(node_id),
            Self::StringLiteral(it) => it.set_node_id(node_id),
            Self::BooleanLiteral(it) => it.set_node_id(node_id),
            Self::NullLiteral(it) => it.set_node_id(node_id),
            Self::This(it) => it.set_node_id(node_id),
            Self::Super(it) => it.set_node_id(node_id),
            Self::QualifiedThis(it) => it.set_node_id(node_id),
            Self::QualifiedSuper(it) => it.set_node_id(node_id),
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::FieldAccess(it) => it.set_node_id(node_id),
            Self::MethodCall(it) => it.set_node_id(node_id),
            Self::ClassLiteral(it) => it.set_node_id(node_id),
            Self::ClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::QualifiedClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::ArrayCreation(it) => it.set_node_id(node_id),
            Self::ArrayAccess(it) => it.set_node_id(node_id),
            Self::Update(it) => it.set_node_id(node_id),
            Self::Unary(it) => it.set_node_id(node_id),
            Self::Cast(it) => it.set_node_id(node_id),
            Self::Instanceof(it) => it.set_node_id(node_id),
            Self::Binary(it) => it.set_node_id(node_id),
            Self::Assignment(it) => it.set_node_id(node_id),
            Self::Conditional(it) => it.set_node_id(node_id),
            Self::Switch(it) => it.set_node_id(node_id),
            Self::Parenthesized(it) => it.set_node_id(node_id),
            Self::Lambda(it) => it.set_node_id(node_id),
            Self::MethodReference(it) => it.set_node_id(node_id),
        }
    }
}

impl Callee<'_> {
    /// Get [`NodeId`] of [`Callee`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Identifier(it) => it.node_id(),
            Self::Member(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`Callee`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::Member(it) => it.set_node_id(node_id),
        }
    }
}

impl InstanceofRhs<'_> {
    /// Get [`NodeId`] of [`InstanceofRhs`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Primitive(it) => it.node_id(),
            Self::Array(it) => it.node_id(),
            Self::Simple(it) => it.node_id(),
            Self::Qualified(it) => it.node_id(),
            Self::TypePattern(it) => it.node_id(),
            Self::RecordPattern(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`InstanceofRhs`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Primitive(it) => it.set_node_id(node_id),
            Self::Array(it) => it.set_node_id(node_id),
            Self::Simple(it) => it.set_node_id(node_id),
            Self::Qualified(it) => it.set_node_id(node_id),
            Self::TypePattern(it) => it.set_node_id(node_id),
            Self::RecordPattern(it) => it.set_node_id(node_id),
        }
    }
}

impl Pattern<'_> {
    /// Get [`NodeId`] of [`Pattern`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::TypePattern(it) => it.node_id(),
            Self::RecordPattern(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`Pattern`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::TypePattern(it) => it.set_node_id(node_id),
            Self::RecordPattern(it) => it.set_node_id(node_id),
        }
    }
}

impl ComponentPattern<'_> {
    /// Get [`NodeId`] of [`ComponentPattern`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::MatchAll(it) => it.node_id(),
            Self::TypePattern(it) => it.node_id(),
            Self::RecordPattern(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`ComponentPattern`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::MatchAll(it) => it.set_node_id(node_id),
            Self::TypePattern(it) => it.set_node_id(node_id),
            Self::RecordPattern(it) => it.set_node_id(node_id),
        }
    }
}

impl AssignmentTarget<'_> {
    /// Get [`NodeId`] of [`AssignmentTarget`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Identifier(it) => it.node_id(),
            Self::FieldAccess(it) => it.node_id(),
            Self::ArrayAccess(it) => it.node_id(),
            Self::Parenthesized(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`AssignmentTarget`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::FieldAccess(it) => it.set_node_id(node_id),
            Self::ArrayAccess(it) => it.set_node_id(node_id),
            Self::Parenthesized(it) => it.set_node_id(node_id),
        }
    }
}

impl LambdaBody<'_> {
    /// Get [`NodeId`] of [`LambdaBody`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Block(it) => it.node_id(),
            Self::IntLiteral(it) => it.node_id(),
            Self::FloatLiteral(it) => it.node_id(),
            Self::CharLiteral(it) => it.node_id(),
            Self::StringLiteral(it) => it.node_id(),
            Self::BooleanLiteral(it) => it.node_id(),
            Self::NullLiteral(it) => it.node_id(),
            Self::This(it) => it.node_id(),
            Self::Super(it) => it.node_id(),
            Self::QualifiedThis(it) => it.node_id(),
            Self::QualifiedSuper(it) => it.node_id(),
            Self::Identifier(it) => it.node_id(),
            Self::FieldAccess(it) => it.node_id(),
            Self::MethodCall(it) => it.node_id(),
            Self::ClassLiteral(it) => it.node_id(),
            Self::ClassInstanceCreation(it) => it.node_id(),
            Self::QualifiedClassInstanceCreation(it) => it.node_id(),
            Self::ArrayCreation(it) => it.node_id(),
            Self::ArrayAccess(it) => it.node_id(),
            Self::Update(it) => it.node_id(),
            Self::Unary(it) => it.node_id(),
            Self::Cast(it) => it.node_id(),
            Self::Instanceof(it) => it.node_id(),
            Self::Binary(it) => it.node_id(),
            Self::Assignment(it) => it.node_id(),
            Self::Conditional(it) => it.node_id(),
            Self::Switch(it) => it.node_id(),
            Self::Parenthesized(it) => it.node_id(),
            Self::Lambda(it) => it.node_id(),
            Self::MethodReference(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`LambdaBody`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Block(it) => it.set_node_id(node_id),
            Self::IntLiteral(it) => it.set_node_id(node_id),
            Self::FloatLiteral(it) => it.set_node_id(node_id),
            Self::CharLiteral(it) => it.set_node_id(node_id),
            Self::StringLiteral(it) => it.set_node_id(node_id),
            Self::BooleanLiteral(it) => it.set_node_id(node_id),
            Self::NullLiteral(it) => it.set_node_id(node_id),
            Self::This(it) => it.set_node_id(node_id),
            Self::Super(it) => it.set_node_id(node_id),
            Self::QualifiedThis(it) => it.set_node_id(node_id),
            Self::QualifiedSuper(it) => it.set_node_id(node_id),
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::FieldAccess(it) => it.set_node_id(node_id),
            Self::MethodCall(it) => it.set_node_id(node_id),
            Self::ClassLiteral(it) => it.set_node_id(node_id),
            Self::ClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::QualifiedClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::ArrayCreation(it) => it.set_node_id(node_id),
            Self::ArrayAccess(it) => it.set_node_id(node_id),
            Self::Update(it) => it.set_node_id(node_id),
            Self::Unary(it) => it.set_node_id(node_id),
            Self::Cast(it) => it.set_node_id(node_id),
            Self::Instanceof(it) => it.set_node_id(node_id),
            Self::Binary(it) => it.set_node_id(node_id),
            Self::Assignment(it) => it.set_node_id(node_id),
            Self::Conditional(it) => it.set_node_id(node_id),
            Self::Switch(it) => it.set_node_id(node_id),
            Self::Parenthesized(it) => it.set_node_id(node_id),
            Self::Lambda(it) => it.set_node_id(node_id),
            Self::MethodReference(it) => it.set_node_id(node_id),
        }
    }
}

impl MethodReferenceTarget<'_> {
    /// Get [`NodeId`] of [`MethodReferenceTarget`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::IntLiteral(it) => it.node_id(),
            Self::FloatLiteral(it) => it.node_id(),
            Self::CharLiteral(it) => it.node_id(),
            Self::StringLiteral(it) => it.node_id(),
            Self::BooleanLiteral(it) => it.node_id(),
            Self::NullLiteral(it) => it.node_id(),
            Self::This(it) => it.node_id(),
            Self::Super(it) => it.node_id(),
            Self::QualifiedThis(it) => it.node_id(),
            Self::QualifiedSuper(it) => it.node_id(),
            Self::Identifier(it) => it.node_id(),
            Self::FieldAccess(it) => it.node_id(),
            Self::MethodCall(it) => it.node_id(),
            Self::ClassLiteral(it) => it.node_id(),
            Self::ClassInstanceCreation(it) => it.node_id(),
            Self::QualifiedClassInstanceCreation(it) => it.node_id(),
            Self::ArrayCreation(it) => it.node_id(),
            Self::ArrayAccess(it) => it.node_id(),
            Self::Update(it) => it.node_id(),
            Self::Unary(it) => it.node_id(),
            Self::Cast(it) => it.node_id(),
            Self::Instanceof(it) => it.node_id(),
            Self::Binary(it) => it.node_id(),
            Self::Assignment(it) => it.node_id(),
            Self::Conditional(it) => it.node_id(),
            Self::Switch(it) => it.node_id(),
            Self::Parenthesized(it) => it.node_id(),
            Self::Lambda(it) => it.node_id(),
            Self::MethodReference(it) => it.node_id(),
            Self::Primitive(it) => it.node_id(),
            Self::Array(it) => it.node_id(),
            Self::Simple(it) => it.node_id(),
            Self::Qualified(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`MethodReferenceTarget`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::IntLiteral(it) => it.set_node_id(node_id),
            Self::FloatLiteral(it) => it.set_node_id(node_id),
            Self::CharLiteral(it) => it.set_node_id(node_id),
            Self::StringLiteral(it) => it.set_node_id(node_id),
            Self::BooleanLiteral(it) => it.set_node_id(node_id),
            Self::NullLiteral(it) => it.set_node_id(node_id),
            Self::This(it) => it.set_node_id(node_id),
            Self::Super(it) => it.set_node_id(node_id),
            Self::QualifiedThis(it) => it.set_node_id(node_id),
            Self::QualifiedSuper(it) => it.set_node_id(node_id),
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::FieldAccess(it) => it.set_node_id(node_id),
            Self::MethodCall(it) => it.set_node_id(node_id),
            Self::ClassLiteral(it) => it.set_node_id(node_id),
            Self::ClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::QualifiedClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::ArrayCreation(it) => it.set_node_id(node_id),
            Self::ArrayAccess(it) => it.set_node_id(node_id),
            Self::Update(it) => it.set_node_id(node_id),
            Self::Unary(it) => it.set_node_id(node_id),
            Self::Cast(it) => it.set_node_id(node_id),
            Self::Instanceof(it) => it.set_node_id(node_id),
            Self::Binary(it) => it.set_node_id(node_id),
            Self::Assignment(it) => it.set_node_id(node_id),
            Self::Conditional(it) => it.set_node_id(node_id),
            Self::Switch(it) => it.set_node_id(node_id),
            Self::Parenthesized(it) => it.set_node_id(node_id),
            Self::Lambda(it) => it.set_node_id(node_id),
            Self::MethodReference(it) => it.set_node_id(node_id),
            Self::Primitive(it) => it.set_node_id(node_id),
            Self::Array(it) => it.set_node_id(node_id),
            Self::Simple(it) => it.set_node_id(node_id),
            Self::Qualified(it) => it.set_node_id(node_id),
        }
    }
}

impl VariableType<'_> {
    /// Get [`NodeId`] of [`VariableType`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Var(it) => it.node_id(),
            Self::Primitive(it) => it.node_id(),
            Self::Array(it) => it.node_id(),
            Self::Simple(it) => it.node_id(),
            Self::Qualified(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`VariableType`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Var(it) => it.set_node_id(node_id),
            Self::Primitive(it) => it.set_node_id(node_id),
            Self::Array(it) => it.set_node_id(node_id),
            Self::Simple(it) => it.set_node_id(node_id),
            Self::Qualified(it) => it.set_node_id(node_id),
        }
    }
}

impl Type<'_> {
    /// Get [`NodeId`] of [`Type`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Primitive(it) => it.node_id(),
            Self::Array(it) => it.node_id(),
            Self::Simple(it) => it.node_id(),
            Self::Qualified(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`Type`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Primitive(it) => it.set_node_id(node_id),
            Self::Array(it) => it.set_node_id(node_id),
            Self::Simple(it) => it.set_node_id(node_id),
            Self::Qualified(it) => it.set_node_id(node_id),
        }
    }
}

impl MethodResult<'_> {
    /// Get [`NodeId`] of [`MethodResult`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Void(it) => it.node_id(),
            Self::Primitive(it) => it.node_id(),
            Self::Array(it) => it.node_id(),
            Self::Simple(it) => it.node_id(),
            Self::Qualified(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`MethodResult`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Void(it) => it.set_node_id(node_id),
            Self::Primitive(it) => it.set_node_id(node_id),
            Self::Array(it) => it.set_node_id(node_id),
            Self::Simple(it) => it.set_node_id(node_id),
            Self::Qualified(it) => it.set_node_id(node_id),
        }
    }
}

impl ClassType<'_> {
    /// Get [`NodeId`] of [`ClassType`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Simple(it) => it.node_id(),
            Self::Qualified(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`ClassType`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Simple(it) => it.set_node_id(node_id),
            Self::Qualified(it) => it.set_node_id(node_id),
        }
    }
}

impl TypeArgument<'_> {
    /// Get [`NodeId`] of [`TypeArgument`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Wildcard(it) => it.node_id(),
            Self::Primitive(it) => it.node_id(),
            Self::Array(it) => it.node_id(),
            Self::Simple(it) => it.node_id(),
            Self::Qualified(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`TypeArgument`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Wildcard(it) => it.set_node_id(node_id),
            Self::Primitive(it) => it.set_node_id(node_id),
            Self::Array(it) => it.set_node_id(node_id),
            Self::Simple(it) => it.set_node_id(node_id),
            Self::Qualified(it) => it.set_node_id(node_id),
        }
    }
}

impl Name<'_> {
    /// Get [`NodeId`] of [`Name`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Identifier(it) => it.node_id(),
            Self::Qualified(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`Name`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::Qualified(it) => it.set_node_id(node_id),
        }
    }
}

impl TypeName<'_> {
    /// Get [`NodeId`] of [`TypeName`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Identifier(it) => it.node_id(),
            Self::Qualified(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`TypeName`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::Qualified(it) => it.set_node_id(node_id),
        }
    }
}

impl AnnotationArgument<'_> {
    /// Get [`NodeId`] of [`AnnotationArgument`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Pair(it) => it.node_id(),
            Self::Annotation(it) => it.node_id(),
            Self::Array(it) => it.node_id(),
            Self::IntLiteral(it) => it.node_id(),
            Self::FloatLiteral(it) => it.node_id(),
            Self::CharLiteral(it) => it.node_id(),
            Self::StringLiteral(it) => it.node_id(),
            Self::BooleanLiteral(it) => it.node_id(),
            Self::NullLiteral(it) => it.node_id(),
            Self::This(it) => it.node_id(),
            Self::Super(it) => it.node_id(),
            Self::QualifiedThis(it) => it.node_id(),
            Self::QualifiedSuper(it) => it.node_id(),
            Self::Identifier(it) => it.node_id(),
            Self::FieldAccess(it) => it.node_id(),
            Self::MethodCall(it) => it.node_id(),
            Self::ClassLiteral(it) => it.node_id(),
            Self::ClassInstanceCreation(it) => it.node_id(),
            Self::QualifiedClassInstanceCreation(it) => it.node_id(),
            Self::ArrayCreation(it) => it.node_id(),
            Self::ArrayAccess(it) => it.node_id(),
            Self::Update(it) => it.node_id(),
            Self::Unary(it) => it.node_id(),
            Self::Cast(it) => it.node_id(),
            Self::Instanceof(it) => it.node_id(),
            Self::Binary(it) => it.node_id(),
            Self::Assignment(it) => it.node_id(),
            Self::Conditional(it) => it.node_id(),
            Self::Switch(it) => it.node_id(),
            Self::Parenthesized(it) => it.node_id(),
            Self::Lambda(it) => it.node_id(),
            Self::MethodReference(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`AnnotationArgument`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Pair(it) => it.set_node_id(node_id),
            Self::Annotation(it) => it.set_node_id(node_id),
            Self::Array(it) => it.set_node_id(node_id),
            Self::IntLiteral(it) => it.set_node_id(node_id),
            Self::FloatLiteral(it) => it.set_node_id(node_id),
            Self::CharLiteral(it) => it.set_node_id(node_id),
            Self::StringLiteral(it) => it.set_node_id(node_id),
            Self::BooleanLiteral(it) => it.set_node_id(node_id),
            Self::NullLiteral(it) => it.set_node_id(node_id),
            Self::This(it) => it.set_node_id(node_id),
            Self::Super(it) => it.set_node_id(node_id),
            Self::QualifiedThis(it) => it.set_node_id(node_id),
            Self::QualifiedSuper(it) => it.set_node_id(node_id),
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::FieldAccess(it) => it.set_node_id(node_id),
            Self::MethodCall(it) => it.set_node_id(node_id),
            Self::ClassLiteral(it) => it.set_node_id(node_id),
            Self::ClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::QualifiedClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::ArrayCreation(it) => it.set_node_id(node_id),
            Self::ArrayAccess(it) => it.set_node_id(node_id),
            Self::Update(it) => it.set_node_id(node_id),
            Self::Unary(it) => it.set_node_id(node_id),
            Self::Cast(it) => it.set_node_id(node_id),
            Self::Instanceof(it) => it.set_node_id(node_id),
            Self::Binary(it) => it.set_node_id(node_id),
            Self::Assignment(it) => it.set_node_id(node_id),
            Self::Conditional(it) => it.set_node_id(node_id),
            Self::Switch(it) => it.set_node_id(node_id),
            Self::Parenthesized(it) => it.set_node_id(node_id),
            Self::Lambda(it) => it.set_node_id(node_id),
            Self::MethodReference(it) => it.set_node_id(node_id),
        }
    }
}

impl ElementValue<'_> {
    /// Get [`NodeId`] of [`ElementValue`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Annotation(it) => it.node_id(),
            Self::Array(it) => it.node_id(),
            Self::IntLiteral(it) => it.node_id(),
            Self::FloatLiteral(it) => it.node_id(),
            Self::CharLiteral(it) => it.node_id(),
            Self::StringLiteral(it) => it.node_id(),
            Self::BooleanLiteral(it) => it.node_id(),
            Self::NullLiteral(it) => it.node_id(),
            Self::This(it) => it.node_id(),
            Self::Super(it) => it.node_id(),
            Self::QualifiedThis(it) => it.node_id(),
            Self::QualifiedSuper(it) => it.node_id(),
            Self::Identifier(it) => it.node_id(),
            Self::FieldAccess(it) => it.node_id(),
            Self::MethodCall(it) => it.node_id(),
            Self::ClassLiteral(it) => it.node_id(),
            Self::ClassInstanceCreation(it) => it.node_id(),
            Self::QualifiedClassInstanceCreation(it) => it.node_id(),
            Self::ArrayCreation(it) => it.node_id(),
            Self::ArrayAccess(it) => it.node_id(),
            Self::Update(it) => it.node_id(),
            Self::Unary(it) => it.node_id(),
            Self::Cast(it) => it.node_id(),
            Self::Instanceof(it) => it.node_id(),
            Self::Binary(it) => it.node_id(),
            Self::Assignment(it) => it.node_id(),
            Self::Conditional(it) => it.node_id(),
            Self::Switch(it) => it.node_id(),
            Self::Parenthesized(it) => it.node_id(),
            Self::Lambda(it) => it.node_id(),
            Self::MethodReference(it) => it.node_id(),
        }
    }

    /// Set [`NodeId`] of [`ElementValue`].
    // `#[inline(always)]` because this should boil down to a single instruction.
    #[inline(always)]
    pub fn set_node_id(&self, node_id: NodeId) {
        match self {
            Self::Annotation(it) => it.set_node_id(node_id),
            Self::Array(it) => it.set_node_id(node_id),
            Self::IntLiteral(it) => it.set_node_id(node_id),
            Self::FloatLiteral(it) => it.set_node_id(node_id),
            Self::CharLiteral(it) => it.set_node_id(node_id),
            Self::StringLiteral(it) => it.set_node_id(node_id),
            Self::BooleanLiteral(it) => it.set_node_id(node_id),
            Self::NullLiteral(it) => it.set_node_id(node_id),
            Self::This(it) => it.set_node_id(node_id),
            Self::Super(it) => it.set_node_id(node_id),
            Self::QualifiedThis(it) => it.set_node_id(node_id),
            Self::QualifiedSuper(it) => it.set_node_id(node_id),
            Self::Identifier(it) => it.set_node_id(node_id),
            Self::FieldAccess(it) => it.set_node_id(node_id),
            Self::MethodCall(it) => it.set_node_id(node_id),
            Self::ClassLiteral(it) => it.set_node_id(node_id),
            Self::ClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::QualifiedClassInstanceCreation(it) => it.set_node_id(node_id),
            Self::ArrayCreation(it) => it.set_node_id(node_id),
            Self::ArrayAccess(it) => it.set_node_id(node_id),
            Self::Update(it) => it.set_node_id(node_id),
            Self::Unary(it) => it.set_node_id(node_id),
            Self::Cast(it) => it.set_node_id(node_id),
            Self::Instanceof(it) => it.set_node_id(node_id),
            Self::Binary(it) => it.set_node_id(node_id),
            Self::Assignment(it) => it.set_node_id(node_id),
            Self::Conditional(it) => it.set_node_id(node_id),
            Self::Switch(it) => it.set_node_id(node_id),
            Self::Parenthesized(it) => it.set_node_id(node_id),
            Self::Lambda(it) => it.set_node_id(node_id),
            Self::MethodReference(it) => it.set_node_id(node_id),
        }
    }
}

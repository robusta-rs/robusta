// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/assert_layouts.rs`.

use std::mem::offset_of;

use oxc_index::nonmax::NonMaxU32;
use robusta_span::Span;
use robusta_syntax::{
    node::NodeId,
    operator::{AssignOp, BinaryOp, UnaryOp, UpdateOp},
    reference::ReferenceId,
    scope::ScopeId,
    symbol::SymbolId,
};

use crate::ast::*;

#[cfg(target_pointer_width = "64")]
const _: () = {
    // Padding: 0 bytes
    assert!(size_of::<CompilationUnit>() == 184);
    assert!(align_of::<CompilationUnit>() == 8);
    assert!(offset_of!(CompilationUnit, span) == 0);
    assert!(offset_of!(CompilationUnit, node_id) == 8);
    assert!(offset_of!(CompilationUnit, scope_id) == 12);
    assert!(offset_of!(CompilationUnit, source_text) == 16);
    assert!(offset_of!(CompilationUnit, comments) == 32);
    assert!(offset_of!(CompilationUnit, content) == 56);

    assert!(size_of::<CompilationUnitContent>() == 128);
    assert!(align_of::<CompilationUnitContent>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<OrdinaryCompilationUnit>() == 120);
    assert!(align_of::<OrdinaryCompilationUnit>() == 8);
    assert!(offset_of!(OrdinaryCompilationUnit, span) == 0);
    assert!(offset_of!(OrdinaryCompilationUnit, node_id) == 8);
    assert!(offset_of!(OrdinaryCompilationUnit, package) == 16);
    assert!(offset_of!(OrdinaryCompilationUnit, imports) == 72);
    assert!(offset_of!(OrdinaryCompilationUnit, types) == 96);

    // Padding: 4 bytes
    assert!(size_of::<ModularCompilationUnit>() == 120);
    assert!(align_of::<ModularCompilationUnit>() == 8);
    assert!(offset_of!(ModularCompilationUnit, span) == 0);
    assert!(offset_of!(ModularCompilationUnit, node_id) == 8);
    assert!(offset_of!(ModularCompilationUnit, imports) == 16);
    assert!(offset_of!(ModularCompilationUnit, module) == 40);

    // Padding: 4 bytes
    assert!(size_of::<PackageDeclaration>() == 56);
    assert!(align_of::<PackageDeclaration>() == 8);
    assert!(offset_of!(PackageDeclaration, span) == 0);
    assert!(offset_of!(PackageDeclaration, node_id) == 8);
    assert!(offset_of!(PackageDeclaration, annotations) == 16);
    assert!(offset_of!(PackageDeclaration, name) == 40);

    // Padding: 3 bytes
    assert!(size_of::<ImportDeclaration>() == 64);
    assert!(align_of::<ImportDeclaration>() == 8);
    assert!(offset_of!(ImportDeclaration, span) == 0);
    assert!(offset_of!(ImportDeclaration, node_id) == 8);
    assert!(offset_of!(ImportDeclaration, kind) == 12);
    assert!(offset_of!(ImportDeclaration, name) == 16);
    assert!(offset_of!(ImportDeclaration, local) == 32);

    assert!(size_of::<ImportKind>() == 1);
    assert!(align_of::<ImportKind>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<CompactCompilationUnit>() == 64);
    assert!(align_of::<CompactCompilationUnit>() == 8);
    assert!(offset_of!(CompactCompilationUnit, span) == 0);
    assert!(offset_of!(CompactCompilationUnit, node_id) == 8);
    assert!(offset_of!(CompactCompilationUnit, scope_id) == 12);
    assert!(offset_of!(CompactCompilationUnit, imports) == 16);
    assert!(offset_of!(CompactCompilationUnit, members) == 40);

    // Padding: 3 bytes
    assert!(size_of::<ModuleDeclaration>() == 80);
    assert!(align_of::<ModuleDeclaration>() == 8);
    assert!(offset_of!(ModuleDeclaration, span) == 0);
    assert!(offset_of!(ModuleDeclaration, node_id) == 8);
    assert!(offset_of!(ModuleDeclaration, is_open) == 12);
    assert!(offset_of!(ModuleDeclaration, annotations) == 16);
    assert!(offset_of!(ModuleDeclaration, name) == 40);
    assert!(offset_of!(ModuleDeclaration, directives) == 56);

    assert!(size_of::<ModuleDirective>() == 64);
    assert!(align_of::<ModuleDirective>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<RequiresDirective>() == 56);
    assert!(align_of::<RequiresDirective>() == 8);
    assert!(offset_of!(RequiresDirective, span) == 0);
    assert!(offset_of!(RequiresDirective, node_id) == 8);
    assert!(offset_of!(RequiresDirective, modifiers) == 16);
    assert!(offset_of!(RequiresDirective, name) == 40);

    // Padding: 3 bytes
    assert!(size_of::<PackageDirective>() == 56);
    assert!(align_of::<PackageDirective>() == 8);
    assert!(offset_of!(PackageDirective, span) == 0);
    assert!(offset_of!(PackageDirective, node_id) == 8);
    assert!(offset_of!(PackageDirective, kind) == 12);
    assert!(offset_of!(PackageDirective, name) == 16);
    assert!(offset_of!(PackageDirective, targets) == 32);

    assert!(size_of::<PackageDirectiveKind>() == 1);
    assert!(align_of::<PackageDirectiveKind>() == 1);

    // Padding: 4 bytes
    assert!(size_of::<UsesDirective>() == 32);
    assert!(align_of::<UsesDirective>() == 8);
    assert!(offset_of!(UsesDirective, span) == 0);
    assert!(offset_of!(UsesDirective, node_id) == 8);
    assert!(offset_of!(UsesDirective, name) == 16);

    // Padding: 4 bytes
    assert!(size_of::<ProvidesDirective>() == 56);
    assert!(align_of::<ProvidesDirective>() == 8);
    assert!(offset_of!(ProvidesDirective, span) == 0);
    assert!(offset_of!(ProvidesDirective, node_id) == 8);
    assert!(offset_of!(ProvidesDirective, name) == 16);
    assert!(offset_of!(ProvidesDirective, with) == 32);

    assert!(size_of::<TypeDeclaration>() == 16);
    assert!(align_of::<TypeDeclaration>() == 8);

    assert!(size_of::<LocalClassOrInterfaceDeclaration>() == 16);
    assert!(align_of::<LocalClassOrInterfaceDeclaration>() == 8);

    // Padding: 0 bytes
    assert!(size_of::<ClassDeclaration>() == 184);
    assert!(align_of::<ClassDeclaration>() == 8);
    assert!(offset_of!(ClassDeclaration, span) == 0);
    assert!(offset_of!(ClassDeclaration, node_id) == 8);
    assert!(offset_of!(ClassDeclaration, scope_id) == 12);
    assert!(offset_of!(ClassDeclaration, modifiers) == 16);
    assert!(offset_of!(ClassDeclaration, name) == 40);
    assert!(offset_of!(ClassDeclaration, type_params) == 72);
    assert!(offset_of!(ClassDeclaration, superclass) == 80);
    assert!(offset_of!(ClassDeclaration, interfaces) == 96);
    assert!(offset_of!(ClassDeclaration, permits) == 120);
    assert!(offset_of!(ClassDeclaration, body) == 144);

    // Padding: 0 bytes
    assert!(size_of::<ClassBody>() == 40);
    assert!(align_of::<ClassBody>() == 8);
    assert!(offset_of!(ClassBody, span) == 0);
    assert!(offset_of!(ClassBody, node_id) == 8);
    assert!(offset_of!(ClassBody, scope_id) == 12);
    assert!(offset_of!(ClassBody, members) == 16);

    // Padding: 0 bytes
    assert!(size_of::<InterfaceDeclaration>() == 168);
    assert!(align_of::<InterfaceDeclaration>() == 8);
    assert!(offset_of!(InterfaceDeclaration, span) == 0);
    assert!(offset_of!(InterfaceDeclaration, node_id) == 8);
    assert!(offset_of!(InterfaceDeclaration, scope_id) == 12);
    assert!(offset_of!(InterfaceDeclaration, modifiers) == 16);
    assert!(offset_of!(InterfaceDeclaration, name) == 40);
    assert!(offset_of!(InterfaceDeclaration, type_params) == 72);
    assert!(offset_of!(InterfaceDeclaration, extends) == 80);
    assert!(offset_of!(InterfaceDeclaration, permits) == 104);
    assert!(offset_of!(InterfaceDeclaration, body) == 128);

    // Padding: 0 bytes
    assert!(size_of::<InterfaceBody>() == 40);
    assert!(align_of::<InterfaceBody>() == 8);
    assert!(offset_of!(InterfaceBody, span) == 0);
    assert!(offset_of!(InterfaceBody, node_id) == 8);
    assert!(offset_of!(InterfaceBody, scope_id) == 12);
    assert!(offset_of!(InterfaceBody, members) == 16);

    // Padding: 4 bytes
    assert!(size_of::<EnumDeclaration>() == 168);
    assert!(align_of::<EnumDeclaration>() == 8);
    assert!(offset_of!(EnumDeclaration, span) == 0);
    assert!(offset_of!(EnumDeclaration, node_id) == 8);
    assert!(offset_of!(EnumDeclaration, modifiers) == 16);
    assert!(offset_of!(EnumDeclaration, name) == 40);
    assert!(offset_of!(EnumDeclaration, interfaces) == 72);
    assert!(offset_of!(EnumDeclaration, body) == 96);

    // Padding: 7 bytes
    assert!(size_of::<EnumBody>() == 72);
    assert!(align_of::<EnumBody>() == 8);
    assert!(offset_of!(EnumBody, span) == 0);
    assert!(offset_of!(EnumBody, node_id) == 8);
    assert!(offset_of!(EnumBody, scope_id) == 12);
    assert!(offset_of!(EnumBody, constants) == 16);
    assert!(offset_of!(EnumBody, members) == 40);
    assert!(offset_of!(EnumBody, has_body_semicolon) == 64);

    // Padding: 4 bytes
    assert!(size_of::<EnumConstant>() == 136);
    assert!(align_of::<EnumConstant>() == 8);
    assert!(offset_of!(EnumConstant, span) == 0);
    assert!(offset_of!(EnumConstant, node_id) == 8);
    assert!(offset_of!(EnumConstant, modifiers) == 16);
    assert!(offset_of!(EnumConstant, name) == 40);
    assert!(offset_of!(EnumConstant, args) == 72);
    assert!(offset_of!(EnumConstant, body) == 96);

    // Padding: 0 bytes
    assert!(size_of::<RecordDeclaration>() == 184);
    assert!(align_of::<RecordDeclaration>() == 8);
    assert!(offset_of!(RecordDeclaration, span) == 0);
    assert!(offset_of!(RecordDeclaration, node_id) == 8);
    assert!(offset_of!(RecordDeclaration, scope_id) == 12);
    assert!(offset_of!(RecordDeclaration, modifiers) == 16);
    assert!(offset_of!(RecordDeclaration, name) == 40);
    assert!(offset_of!(RecordDeclaration, type_params) == 72);
    assert!(offset_of!(RecordDeclaration, header) == 80);
    assert!(offset_of!(RecordDeclaration, interfaces) == 120);
    assert!(offset_of!(RecordDeclaration, body) == 144);

    // Padding: 4 bytes
    assert!(size_of::<RecordHeader>() == 40);
    assert!(align_of::<RecordHeader>() == 8);
    assert!(offset_of!(RecordHeader, span) == 0);
    assert!(offset_of!(RecordHeader, node_id) == 8);
    assert!(offset_of!(RecordHeader, components) == 16);

    // Padding: 3 bytes
    assert!(size_of::<RecordComponent>() == 112);
    assert!(align_of::<RecordComponent>() == 8);
    assert!(offset_of!(RecordComponent, span) == 0);
    assert!(offset_of!(RecordComponent, node_id) == 8);
    assert!(offset_of!(RecordComponent, varargs) == 12);
    assert!(offset_of!(RecordComponent, modifiers) == 16);
    assert!(offset_of!(RecordComponent, ty) == 40);
    assert!(offset_of!(RecordComponent, varargs_annotations) == 56);
    assert!(offset_of!(RecordComponent, name) == 80);

    // Padding: 4 bytes
    assert!(size_of::<AnnotationInterfaceDeclaration>() == 112);
    assert!(align_of::<AnnotationInterfaceDeclaration>() == 8);
    assert!(offset_of!(AnnotationInterfaceDeclaration, span) == 0);
    assert!(offset_of!(AnnotationInterfaceDeclaration, node_id) == 8);
    assert!(offset_of!(AnnotationInterfaceDeclaration, modifiers) == 16);
    assert!(offset_of!(AnnotationInterfaceDeclaration, name) == 40);
    assert!(offset_of!(AnnotationInterfaceDeclaration, body) == 72);

    // Padding: 0 bytes
    assert!(size_of::<AnnotationInterfaceBody>() == 40);
    assert!(align_of::<AnnotationInterfaceBody>() == 8);
    assert!(offset_of!(AnnotationInterfaceBody, span) == 0);
    assert!(offset_of!(AnnotationInterfaceBody, node_id) == 8);
    assert!(offset_of!(AnnotationInterfaceBody, scope_id) == 12);
    assert!(offset_of!(AnnotationInterfaceBody, members) == 16);

    assert!(size_of::<AnnotationInterfaceMemberDeclaration>() == 16);
    assert!(align_of::<AnnotationInterfaceMemberDeclaration>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<AnnotationInterfaceElementDeclaration>() == 128);
    assert!(align_of::<AnnotationInterfaceElementDeclaration>() == 8);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, span) == 0);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, node_id) == 8);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, modifiers) == 16);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, ty) == 40);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, name) == 56);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, post_paren_dimensions) == 88);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, default_value) == 112);

    assert!(size_of::<ClassMember>() == 16);
    assert!(align_of::<ClassMember>() == 8);

    assert!(size_of::<InterfaceMember>() == 16);
    assert!(align_of::<InterfaceMember>() == 8);

    // Padding: 3 bytes
    assert!(size_of::<InitializerBlock>() == 56);
    assert!(align_of::<InitializerBlock>() == 8);
    assert!(offset_of!(InitializerBlock, span) == 0);
    assert!(offset_of!(InitializerBlock, node_id) == 8);
    assert!(offset_of!(InitializerBlock, is_static) == 12);
    assert!(offset_of!(InitializerBlock, block) == 16);

    // Padding: 4 bytes
    assert!(size_of::<FieldDeclaration>() == 80);
    assert!(align_of::<FieldDeclaration>() == 8);
    assert!(offset_of!(FieldDeclaration, span) == 0);
    assert!(offset_of!(FieldDeclaration, node_id) == 8);
    assert!(offset_of!(FieldDeclaration, modifiers) == 16);
    assert!(offset_of!(FieldDeclaration, ty) == 40);
    assert!(offset_of!(FieldDeclaration, declarators) == 56);

    // Padding: 0 bytes
    assert!(size_of::<MethodDeclaration>() == 232);
    assert!(align_of::<MethodDeclaration>() == 8);
    assert!(offset_of!(MethodDeclaration, span) == 0);
    assert!(offset_of!(MethodDeclaration, node_id) == 8);
    assert!(offset_of!(MethodDeclaration, scope_id) == 12);
    assert!(offset_of!(MethodDeclaration, modifiers) == 16);
    assert!(offset_of!(MethodDeclaration, type_params) == 40);
    assert!(offset_of!(MethodDeclaration, return_type) == 48);
    assert!(offset_of!(MethodDeclaration, name) == 64);
    assert!(offset_of!(MethodDeclaration, params) == 96);
    assert!(offset_of!(MethodDeclaration, post_paren_dimensions) == 144);
    assert!(offset_of!(MethodDeclaration, throws) == 168);
    assert!(offset_of!(MethodDeclaration, body) == 192);

    // Padding: 0 bytes
    assert!(size_of::<ConstructorDeclaration>() == 192);
    assert!(align_of::<ConstructorDeclaration>() == 8);
    assert!(offset_of!(ConstructorDeclaration, span) == 0);
    assert!(offset_of!(ConstructorDeclaration, node_id) == 8);
    assert!(offset_of!(ConstructorDeclaration, scope_id) == 12);
    assert!(offset_of!(ConstructorDeclaration, modifiers) == 16);
    assert!(offset_of!(ConstructorDeclaration, type_params) == 40);
    assert!(offset_of!(ConstructorDeclaration, name) == 48);
    assert!(offset_of!(ConstructorDeclaration, params) == 80);
    assert!(offset_of!(ConstructorDeclaration, throws) == 128);
    assert!(offset_of!(ConstructorDeclaration, body) == 152);

    // Padding: 0 bytes
    assert!(size_of::<CompactConstructorDeclaration>() == 112);
    assert!(align_of::<CompactConstructorDeclaration>() == 8);
    assert!(offset_of!(CompactConstructorDeclaration, span) == 0);
    assert!(offset_of!(CompactConstructorDeclaration, node_id) == 8);
    assert!(offset_of!(CompactConstructorDeclaration, scope_id) == 12);
    assert!(offset_of!(CompactConstructorDeclaration, modifiers) == 16);
    assert!(offset_of!(CompactConstructorDeclaration, name) == 40);
    assert!(offset_of!(CompactConstructorDeclaration, body) == 72);

    // Padding: 4 bytes
    assert!(size_of::<ReceiverParameter>() == 88);
    assert!(align_of::<ReceiverParameter>() == 8);
    assert!(offset_of!(ReceiverParameter, span) == 0);
    assert!(offset_of!(ReceiverParameter, node_id) == 8);
    assert!(offset_of!(ReceiverParameter, modifiers) == 16);
    assert!(offset_of!(ReceiverParameter, ty) == 40);
    assert!(offset_of!(ReceiverParameter, qualifier) == 56);

    // Padding: 4 bytes
    assert!(size_of::<FormalParameters>() == 48);
    assert!(align_of::<FormalParameters>() == 8);
    assert!(offset_of!(FormalParameters, span) == 0);
    assert!(offset_of!(FormalParameters, node_id) == 8);
    assert!(offset_of!(FormalParameters, receiver) == 16);
    assert!(offset_of!(FormalParameters, items) == 24);

    // Padding: 3 bytes
    assert!(size_of::<FormalParameter>() == 136);
    assert!(align_of::<FormalParameter>() == 8);
    assert!(offset_of!(FormalParameter, span) == 0);
    assert!(offset_of!(FormalParameter, node_id) == 8);
    assert!(offset_of!(FormalParameter, varargs) == 12);
    assert!(offset_of!(FormalParameter, modifiers) == 16);
    assert!(offset_of!(FormalParameter, ty) == 40);
    assert!(offset_of!(FormalParameter, varargs_annotations) == 56);
    assert!(offset_of!(FormalParameter, name) == 80);
    assert!(offset_of!(FormalParameter, dims) == 112);

    // Padding: 0 bytes
    assert!(size_of::<Block>() == 40);
    assert!(align_of::<Block>() == 8);
    assert!(offset_of!(Block, span) == 0);
    assert!(offset_of!(Block, node_id) == 8);
    assert!(offset_of!(Block, scope_id) == 12);
    assert!(offset_of!(Block, statements) == 16);

    assert!(size_of::<Statement>() == 16);
    assert!(align_of::<Statement>() == 8);

    // Padding: 0 bytes
    assert!(size_of::<EmptyStatement>() == 12);
    assert!(align_of::<EmptyStatement>() == 4);
    assert!(offset_of!(EmptyStatement, span) == 0);
    assert!(offset_of!(EmptyStatement, node_id) == 8);

    // Padding: 4 bytes
    assert!(size_of::<IfStatement>() == 64);
    assert!(align_of::<IfStatement>() == 8);
    assert!(offset_of!(IfStatement, span) == 0);
    assert!(offset_of!(IfStatement, node_id) == 8);
    assert!(offset_of!(IfStatement, condition) == 16);
    assert!(offset_of!(IfStatement, then_branch) == 32);
    assert!(offset_of!(IfStatement, else_branch) == 48);

    // Padding: 0 bytes
    assert!(size_of::<SwitchStatement>() == 56);
    assert!(align_of::<SwitchStatement>() == 8);
    assert!(offset_of!(SwitchStatement, span) == 0);
    assert!(offset_of!(SwitchStatement, node_id) == 8);
    assert!(offset_of!(SwitchStatement, scope_id) == 12);
    assert!(offset_of!(SwitchStatement, selector) == 16);
    assert!(offset_of!(SwitchStatement, cases) == 32);

    assert!(size_of::<SwitchItem>() == 16);
    assert!(align_of::<SwitchItem>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<SwitchRule>() == 48);
    assert!(align_of::<SwitchRule>() == 8);
    assert!(offset_of!(SwitchRule, span) == 0);
    assert!(offset_of!(SwitchRule, node_id) == 8);
    assert!(offset_of!(SwitchRule, label) == 16);
    assert!(offset_of!(SwitchRule, body) == 32);

    assert!(size_of::<SwitchRuleBody>() == 16);
    assert!(align_of::<SwitchRuleBody>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<SwitchCase>() == 56);
    assert!(align_of::<SwitchCase>() == 8);
    assert!(offset_of!(SwitchCase, span) == 0);
    assert!(offset_of!(SwitchCase, node_id) == 8);
    assert!(offset_of!(SwitchCase, label) == 16);
    assert!(offset_of!(SwitchCase, statements) == 32);

    assert!(size_of::<SwitchLabel>() == 16);
    assert!(align_of::<SwitchLabel>() == 8);

    // Padding: 3 bytes
    assert!(size_of::<CaseLabel>() == 40);
    assert!(align_of::<CaseLabel>() == 8);
    assert!(offset_of!(CaseLabel, span) == 0);
    assert!(offset_of!(CaseLabel, node_id) == 8);
    assert!(offset_of!(CaseLabel, has_default) == 12);
    assert!(offset_of!(CaseLabel, constants) == 16);

    // Padding: 4 bytes
    assert!(size_of::<CasePatternLabel>() == 56);
    assert!(align_of::<CasePatternLabel>() == 8);
    assert!(offset_of!(CasePatternLabel, span) == 0);
    assert!(offset_of!(CasePatternLabel, node_id) == 8);
    assert!(offset_of!(CasePatternLabel, patterns) == 16);
    assert!(offset_of!(CasePatternLabel, guard) == 40);

    // Padding: 4 bytes
    assert!(size_of::<WhileStatement>() == 48);
    assert!(align_of::<WhileStatement>() == 8);
    assert!(offset_of!(WhileStatement, span) == 0);
    assert!(offset_of!(WhileStatement, node_id) == 8);
    assert!(offset_of!(WhileStatement, condition) == 16);
    assert!(offset_of!(WhileStatement, body) == 32);

    // Padding: 4 bytes
    assert!(size_of::<DoStatement>() == 48);
    assert!(align_of::<DoStatement>() == 8);
    assert!(offset_of!(DoStatement, span) == 0);
    assert!(offset_of!(DoStatement, node_id) == 8);
    assert!(offset_of!(DoStatement, body) == 16);
    assert!(offset_of!(DoStatement, condition) == 32);

    // Padding: 0 bytes
    assert!(size_of::<ForStatement>() == 104);
    assert!(align_of::<ForStatement>() == 8);
    assert!(offset_of!(ForStatement, span) == 0);
    assert!(offset_of!(ForStatement, node_id) == 8);
    assert!(offset_of!(ForStatement, scope_id) == 12);
    assert!(offset_of!(ForStatement, init) == 16);
    assert!(offset_of!(ForStatement, condition) == 32);
    assert!(offset_of!(ForStatement, update) == 48);
    assert!(offset_of!(ForStatement, body) == 88);

    assert!(size_of::<ForInit>() == 16);
    assert!(align_of::<ForInit>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<StatementExpressionList>() == 40);
    assert!(align_of::<StatementExpressionList>() == 8);
    assert!(offset_of!(StatementExpressionList, span) == 0);
    assert!(offset_of!(StatementExpressionList, node_id) == 8);
    assert!(offset_of!(StatementExpressionList, expressions) == 16);

    // Padding: 0 bytes
    assert!(size_of::<EnhancedForStatement>() == 144);
    assert!(align_of::<EnhancedForStatement>() == 8);
    assert!(offset_of!(EnhancedForStatement, span) == 0);
    assert!(offset_of!(EnhancedForStatement, node_id) == 8);
    assert!(offset_of!(EnhancedForStatement, scope_id) == 12);
    assert!(offset_of!(EnhancedForStatement, modifiers) == 16);
    assert!(offset_of!(EnhancedForStatement, ty) == 40);
    assert!(offset_of!(EnhancedForStatement, name) == 56);
    assert!(offset_of!(EnhancedForStatement, dims) == 88);
    assert!(offset_of!(EnhancedForStatement, iterable) == 112);
    assert!(offset_of!(EnhancedForStatement, body) == 128);

    // Padding: 4 bytes
    assert!(size_of::<BreakStatement>() == 48);
    assert!(align_of::<BreakStatement>() == 8);
    assert!(offset_of!(BreakStatement, span) == 0);
    assert!(offset_of!(BreakStatement, node_id) == 8);
    assert!(offset_of!(BreakStatement, label) == 16);

    // Padding: 4 bytes
    assert!(size_of::<ContinueStatement>() == 48);
    assert!(align_of::<ContinueStatement>() == 8);
    assert!(offset_of!(ContinueStatement, span) == 0);
    assert!(offset_of!(ContinueStatement, node_id) == 8);
    assert!(offset_of!(ContinueStatement, label) == 16);

    // Padding: 4 bytes
    assert!(size_of::<ThrowStatement>() == 32);
    assert!(align_of::<ThrowStatement>() == 8);
    assert!(offset_of!(ThrowStatement, span) == 0);
    assert!(offset_of!(ThrowStatement, node_id) == 8);
    assert!(offset_of!(ThrowStatement, value) == 16);

    // Padding: 4 bytes
    assert!(size_of::<AssertStatement>() == 48);
    assert!(align_of::<AssertStatement>() == 8);
    assert!(offset_of!(AssertStatement, span) == 0);
    assert!(offset_of!(AssertStatement, node_id) == 8);
    assert!(offset_of!(AssertStatement, condition) == 16);
    assert!(offset_of!(AssertStatement, message) == 32);

    // Padding: 7 bytes
    assert!(size_of::<TryStatement>() == 152);
    assert!(align_of::<TryStatement>() == 8);
    assert!(offset_of!(TryStatement, span) == 0);
    assert!(offset_of!(TryStatement, node_id) == 8);
    assert!(offset_of!(TryStatement, scope_id) == 12);
    assert!(offset_of!(TryStatement, resources) == 16);
    assert!(offset_of!(TryStatement, block) == 40);
    assert!(offset_of!(TryStatement, catches) == 80);
    assert!(offset_of!(TryStatement, finally_block) == 104);
    assert!(offset_of!(TryStatement, resources_trailing_semicolon) == 144);

    assert!(size_of::<Resource>() == 16);
    assert!(align_of::<Resource>() == 8);

    assert!(size_of::<ResourceAccess>() == 16);
    assert!(align_of::<ResourceAccess>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<ResourceDeclaration>() == 104);
    assert!(align_of::<ResourceDeclaration>() == 8);
    assert!(offset_of!(ResourceDeclaration, span) == 0);
    assert!(offset_of!(ResourceDeclaration, node_id) == 8);
    assert!(offset_of!(ResourceDeclaration, modifiers) == 16);
    assert!(offset_of!(ResourceDeclaration, ty) == 40);
    assert!(offset_of!(ResourceDeclaration, name) == 56);
    assert!(offset_of!(ResourceDeclaration, init) == 88);

    // Padding: 0 bytes
    assert!(size_of::<CatchClause>() == 136);
    assert!(align_of::<CatchClause>() == 8);
    assert!(offset_of!(CatchClause, span) == 0);
    assert!(offset_of!(CatchClause, node_id) == 8);
    assert!(offset_of!(CatchClause, scope_id) == 12);
    assert!(offset_of!(CatchClause, modifiers) == 16);
    assert!(offset_of!(CatchClause, types) == 40);
    assert!(offset_of!(CatchClause, name) == 64);
    assert!(offset_of!(CatchClause, block) == 96);

    // Padding: 4 bytes
    assert!(size_of::<SynchronizedStatement>() == 72);
    assert!(align_of::<SynchronizedStatement>() == 8);
    assert!(offset_of!(SynchronizedStatement, span) == 0);
    assert!(offset_of!(SynchronizedStatement, node_id) == 8);
    assert!(offset_of!(SynchronizedStatement, lock) == 16);
    assert!(offset_of!(SynchronizedStatement, body) == 32);

    // Padding: 4 bytes
    assert!(size_of::<LabeledStatement>() == 64);
    assert!(align_of::<LabeledStatement>() == 8);
    assert!(offset_of!(LabeledStatement, span) == 0);
    assert!(offset_of!(LabeledStatement, node_id) == 8);
    assert!(offset_of!(LabeledStatement, label) == 16);
    assert!(offset_of!(LabeledStatement, statement) == 48);

    // Padding: 3 bytes
    assert!(size_of::<ConstructorInvocation>() == 64);
    assert!(align_of::<ConstructorInvocation>() == 8);
    assert!(offset_of!(ConstructorInvocation, span) == 0);
    assert!(offset_of!(ConstructorInvocation, node_id) == 8);
    assert!(offset_of!(ConstructorInvocation, kind) == 12);
    assert!(offset_of!(ConstructorInvocation, qualifier) == 16);
    assert!(offset_of!(ConstructorInvocation, type_args) == 32);
    assert!(offset_of!(ConstructorInvocation, args) == 40);

    assert!(size_of::<ConstructorInvocationKind>() == 1);
    assert!(align_of::<ConstructorInvocationKind>() == 1);

    // Padding: 4 bytes
    assert!(size_of::<ExpressionStatement>() == 32);
    assert!(align_of::<ExpressionStatement>() == 8);
    assert!(offset_of!(ExpressionStatement, span) == 0);
    assert!(offset_of!(ExpressionStatement, node_id) == 8);
    assert!(offset_of!(ExpressionStatement, expression) == 16);

    // Padding: 4 bytes
    assert!(size_of::<LocalVariableDeclaration>() == 80);
    assert!(align_of::<LocalVariableDeclaration>() == 8);
    assert!(offset_of!(LocalVariableDeclaration, span) == 0);
    assert!(offset_of!(LocalVariableDeclaration, node_id) == 8);
    assert!(offset_of!(LocalVariableDeclaration, modifiers) == 16);
    assert!(offset_of!(LocalVariableDeclaration, ty) == 40);
    assert!(offset_of!(LocalVariableDeclaration, declarators) == 56);

    // Padding: 4 bytes
    assert!(size_of::<ReturnStatement>() == 32);
    assert!(align_of::<ReturnStatement>() == 8);
    assert!(offset_of!(ReturnStatement, span) == 0);
    assert!(offset_of!(ReturnStatement, node_id) == 8);
    assert!(offset_of!(ReturnStatement, value) == 16);

    // Padding: 4 bytes
    assert!(size_of::<YieldStatement>() == 32);
    assert!(align_of::<YieldStatement>() == 8);
    assert!(offset_of!(YieldStatement, span) == 0);
    assert!(offset_of!(YieldStatement, node_id) == 8);
    assert!(offset_of!(YieldStatement, value) == 16);

    // Padding: 4 bytes
    assert!(size_of::<VariableDeclarator>() == 88);
    assert!(align_of::<VariableDeclarator>() == 8);
    assert!(offset_of!(VariableDeclarator, span) == 0);
    assert!(offset_of!(VariableDeclarator, node_id) == 8);
    assert!(offset_of!(VariableDeclarator, name) == 16);
    assert!(offset_of!(VariableDeclarator, dims) == 48);
    assert!(offset_of!(VariableDeclarator, init) == 72);

    assert!(size_of::<VariableInitializer>() == 16);
    assert!(align_of::<VariableInitializer>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<ArrayInitializer>() == 40);
    assert!(align_of::<ArrayInitializer>() == 8);
    assert!(offset_of!(ArrayInitializer, span) == 0);
    assert!(offset_of!(ArrayInitializer, node_id) == 8);
    assert!(offset_of!(ArrayInitializer, elements) == 16);

    assert!(size_of::<Expression>() == 16);
    assert!(align_of::<Expression>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<FieldAccess>() == 64);
    assert!(align_of::<FieldAccess>() == 8);
    assert!(offset_of!(FieldAccess, span) == 0);
    assert!(offset_of!(FieldAccess, node_id) == 8);
    assert!(offset_of!(FieldAccess, target) == 16);
    assert!(offset_of!(FieldAccess, name) == 32);

    // Padding: 4 bytes
    assert!(size_of::<MethodCall>() == 56);
    assert!(align_of::<MethodCall>() == 8);
    assert!(offset_of!(MethodCall, span) == 0);
    assert!(offset_of!(MethodCall, node_id) == 8);
    assert!(offset_of!(MethodCall, callee) == 16);
    assert!(offset_of!(MethodCall, args) == 32);

    assert!(size_of::<Callee>() == 16);
    assert!(align_of::<Callee>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<MethodMember>() == 72);
    assert!(align_of::<MethodMember>() == 8);
    assert!(offset_of!(MethodMember, span) == 0);
    assert!(offset_of!(MethodMember, node_id) == 8);
    assert!(offset_of!(MethodMember, target) == 16);
    assert!(offset_of!(MethodMember, type_args) == 32);
    assert!(offset_of!(MethodMember, name) == 40);

    // Padding: 4 bytes
    assert!(size_of::<ClassLiteral>() == 32);
    assert!(align_of::<ClassLiteral>() == 8);
    assert!(offset_of!(ClassLiteral, span) == 0);
    assert!(offset_of!(ClassLiteral, node_id) == 8);
    assert!(offset_of!(ClassLiteral, ty) == 16);

    // Padding: 0 bytes
    assert!(size_of::<ThisExpression>() == 12);
    assert!(align_of::<ThisExpression>() == 4);
    assert!(offset_of!(ThisExpression, span) == 0);
    assert!(offset_of!(ThisExpression, node_id) == 8);

    // Padding: 0 bytes
    assert!(size_of::<Super>() == 12);
    assert!(align_of::<Super>() == 4);
    assert!(offset_of!(Super, span) == 0);
    assert!(offset_of!(Super, node_id) == 8);

    // Padding: 4 bytes
    assert!(size_of::<QualifiedThis>() == 32);
    assert!(align_of::<QualifiedThis>() == 8);
    assert!(offset_of!(QualifiedThis, span) == 0);
    assert!(offset_of!(QualifiedThis, node_id) == 8);
    assert!(offset_of!(QualifiedThis, qualifier) == 16);

    // Padding: 4 bytes
    assert!(size_of::<QualifiedSuper>() == 32);
    assert!(align_of::<QualifiedSuper>() == 8);
    assert!(offset_of!(QualifiedSuper, span) == 0);
    assert!(offset_of!(QualifiedSuper, node_id) == 8);
    assert!(offset_of!(QualifiedSuper, qualifier) == 16);

    // Padding: 4 bytes
    assert!(size_of::<ClassInstanceCreation>() == 104);
    assert!(align_of::<ClassInstanceCreation>() == 8);
    assert!(offset_of!(ClassInstanceCreation, span) == 0);
    assert!(offset_of!(ClassInstanceCreation, node_id) == 8);
    assert!(offset_of!(ClassInstanceCreation, constructor_type_args) == 16);
    assert!(offset_of!(ClassInstanceCreation, ty) == 24);
    assert!(offset_of!(ClassInstanceCreation, args) == 40);
    assert!(offset_of!(ClassInstanceCreation, body) == 64);

    // Padding: 4 bytes
    assert!(size_of::<QualifiedClassInstanceCreation>() == 168);
    assert!(align_of::<QualifiedClassInstanceCreation>() == 8);
    assert!(offset_of!(QualifiedClassInstanceCreation, span) == 0);
    assert!(offset_of!(QualifiedClassInstanceCreation, node_id) == 8);
    assert!(offset_of!(QualifiedClassInstanceCreation, qualifier) == 16);
    assert!(offset_of!(QualifiedClassInstanceCreation, constructor_type_args) == 32);
    assert!(offset_of!(QualifiedClassInstanceCreation, annotations) == 40);
    assert!(offset_of!(QualifiedClassInstanceCreation, name) == 64);
    assert!(offset_of!(QualifiedClassInstanceCreation, type_args) == 96);
    assert!(offset_of!(QualifiedClassInstanceCreation, args) == 104);
    assert!(offset_of!(QualifiedClassInstanceCreation, body) == 128);

    // Padding: 4 bytes
    assert!(size_of::<ArrayCreation>() == 96);
    assert!(align_of::<ArrayCreation>() == 8);
    assert!(offset_of!(ArrayCreation, span) == 0);
    assert!(offset_of!(ArrayCreation, node_id) == 8);
    assert!(offset_of!(ArrayCreation, element) == 16);
    assert!(offset_of!(ArrayCreation, dims) == 32);
    assert!(offset_of!(ArrayCreation, init) == 56);

    // Padding: 4 bytes
    assert!(size_of::<ArrayDimension>() == 56);
    assert!(align_of::<ArrayDimension>() == 8);
    assert!(offset_of!(ArrayDimension, span) == 0);
    assert!(offset_of!(ArrayDimension, node_id) == 8);
    assert!(offset_of!(ArrayDimension, annotations) == 16);
    assert!(offset_of!(ArrayDimension, length) == 40);

    // Padding: 4 bytes
    assert!(size_of::<ArrayAccess>() == 48);
    assert!(align_of::<ArrayAccess>() == 8);
    assert!(offset_of!(ArrayAccess, span) == 0);
    assert!(offset_of!(ArrayAccess, node_id) == 8);
    assert!(offset_of!(ArrayAccess, array) == 16);
    assert!(offset_of!(ArrayAccess, index) == 32);

    // Padding: 2 bytes
    assert!(size_of::<UpdateExpression>() == 32);
    assert!(align_of::<UpdateExpression>() == 8);
    assert!(offset_of!(UpdateExpression, span) == 0);
    assert!(offset_of!(UpdateExpression, node_id) == 8);
    assert!(offset_of!(UpdateExpression, op) == 12);
    assert!(offset_of!(UpdateExpression, prefix) == 13);
    assert!(offset_of!(UpdateExpression, target) == 16);

    // Padding: 3 bytes
    assert!(size_of::<UnaryExpression>() == 32);
    assert!(align_of::<UnaryExpression>() == 8);
    assert!(offset_of!(UnaryExpression, span) == 0);
    assert!(offset_of!(UnaryExpression, node_id) == 8);
    assert!(offset_of!(UnaryExpression, op) == 12);
    assert!(offset_of!(UnaryExpression, operand) == 16);

    // Padding: 4 bytes
    assert!(size_of::<CastExpression>() == 72);
    assert!(align_of::<CastExpression>() == 8);
    assert!(offset_of!(CastExpression, span) == 0);
    assert!(offset_of!(CastExpression, node_id) == 8);
    assert!(offset_of!(CastExpression, ty) == 16);
    assert!(offset_of!(CastExpression, additional_bounds) == 32);
    assert!(offset_of!(CastExpression, operand) == 56);

    // Padding: 4 bytes
    assert!(size_of::<InstanceofExpression>() == 48);
    assert!(align_of::<InstanceofExpression>() == 8);
    assert!(offset_of!(InstanceofExpression, span) == 0);
    assert!(offset_of!(InstanceofExpression, node_id) == 8);
    assert!(offset_of!(InstanceofExpression, operand) == 16);
    assert!(offset_of!(InstanceofExpression, rhs) == 32);

    assert!(size_of::<InstanceofRhs>() == 16);
    assert!(align_of::<InstanceofRhs>() == 8);

    assert!(size_of::<Pattern>() == 16);
    assert!(align_of::<Pattern>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<TypePattern>() == 88);
    assert!(align_of::<TypePattern>() == 8);
    assert!(offset_of!(TypePattern, span) == 0);
    assert!(offset_of!(TypePattern, node_id) == 8);
    assert!(offset_of!(TypePattern, modifiers) == 16);
    assert!(offset_of!(TypePattern, ty) == 40);
    assert!(offset_of!(TypePattern, name) == 56);

    // Padding: 4 bytes
    assert!(size_of::<RecordPattern>() == 56);
    assert!(align_of::<RecordPattern>() == 8);
    assert!(offset_of!(RecordPattern, span) == 0);
    assert!(offset_of!(RecordPattern, node_id) == 8);
    assert!(offset_of!(RecordPattern, ty) == 16);
    assert!(offset_of!(RecordPattern, components) == 32);

    assert!(size_of::<ComponentPattern>() == 16);
    assert!(align_of::<ComponentPattern>() == 8);

    // Padding: 0 bytes
    assert!(size_of::<MatchAllPattern>() == 12);
    assert!(align_of::<MatchAllPattern>() == 4);
    assert!(offset_of!(MatchAllPattern, span) == 0);
    assert!(offset_of!(MatchAllPattern, node_id) == 8);

    // Padding: 3 bytes
    assert!(size_of::<BinaryExpression>() == 48);
    assert!(align_of::<BinaryExpression>() == 8);
    assert!(offset_of!(BinaryExpression, span) == 0);
    assert!(offset_of!(BinaryExpression, node_id) == 8);
    assert!(offset_of!(BinaryExpression, op) == 12);
    assert!(offset_of!(BinaryExpression, lhs) == 16);
    assert!(offset_of!(BinaryExpression, rhs) == 32);

    // Padding: 3 bytes
    assert!(size_of::<AssignmentExpression>() == 48);
    assert!(align_of::<AssignmentExpression>() == 8);
    assert!(offset_of!(AssignmentExpression, span) == 0);
    assert!(offset_of!(AssignmentExpression, node_id) == 8);
    assert!(offset_of!(AssignmentExpression, op) == 12);
    assert!(offset_of!(AssignmentExpression, target) == 16);
    assert!(offset_of!(AssignmentExpression, value) == 32);

    assert!(size_of::<AssignmentTarget>() == 16);
    assert!(align_of::<AssignmentTarget>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<ConditionalExpression>() == 64);
    assert!(align_of::<ConditionalExpression>() == 8);
    assert!(offset_of!(ConditionalExpression, span) == 0);
    assert!(offset_of!(ConditionalExpression, node_id) == 8);
    assert!(offset_of!(ConditionalExpression, condition) == 16);
    assert!(offset_of!(ConditionalExpression, when_true) == 32);
    assert!(offset_of!(ConditionalExpression, when_false) == 48);

    // Padding: 0 bytes
    assert!(size_of::<SwitchExpression>() == 56);
    assert!(align_of::<SwitchExpression>() == 8);
    assert!(offset_of!(SwitchExpression, span) == 0);
    assert!(offset_of!(SwitchExpression, node_id) == 8);
    assert!(offset_of!(SwitchExpression, scope_id) == 12);
    assert!(offset_of!(SwitchExpression, selector) == 16);
    assert!(offset_of!(SwitchExpression, cases) == 32);

    // Padding: 4 bytes
    assert!(size_of::<ParenthesizedExpression>() == 32);
    assert!(align_of::<ParenthesizedExpression>() == 8);
    assert!(offset_of!(ParenthesizedExpression, span) == 0);
    assert!(offset_of!(ParenthesizedExpression, node_id) == 8);
    assert!(offset_of!(ParenthesizedExpression, expression) == 16);

    // Padding: 0 bytes
    assert!(size_of::<LambdaExpression>() == 80);
    assert!(align_of::<LambdaExpression>() == 8);
    assert!(offset_of!(LambdaExpression, span) == 0);
    assert!(offset_of!(LambdaExpression, node_id) == 8);
    assert!(offset_of!(LambdaExpression, scope_id) == 12);
    assert!(offset_of!(LambdaExpression, params) == 16);
    assert!(offset_of!(LambdaExpression, body) == 64);

    assert!(size_of::<LambdaBody>() == 16);
    assert!(align_of::<LambdaBody>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<MethodReference>() == 72);
    assert!(align_of::<MethodReference>() == 8);
    assert!(offset_of!(MethodReference, span) == 0);
    assert!(offset_of!(MethodReference, node_id) == 8);
    assert!(offset_of!(MethodReference, target) == 16);
    assert!(offset_of!(MethodReference, type_args) == 32);
    assert!(offset_of!(MethodReference, method) == 40);

    assert!(size_of::<MethodReferenceTarget>() == 16);
    assert!(align_of::<MethodReferenceTarget>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<TypeParameters>() == 40);
    assert!(align_of::<TypeParameters>() == 8);
    assert!(offset_of!(TypeParameters, span) == 0);
    assert!(offset_of!(TypeParameters, node_id) == 8);
    assert!(offset_of!(TypeParameters, params) == 16);

    // Padding: 4 bytes
    assert!(size_of::<TypeParameter>() == 96);
    assert!(align_of::<TypeParameter>() == 8);
    assert!(offset_of!(TypeParameter, span) == 0);
    assert!(offset_of!(TypeParameter, node_id) == 8);
    assert!(offset_of!(TypeParameter, annotations) == 16);
    assert!(offset_of!(TypeParameter, name) == 40);
    assert!(offset_of!(TypeParameter, bounds) == 72);

    assert!(size_of::<VariableType>() == 16);
    assert!(align_of::<VariableType>() == 8);

    // Padding: 0 bytes
    assert!(size_of::<VarType>() == 12);
    assert!(align_of::<VarType>() == 4);
    assert!(offset_of!(VarType, span) == 0);
    assert!(offset_of!(VarType, node_id) == 8);

    assert!(size_of::<Type>() == 16);
    assert!(align_of::<Type>() == 8);

    assert!(size_of::<MethodResult>() == 16);
    assert!(align_of::<MethodResult>() == 8);

    // Padding: 0 bytes
    assert!(size_of::<VoidType>() == 12);
    assert!(align_of::<VoidType>() == 4);
    assert!(offset_of!(VoidType, span) == 0);
    assert!(offset_of!(VoidType, node_id) == 8);

    // Padding: 3 bytes
    assert!(size_of::<PrimitiveType>() == 40);
    assert!(align_of::<PrimitiveType>() == 8);
    assert!(offset_of!(PrimitiveType, span) == 0);
    assert!(offset_of!(PrimitiveType, node_id) == 8);
    assert!(offset_of!(PrimitiveType, kind) == 12);
    assert!(offset_of!(PrimitiveType, annotations) == 16);

    assert!(size_of::<PrimitiveTypeKind>() == 1);
    assert!(align_of::<PrimitiveTypeKind>() == 1);

    assert!(size_of::<ClassType>() == 16);
    assert!(align_of::<ClassType>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<SimpleClassType>() == 80);
    assert!(align_of::<SimpleClassType>() == 8);
    assert!(offset_of!(SimpleClassType, span) == 0);
    assert!(offset_of!(SimpleClassType, node_id) == 8);
    assert!(offset_of!(SimpleClassType, annotations) == 16);
    assert!(offset_of!(SimpleClassType, name) == 40);
    assert!(offset_of!(SimpleClassType, args) == 72);

    // Padding: 4 bytes
    assert!(size_of::<QualifiedClassType>() == 96);
    assert!(align_of::<QualifiedClassType>() == 8);
    assert!(offset_of!(QualifiedClassType, span) == 0);
    assert!(offset_of!(QualifiedClassType, node_id) == 8);
    assert!(offset_of!(QualifiedClassType, qualifier) == 16);
    assert!(offset_of!(QualifiedClassType, annotations) == 32);
    assert!(offset_of!(QualifiedClassType, name) == 56);
    assert!(offset_of!(QualifiedClassType, args) == 88);

    // Padding: 4 bytes
    assert!(size_of::<ArrayType>() == 56);
    assert!(align_of::<ArrayType>() == 8);
    assert!(offset_of!(ArrayType, span) == 0);
    assert!(offset_of!(ArrayType, node_id) == 8);
    assert!(offset_of!(ArrayType, element) == 16);
    assert!(offset_of!(ArrayType, annotations) == 32);

    // Padding: 4 bytes
    assert!(size_of::<ArrayTypeDimension>() == 40);
    assert!(align_of::<ArrayTypeDimension>() == 8);
    assert!(offset_of!(ArrayTypeDimension, span) == 0);
    assert!(offset_of!(ArrayTypeDimension, node_id) == 8);
    assert!(offset_of!(ArrayTypeDimension, annotations) == 16);

    // Padding: 4 bytes
    assert!(size_of::<TypeArguments>() == 40);
    assert!(align_of::<TypeArguments>() == 8);
    assert!(offset_of!(TypeArguments, span) == 0);
    assert!(offset_of!(TypeArguments, node_id) == 8);
    assert!(offset_of!(TypeArguments, args) == 16);

    assert!(size_of::<TypeArgument>() == 16);
    assert!(align_of::<TypeArgument>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<WildcardTypeArgument>() == 72);
    assert!(align_of::<WildcardTypeArgument>() == 8);
    assert!(offset_of!(WildcardTypeArgument, span) == 0);
    assert!(offset_of!(WildcardTypeArgument, node_id) == 8);
    assert!(offset_of!(WildcardTypeArgument, annotations) == 16);
    assert!(offset_of!(WildcardTypeArgument, bound) == 40);

    // Padding: 3 bytes
    assert!(size_of::<WildcardBound>() == 32);
    assert!(align_of::<WildcardBound>() == 8);
    assert!(offset_of!(WildcardBound, span) == 0);
    assert!(offset_of!(WildcardBound, node_id) == 8);
    assert!(offset_of!(WildcardBound, kind) == 12);
    assert!(offset_of!(WildcardBound, ty) == 16);

    assert!(size_of::<WildcardBoundKind>() == 1);
    assert!(align_of::<WildcardBoundKind>() == 1);

    // Padding: 4 bytes
    assert!(size_of::<IntLiteral>() == 32);
    assert!(align_of::<IntLiteral>() == 8);
    assert!(offset_of!(IntLiteral, span) == 0);
    assert!(offset_of!(IntLiteral, node_id) == 8);
    assert!(offset_of!(IntLiteral, raw) == 16);

    // Padding: 4 bytes
    assert!(size_of::<FloatLiteral>() == 32);
    assert!(align_of::<FloatLiteral>() == 8);
    assert!(offset_of!(FloatLiteral, span) == 0);
    assert!(offset_of!(FloatLiteral, node_id) == 8);
    assert!(offset_of!(FloatLiteral, raw) == 16);

    // Padding: 4 bytes
    assert!(size_of::<CharLiteral>() == 32);
    assert!(align_of::<CharLiteral>() == 8);
    assert!(offset_of!(CharLiteral, span) == 0);
    assert!(offset_of!(CharLiteral, node_id) == 8);
    assert!(offset_of!(CharLiteral, raw) == 16);

    // Padding: 4 bytes
    assert!(size_of::<StringLiteral>() == 32);
    assert!(align_of::<StringLiteral>() == 8);
    assert!(offset_of!(StringLiteral, span) == 0);
    assert!(offset_of!(StringLiteral, node_id) == 8);
    assert!(offset_of!(StringLiteral, raw) == 16);

    // Padding: 3 bytes
    assert!(size_of::<BooleanLiteral>() == 16);
    assert!(align_of::<BooleanLiteral>() == 4);
    assert!(offset_of!(BooleanLiteral, span) == 0);
    assert!(offset_of!(BooleanLiteral, node_id) == 8);
    assert!(offset_of!(BooleanLiteral, value) == 12);

    // Padding: 0 bytes
    assert!(size_of::<NullLiteral>() == 12);
    assert!(align_of::<NullLiteral>() == 4);
    assert!(offset_of!(NullLiteral, span) == 0);
    assert!(offset_of!(NullLiteral, node_id) == 8);

    // Padding: 4 bytes
    assert!(size_of::<IdentifierName>() == 32);
    assert!(align_of::<IdentifierName>() == 8);
    assert!(offset_of!(IdentifierName, span) == 0);
    assert!(offset_of!(IdentifierName, node_id) == 8);
    assert!(offset_of!(IdentifierName, name) == 16);

    // Padding: 0 bytes
    assert!(size_of::<IdentifierReference>() == 32);
    assert!(align_of::<IdentifierReference>() == 8);
    assert!(offset_of!(IdentifierReference, span) == 0);
    assert!(offset_of!(IdentifierReference, node_id) == 8);
    assert!(offset_of!(IdentifierReference, reference_id) == 12);
    assert!(offset_of!(IdentifierReference, name) == 16);

    // Padding: 0 bytes
    assert!(size_of::<BindingIdentifier>() == 32);
    assert!(align_of::<BindingIdentifier>() == 8);
    assert!(offset_of!(BindingIdentifier, span) == 0);
    assert!(offset_of!(BindingIdentifier, node_id) == 8);
    assert!(offset_of!(BindingIdentifier, symbol_id) == 12);
    assert!(offset_of!(BindingIdentifier, name) == 16);

    // Padding: 4 bytes
    assert!(size_of::<LabelIdentifier>() == 32);
    assert!(align_of::<LabelIdentifier>() == 8);
    assert!(offset_of!(LabelIdentifier, span) == 0);
    assert!(offset_of!(LabelIdentifier, node_id) == 8);
    assert!(offset_of!(LabelIdentifier, name) == 16);

    assert!(size_of::<Name>() == 16);
    assert!(align_of::<Name>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<QualifiedName>() == 64);
    assert!(align_of::<QualifiedName>() == 8);
    assert!(offset_of!(QualifiedName, span) == 0);
    assert!(offset_of!(QualifiedName, node_id) == 8);
    assert!(offset_of!(QualifiedName, qualifier) == 16);
    assert!(offset_of!(QualifiedName, name) == 32);

    assert!(size_of::<TypeName>() == 16);
    assert!(align_of::<TypeName>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<QualifiedTypeName>() == 64);
    assert!(align_of::<QualifiedTypeName>() == 8);
    assert!(offset_of!(QualifiedTypeName, span) == 0);
    assert!(offset_of!(QualifiedTypeName, node_id) == 8);
    assert!(offset_of!(QualifiedTypeName, qualifier) == 16);
    assert!(offset_of!(QualifiedTypeName, name) == 32);

    assert!(size_of::<Modifier>() == 24);
    assert!(align_of::<Modifier>() == 8);

    // Padding: 3 bytes
    assert!(size_of::<KeywordModifier>() == 12);
    assert!(align_of::<KeywordModifier>() == 4);
    assert!(offset_of!(KeywordModifier, span) == 0);
    assert!(offset_of!(KeywordModifier, kind) == 8);

    // Padding: 4 bytes
    assert!(size_of::<Annotation>() == 56);
    assert!(align_of::<Annotation>() == 8);
    assert!(offset_of!(Annotation, span) == 0);
    assert!(offset_of!(Annotation, node_id) == 8);
    assert!(offset_of!(Annotation, name) == 16);
    assert!(offset_of!(Annotation, args) == 32);

    assert!(size_of::<AnnotationArgument>() == 16);
    assert!(align_of::<AnnotationArgument>() == 8);

    // Padding: 4 bytes
    assert!(size_of::<ElementValuePair>() == 64);
    assert!(align_of::<ElementValuePair>() == 8);
    assert!(offset_of!(ElementValuePair, span) == 0);
    assert!(offset_of!(ElementValuePair, node_id) == 8);
    assert!(offset_of!(ElementValuePair, name) == 16);
    assert!(offset_of!(ElementValuePair, value) == 48);

    assert!(size_of::<ElementValue>() == 16);
    assert!(align_of::<ElementValue>() == 8);

    // Padding: 3 bytes
    assert!(size_of::<ElementValueArrayInitializer>() == 40);
    assert!(align_of::<ElementValueArrayInitializer>() == 8);
    assert!(offset_of!(ElementValueArrayInitializer, span) == 0);
    assert!(offset_of!(ElementValueArrayInitializer, node_id) == 8);
    assert!(offset_of!(ElementValueArrayInitializer, trailing_comma) == 12);
    assert!(offset_of!(ElementValueArrayInitializer, values) == 16);

    assert!(size_of::<ModifierKind>() == 1);
    assert!(align_of::<ModifierKind>() == 1);

    assert!(size_of::<CommentKind>() == 1);
    assert!(align_of::<CommentKind>() == 1);

    assert!(size_of::<CommentPosition>() == 1);
    assert!(align_of::<CommentPosition>() == 1);

    assert!(size_of::<CommentContent>() == 1);
    assert!(align_of::<CommentContent>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<CommentNewlines>() == 1);
    assert!(align_of::<CommentNewlines>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<Comment>() == 16);
    assert!(align_of::<Comment>() == 4);
    assert!(offset_of!(Comment, span) == 0);
    assert!(offset_of!(Comment, attached_to) == 8);
    assert!(offset_of!(Comment, kind) == 12);
    assert!(offset_of!(Comment, position) == 13);
    assert!(offset_of!(Comment, newlines) == 14);
    assert!(offset_of!(Comment, content) == 15);

    // Padding: 0 bytes
    assert!(size_of::<Span>() == 8);
    assert!(align_of::<Span>() == 4);
    assert!(offset_of!(Span, start) == 0);
    assert!(offset_of!(Span, end) == 4);

    // Padding: 0 bytes
    assert!(size_of::<NonMaxU32>() == 4);
    assert!(align_of::<NonMaxU32>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<NodeId>() == 4);
    assert!(align_of::<NodeId>() == 4);

    assert!(size_of::<UnaryOp>() == 1);
    assert!(align_of::<UnaryOp>() == 1);

    assert!(size_of::<UpdateOp>() == 1);
    assert!(align_of::<UpdateOp>() == 1);

    assert!(size_of::<BinaryOp>() == 1);
    assert!(align_of::<BinaryOp>() == 1);

    assert!(size_of::<AssignOp>() == 1);
    assert!(align_of::<AssignOp>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<ReferenceId>() == 4);
    assert!(align_of::<ReferenceId>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<ScopeId>() == 4);
    assert!(align_of::<ScopeId>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<SymbolId>() == 4);
    assert!(align_of::<SymbolId>() == 4);
};

#[cfg(target_pointer_width = "32")]
const _: () = {
    // Padding: 0 bytes
    assert!(size_of::<CompilationUnit>() == 128);
    assert!(align_of::<CompilationUnit>() == 4);
    assert!(offset_of!(CompilationUnit, span) == 0);
    assert!(offset_of!(CompilationUnit, node_id) == 8);
    assert!(offset_of!(CompilationUnit, scope_id) == 12);
    assert!(offset_of!(CompilationUnit, source_text) == 16);
    assert!(offset_of!(CompilationUnit, comments) == 24);
    assert!(offset_of!(CompilationUnit, content) == 40);

    assert!(size_of::<CompilationUnitContent>() == 88);
    assert!(align_of::<CompilationUnitContent>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<OrdinaryCompilationUnit>() == 80);
    assert!(align_of::<OrdinaryCompilationUnit>() == 4);
    assert!(offset_of!(OrdinaryCompilationUnit, span) == 0);
    assert!(offset_of!(OrdinaryCompilationUnit, node_id) == 8);
    assert!(offset_of!(OrdinaryCompilationUnit, package) == 12);
    assert!(offset_of!(OrdinaryCompilationUnit, imports) == 48);
    assert!(offset_of!(OrdinaryCompilationUnit, types) == 64);

    // Padding: 0 bytes
    assert!(size_of::<ModularCompilationUnit>() == 84);
    assert!(align_of::<ModularCompilationUnit>() == 4);
    assert!(offset_of!(ModularCompilationUnit, span) == 0);
    assert!(offset_of!(ModularCompilationUnit, node_id) == 8);
    assert!(offset_of!(ModularCompilationUnit, imports) == 12);
    assert!(offset_of!(ModularCompilationUnit, module) == 28);

    // Padding: 0 bytes
    assert!(size_of::<PackageDeclaration>() == 36);
    assert!(align_of::<PackageDeclaration>() == 4);
    assert!(offset_of!(PackageDeclaration, span) == 0);
    assert!(offset_of!(PackageDeclaration, node_id) == 8);
    assert!(offset_of!(PackageDeclaration, annotations) == 12);
    assert!(offset_of!(PackageDeclaration, name) == 28);

    // Padding: 3 bytes
    assert!(size_of::<ImportDeclaration>() == 48);
    assert!(align_of::<ImportDeclaration>() == 4);
    assert!(offset_of!(ImportDeclaration, span) == 0);
    assert!(offset_of!(ImportDeclaration, node_id) == 8);
    assert!(offset_of!(ImportDeclaration, kind) == 12);
    assert!(offset_of!(ImportDeclaration, name) == 16);
    assert!(offset_of!(ImportDeclaration, local) == 24);

    assert!(size_of::<ImportKind>() == 1);
    assert!(align_of::<ImportKind>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<CompactCompilationUnit>() == 48);
    assert!(align_of::<CompactCompilationUnit>() == 4);
    assert!(offset_of!(CompactCompilationUnit, span) == 0);
    assert!(offset_of!(CompactCompilationUnit, node_id) == 8);
    assert!(offset_of!(CompactCompilationUnit, scope_id) == 12);
    assert!(offset_of!(CompactCompilationUnit, imports) == 16);
    assert!(offset_of!(CompactCompilationUnit, members) == 32);

    // Padding: 3 bytes
    assert!(size_of::<ModuleDeclaration>() == 56);
    assert!(align_of::<ModuleDeclaration>() == 4);
    assert!(offset_of!(ModuleDeclaration, span) == 0);
    assert!(offset_of!(ModuleDeclaration, node_id) == 8);
    assert!(offset_of!(ModuleDeclaration, is_open) == 12);
    assert!(offset_of!(ModuleDeclaration, annotations) == 16);
    assert!(offset_of!(ModuleDeclaration, name) == 32);
    assert!(offset_of!(ModuleDeclaration, directives) == 40);

    assert!(size_of::<ModuleDirective>() == 44);
    assert!(align_of::<ModuleDirective>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<RequiresDirective>() == 36);
    assert!(align_of::<RequiresDirective>() == 4);
    assert!(offset_of!(RequiresDirective, span) == 0);
    assert!(offset_of!(RequiresDirective, node_id) == 8);
    assert!(offset_of!(RequiresDirective, modifiers) == 12);
    assert!(offset_of!(RequiresDirective, name) == 28);

    // Padding: 3 bytes
    assert!(size_of::<PackageDirective>() == 40);
    assert!(align_of::<PackageDirective>() == 4);
    assert!(offset_of!(PackageDirective, span) == 0);
    assert!(offset_of!(PackageDirective, node_id) == 8);
    assert!(offset_of!(PackageDirective, kind) == 12);
    assert!(offset_of!(PackageDirective, name) == 16);
    assert!(offset_of!(PackageDirective, targets) == 24);

    assert!(size_of::<PackageDirectiveKind>() == 1);
    assert!(align_of::<PackageDirectiveKind>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<UsesDirective>() == 20);
    assert!(align_of::<UsesDirective>() == 4);
    assert!(offset_of!(UsesDirective, span) == 0);
    assert!(offset_of!(UsesDirective, node_id) == 8);
    assert!(offset_of!(UsesDirective, name) == 12);

    // Padding: 0 bytes
    assert!(size_of::<ProvidesDirective>() == 36);
    assert!(align_of::<ProvidesDirective>() == 4);
    assert!(offset_of!(ProvidesDirective, span) == 0);
    assert!(offset_of!(ProvidesDirective, node_id) == 8);
    assert!(offset_of!(ProvidesDirective, name) == 12);
    assert!(offset_of!(ProvidesDirective, with) == 20);

    assert!(size_of::<TypeDeclaration>() == 8);
    assert!(align_of::<TypeDeclaration>() == 4);

    assert!(size_of::<LocalClassOrInterfaceDeclaration>() == 8);
    assert!(align_of::<LocalClassOrInterfaceDeclaration>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<ClassDeclaration>() == 132);
    assert!(align_of::<ClassDeclaration>() == 4);
    assert!(offset_of!(ClassDeclaration, span) == 0);
    assert!(offset_of!(ClassDeclaration, node_id) == 8);
    assert!(offset_of!(ClassDeclaration, scope_id) == 12);
    assert!(offset_of!(ClassDeclaration, modifiers) == 16);
    assert!(offset_of!(ClassDeclaration, name) == 32);
    assert!(offset_of!(ClassDeclaration, type_params) == 56);
    assert!(offset_of!(ClassDeclaration, superclass) == 60);
    assert!(offset_of!(ClassDeclaration, interfaces) == 68);
    assert!(offset_of!(ClassDeclaration, permits) == 84);
    assert!(offset_of!(ClassDeclaration, body) == 100);

    // Padding: 0 bytes
    assert!(size_of::<ClassBody>() == 32);
    assert!(align_of::<ClassBody>() == 4);
    assert!(offset_of!(ClassBody, span) == 0);
    assert!(offset_of!(ClassBody, node_id) == 8);
    assert!(offset_of!(ClassBody, scope_id) == 12);
    assert!(offset_of!(ClassBody, members) == 16);

    // Padding: 0 bytes
    assert!(size_of::<InterfaceDeclaration>() == 124);
    assert!(align_of::<InterfaceDeclaration>() == 4);
    assert!(offset_of!(InterfaceDeclaration, span) == 0);
    assert!(offset_of!(InterfaceDeclaration, node_id) == 8);
    assert!(offset_of!(InterfaceDeclaration, scope_id) == 12);
    assert!(offset_of!(InterfaceDeclaration, modifiers) == 16);
    assert!(offset_of!(InterfaceDeclaration, name) == 32);
    assert!(offset_of!(InterfaceDeclaration, type_params) == 56);
    assert!(offset_of!(InterfaceDeclaration, extends) == 60);
    assert!(offset_of!(InterfaceDeclaration, permits) == 76);
    assert!(offset_of!(InterfaceDeclaration, body) == 92);

    // Padding: 0 bytes
    assert!(size_of::<InterfaceBody>() == 32);
    assert!(align_of::<InterfaceBody>() == 4);
    assert!(offset_of!(InterfaceBody, span) == 0);
    assert!(offset_of!(InterfaceBody, node_id) == 8);
    assert!(offset_of!(InterfaceBody, scope_id) == 12);
    assert!(offset_of!(InterfaceBody, members) == 16);

    // Padding: 0 bytes
    assert!(size_of::<EnumDeclaration>() == 120);
    assert!(align_of::<EnumDeclaration>() == 4);
    assert!(offset_of!(EnumDeclaration, span) == 0);
    assert!(offset_of!(EnumDeclaration, node_id) == 8);
    assert!(offset_of!(EnumDeclaration, modifiers) == 12);
    assert!(offset_of!(EnumDeclaration, name) == 28);
    assert!(offset_of!(EnumDeclaration, interfaces) == 52);
    assert!(offset_of!(EnumDeclaration, body) == 68);

    // Padding: 3 bytes
    assert!(size_of::<EnumBody>() == 52);
    assert!(align_of::<EnumBody>() == 4);
    assert!(offset_of!(EnumBody, span) == 0);
    assert!(offset_of!(EnumBody, node_id) == 8);
    assert!(offset_of!(EnumBody, scope_id) == 12);
    assert!(offset_of!(EnumBody, constants) == 16);
    assert!(offset_of!(EnumBody, members) == 32);
    assert!(offset_of!(EnumBody, has_body_semicolon) == 48);

    // Padding: 0 bytes
    assert!(size_of::<EnumConstant>() == 100);
    assert!(align_of::<EnumConstant>() == 4);
    assert!(offset_of!(EnumConstant, span) == 0);
    assert!(offset_of!(EnumConstant, node_id) == 8);
    assert!(offset_of!(EnumConstant, modifiers) == 12);
    assert!(offset_of!(EnumConstant, name) == 28);
    assert!(offset_of!(EnumConstant, args) == 52);
    assert!(offset_of!(EnumConstant, body) == 68);

    // Padding: 0 bytes
    assert!(size_of::<RecordDeclaration>() == 136);
    assert!(align_of::<RecordDeclaration>() == 4);
    assert!(offset_of!(RecordDeclaration, span) == 0);
    assert!(offset_of!(RecordDeclaration, node_id) == 8);
    assert!(offset_of!(RecordDeclaration, scope_id) == 12);
    assert!(offset_of!(RecordDeclaration, modifiers) == 16);
    assert!(offset_of!(RecordDeclaration, name) == 32);
    assert!(offset_of!(RecordDeclaration, type_params) == 56);
    assert!(offset_of!(RecordDeclaration, header) == 60);
    assert!(offset_of!(RecordDeclaration, interfaces) == 88);
    assert!(offset_of!(RecordDeclaration, body) == 104);

    // Padding: 0 bytes
    assert!(size_of::<RecordHeader>() == 28);
    assert!(align_of::<RecordHeader>() == 4);
    assert!(offset_of!(RecordHeader, span) == 0);
    assert!(offset_of!(RecordHeader, node_id) == 8);
    assert!(offset_of!(RecordHeader, components) == 12);

    // Padding: 3 bytes
    assert!(size_of::<RecordComponent>() == 80);
    assert!(align_of::<RecordComponent>() == 4);
    assert!(offset_of!(RecordComponent, span) == 0);
    assert!(offset_of!(RecordComponent, node_id) == 8);
    assert!(offset_of!(RecordComponent, varargs) == 12);
    assert!(offset_of!(RecordComponent, modifiers) == 16);
    assert!(offset_of!(RecordComponent, ty) == 32);
    assert!(offset_of!(RecordComponent, varargs_annotations) == 40);
    assert!(offset_of!(RecordComponent, name) == 56);

    // Padding: 0 bytes
    assert!(size_of::<AnnotationInterfaceDeclaration>() == 84);
    assert!(align_of::<AnnotationInterfaceDeclaration>() == 4);
    assert!(offset_of!(AnnotationInterfaceDeclaration, span) == 0);
    assert!(offset_of!(AnnotationInterfaceDeclaration, node_id) == 8);
    assert!(offset_of!(AnnotationInterfaceDeclaration, modifiers) == 12);
    assert!(offset_of!(AnnotationInterfaceDeclaration, name) == 28);
    assert!(offset_of!(AnnotationInterfaceDeclaration, body) == 52);

    // Padding: 0 bytes
    assert!(size_of::<AnnotationInterfaceBody>() == 32);
    assert!(align_of::<AnnotationInterfaceBody>() == 4);
    assert!(offset_of!(AnnotationInterfaceBody, span) == 0);
    assert!(offset_of!(AnnotationInterfaceBody, node_id) == 8);
    assert!(offset_of!(AnnotationInterfaceBody, scope_id) == 12);
    assert!(offset_of!(AnnotationInterfaceBody, members) == 16);

    assert!(size_of::<AnnotationInterfaceMemberDeclaration>() == 8);
    assert!(align_of::<AnnotationInterfaceMemberDeclaration>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<AnnotationInterfaceElementDeclaration>() == 84);
    assert!(align_of::<AnnotationInterfaceElementDeclaration>() == 4);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, span) == 0);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, node_id) == 8);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, modifiers) == 12);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, ty) == 28);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, name) == 36);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, post_paren_dimensions) == 60);
    assert!(offset_of!(AnnotationInterfaceElementDeclaration, default_value) == 76);

    assert!(size_of::<ClassMember>() == 8);
    assert!(align_of::<ClassMember>() == 4);

    assert!(size_of::<InterfaceMember>() == 8);
    assert!(align_of::<InterfaceMember>() == 4);

    // Padding: 3 bytes
    assert!(size_of::<InitializerBlock>() == 48);
    assert!(align_of::<InitializerBlock>() == 4);
    assert!(offset_of!(InitializerBlock, span) == 0);
    assert!(offset_of!(InitializerBlock, node_id) == 8);
    assert!(offset_of!(InitializerBlock, is_static) == 12);
    assert!(offset_of!(InitializerBlock, block) == 16);

    // Padding: 0 bytes
    assert!(size_of::<FieldDeclaration>() == 52);
    assert!(align_of::<FieldDeclaration>() == 4);
    assert!(offset_of!(FieldDeclaration, span) == 0);
    assert!(offset_of!(FieldDeclaration, node_id) == 8);
    assert!(offset_of!(FieldDeclaration, modifiers) == 12);
    assert!(offset_of!(FieldDeclaration, ty) == 28);
    assert!(offset_of!(FieldDeclaration, declarators) == 36);

    // Padding: 0 bytes
    assert!(size_of::<MethodDeclaration>() == 164);
    assert!(align_of::<MethodDeclaration>() == 4);
    assert!(offset_of!(MethodDeclaration, span) == 0);
    assert!(offset_of!(MethodDeclaration, node_id) == 8);
    assert!(offset_of!(MethodDeclaration, scope_id) == 12);
    assert!(offset_of!(MethodDeclaration, modifiers) == 16);
    assert!(offset_of!(MethodDeclaration, type_params) == 32);
    assert!(offset_of!(MethodDeclaration, return_type) == 36);
    assert!(offset_of!(MethodDeclaration, name) == 44);
    assert!(offset_of!(MethodDeclaration, params) == 68);
    assert!(offset_of!(MethodDeclaration, post_paren_dimensions) == 100);
    assert!(offset_of!(MethodDeclaration, throws) == 116);
    assert!(offset_of!(MethodDeclaration, body) == 132);

    // Padding: 0 bytes
    assert!(size_of::<ConstructorDeclaration>() == 136);
    assert!(align_of::<ConstructorDeclaration>() == 4);
    assert!(offset_of!(ConstructorDeclaration, span) == 0);
    assert!(offset_of!(ConstructorDeclaration, node_id) == 8);
    assert!(offset_of!(ConstructorDeclaration, scope_id) == 12);
    assert!(offset_of!(ConstructorDeclaration, modifiers) == 16);
    assert!(offset_of!(ConstructorDeclaration, type_params) == 32);
    assert!(offset_of!(ConstructorDeclaration, name) == 36);
    assert!(offset_of!(ConstructorDeclaration, params) == 56);
    assert!(offset_of!(ConstructorDeclaration, throws) == 88);
    assert!(offset_of!(ConstructorDeclaration, body) == 104);

    // Padding: 0 bytes
    assert!(size_of::<CompactConstructorDeclaration>() == 84);
    assert!(align_of::<CompactConstructorDeclaration>() == 4);
    assert!(offset_of!(CompactConstructorDeclaration, span) == 0);
    assert!(offset_of!(CompactConstructorDeclaration, node_id) == 8);
    assert!(offset_of!(CompactConstructorDeclaration, scope_id) == 12);
    assert!(offset_of!(CompactConstructorDeclaration, modifiers) == 16);
    assert!(offset_of!(CompactConstructorDeclaration, name) == 32);
    assert!(offset_of!(CompactConstructorDeclaration, body) == 52);

    // Padding: 0 bytes
    assert!(size_of::<ReceiverParameter>() == 56);
    assert!(align_of::<ReceiverParameter>() == 4);
    assert!(offset_of!(ReceiverParameter, span) == 0);
    assert!(offset_of!(ReceiverParameter, node_id) == 8);
    assert!(offset_of!(ReceiverParameter, modifiers) == 12);
    assert!(offset_of!(ReceiverParameter, ty) == 28);
    assert!(offset_of!(ReceiverParameter, qualifier) == 36);

    // Padding: 0 bytes
    assert!(size_of::<FormalParameters>() == 32);
    assert!(align_of::<FormalParameters>() == 4);
    assert!(offset_of!(FormalParameters, span) == 0);
    assert!(offset_of!(FormalParameters, node_id) == 8);
    assert!(offset_of!(FormalParameters, receiver) == 12);
    assert!(offset_of!(FormalParameters, items) == 16);

    // Padding: 3 bytes
    assert!(size_of::<FormalParameter>() == 96);
    assert!(align_of::<FormalParameter>() == 4);
    assert!(offset_of!(FormalParameter, span) == 0);
    assert!(offset_of!(FormalParameter, node_id) == 8);
    assert!(offset_of!(FormalParameter, varargs) == 12);
    assert!(offset_of!(FormalParameter, modifiers) == 16);
    assert!(offset_of!(FormalParameter, ty) == 32);
    assert!(offset_of!(FormalParameter, varargs_annotations) == 40);
    assert!(offset_of!(FormalParameter, name) == 56);
    assert!(offset_of!(FormalParameter, dims) == 80);

    // Padding: 0 bytes
    assert!(size_of::<Block>() == 32);
    assert!(align_of::<Block>() == 4);
    assert!(offset_of!(Block, span) == 0);
    assert!(offset_of!(Block, node_id) == 8);
    assert!(offset_of!(Block, scope_id) == 12);
    assert!(offset_of!(Block, statements) == 16);

    assert!(size_of::<Statement>() == 8);
    assert!(align_of::<Statement>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<EmptyStatement>() == 12);
    assert!(align_of::<EmptyStatement>() == 4);
    assert!(offset_of!(EmptyStatement, span) == 0);
    assert!(offset_of!(EmptyStatement, node_id) == 8);

    // Padding: 0 bytes
    assert!(size_of::<IfStatement>() == 36);
    assert!(align_of::<IfStatement>() == 4);
    assert!(offset_of!(IfStatement, span) == 0);
    assert!(offset_of!(IfStatement, node_id) == 8);
    assert!(offset_of!(IfStatement, condition) == 12);
    assert!(offset_of!(IfStatement, then_branch) == 20);
    assert!(offset_of!(IfStatement, else_branch) == 28);

    // Padding: 0 bytes
    assert!(size_of::<SwitchStatement>() == 40);
    assert!(align_of::<SwitchStatement>() == 4);
    assert!(offset_of!(SwitchStatement, span) == 0);
    assert!(offset_of!(SwitchStatement, node_id) == 8);
    assert!(offset_of!(SwitchStatement, scope_id) == 12);
    assert!(offset_of!(SwitchStatement, selector) == 16);
    assert!(offset_of!(SwitchStatement, cases) == 24);

    assert!(size_of::<SwitchItem>() == 8);
    assert!(align_of::<SwitchItem>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<SwitchRule>() == 28);
    assert!(align_of::<SwitchRule>() == 4);
    assert!(offset_of!(SwitchRule, span) == 0);
    assert!(offset_of!(SwitchRule, node_id) == 8);
    assert!(offset_of!(SwitchRule, label) == 12);
    assert!(offset_of!(SwitchRule, body) == 20);

    assert!(size_of::<SwitchRuleBody>() == 8);
    assert!(align_of::<SwitchRuleBody>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<SwitchCase>() == 36);
    assert!(align_of::<SwitchCase>() == 4);
    assert!(offset_of!(SwitchCase, span) == 0);
    assert!(offset_of!(SwitchCase, node_id) == 8);
    assert!(offset_of!(SwitchCase, label) == 12);
    assert!(offset_of!(SwitchCase, statements) == 20);

    assert!(size_of::<SwitchLabel>() == 8);
    assert!(align_of::<SwitchLabel>() == 4);

    // Padding: 3 bytes
    assert!(size_of::<CaseLabel>() == 32);
    assert!(align_of::<CaseLabel>() == 4);
    assert!(offset_of!(CaseLabel, span) == 0);
    assert!(offset_of!(CaseLabel, node_id) == 8);
    assert!(offset_of!(CaseLabel, has_default) == 12);
    assert!(offset_of!(CaseLabel, constants) == 16);

    // Padding: 0 bytes
    assert!(size_of::<CasePatternLabel>() == 36);
    assert!(align_of::<CasePatternLabel>() == 4);
    assert!(offset_of!(CasePatternLabel, span) == 0);
    assert!(offset_of!(CasePatternLabel, node_id) == 8);
    assert!(offset_of!(CasePatternLabel, patterns) == 12);
    assert!(offset_of!(CasePatternLabel, guard) == 28);

    // Padding: 0 bytes
    assert!(size_of::<WhileStatement>() == 28);
    assert!(align_of::<WhileStatement>() == 4);
    assert!(offset_of!(WhileStatement, span) == 0);
    assert!(offset_of!(WhileStatement, node_id) == 8);
    assert!(offset_of!(WhileStatement, condition) == 12);
    assert!(offset_of!(WhileStatement, body) == 20);

    // Padding: 0 bytes
    assert!(size_of::<DoStatement>() == 28);
    assert!(align_of::<DoStatement>() == 4);
    assert!(offset_of!(DoStatement, span) == 0);
    assert!(offset_of!(DoStatement, node_id) == 8);
    assert!(offset_of!(DoStatement, body) == 12);
    assert!(offset_of!(DoStatement, condition) == 20);

    // Padding: 0 bytes
    assert!(size_of::<ForStatement>() == 68);
    assert!(align_of::<ForStatement>() == 4);
    assert!(offset_of!(ForStatement, span) == 0);
    assert!(offset_of!(ForStatement, node_id) == 8);
    assert!(offset_of!(ForStatement, scope_id) == 12);
    assert!(offset_of!(ForStatement, init) == 16);
    assert!(offset_of!(ForStatement, condition) == 24);
    assert!(offset_of!(ForStatement, update) == 32);
    assert!(offset_of!(ForStatement, body) == 60);

    assert!(size_of::<ForInit>() == 8);
    assert!(align_of::<ForInit>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<StatementExpressionList>() == 28);
    assert!(align_of::<StatementExpressionList>() == 4);
    assert!(offset_of!(StatementExpressionList, span) == 0);
    assert!(offset_of!(StatementExpressionList, node_id) == 8);
    assert!(offset_of!(StatementExpressionList, expressions) == 12);

    // Padding: 0 bytes
    assert!(size_of::<EnhancedForStatement>() == 96);
    assert!(align_of::<EnhancedForStatement>() == 4);
    assert!(offset_of!(EnhancedForStatement, span) == 0);
    assert!(offset_of!(EnhancedForStatement, node_id) == 8);
    assert!(offset_of!(EnhancedForStatement, scope_id) == 12);
    assert!(offset_of!(EnhancedForStatement, modifiers) == 16);
    assert!(offset_of!(EnhancedForStatement, ty) == 32);
    assert!(offset_of!(EnhancedForStatement, name) == 40);
    assert!(offset_of!(EnhancedForStatement, dims) == 64);
    assert!(offset_of!(EnhancedForStatement, iterable) == 80);
    assert!(offset_of!(EnhancedForStatement, body) == 88);

    // Padding: 0 bytes
    assert!(size_of::<BreakStatement>() == 32);
    assert!(align_of::<BreakStatement>() == 4);
    assert!(offset_of!(BreakStatement, span) == 0);
    assert!(offset_of!(BreakStatement, node_id) == 8);
    assert!(offset_of!(BreakStatement, label) == 12);

    // Padding: 0 bytes
    assert!(size_of::<ContinueStatement>() == 32);
    assert!(align_of::<ContinueStatement>() == 4);
    assert!(offset_of!(ContinueStatement, span) == 0);
    assert!(offset_of!(ContinueStatement, node_id) == 8);
    assert!(offset_of!(ContinueStatement, label) == 12);

    // Padding: 0 bytes
    assert!(size_of::<ThrowStatement>() == 20);
    assert!(align_of::<ThrowStatement>() == 4);
    assert!(offset_of!(ThrowStatement, span) == 0);
    assert!(offset_of!(ThrowStatement, node_id) == 8);
    assert!(offset_of!(ThrowStatement, value) == 12);

    // Padding: 0 bytes
    assert!(size_of::<AssertStatement>() == 28);
    assert!(align_of::<AssertStatement>() == 4);
    assert!(offset_of!(AssertStatement, span) == 0);
    assert!(offset_of!(AssertStatement, node_id) == 8);
    assert!(offset_of!(AssertStatement, condition) == 12);
    assert!(offset_of!(AssertStatement, message) == 20);

    // Padding: 3 bytes
    assert!(size_of::<TryStatement>() == 116);
    assert!(align_of::<TryStatement>() == 4);
    assert!(offset_of!(TryStatement, span) == 0);
    assert!(offset_of!(TryStatement, node_id) == 8);
    assert!(offset_of!(TryStatement, scope_id) == 12);
    assert!(offset_of!(TryStatement, resources) == 16);
    assert!(offset_of!(TryStatement, block) == 32);
    assert!(offset_of!(TryStatement, catches) == 64);
    assert!(offset_of!(TryStatement, finally_block) == 80);
    assert!(offset_of!(TryStatement, resources_trailing_semicolon) == 112);

    assert!(size_of::<Resource>() == 8);
    assert!(align_of::<Resource>() == 4);

    assert!(size_of::<ResourceAccess>() == 8);
    assert!(align_of::<ResourceAccess>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<ResourceDeclaration>() == 68);
    assert!(align_of::<ResourceDeclaration>() == 4);
    assert!(offset_of!(ResourceDeclaration, span) == 0);
    assert!(offset_of!(ResourceDeclaration, node_id) == 8);
    assert!(offset_of!(ResourceDeclaration, modifiers) == 12);
    assert!(offset_of!(ResourceDeclaration, ty) == 28);
    assert!(offset_of!(ResourceDeclaration, name) == 36);
    assert!(offset_of!(ResourceDeclaration, init) == 60);

    // Padding: 0 bytes
    assert!(size_of::<CatchClause>() == 104);
    assert!(align_of::<CatchClause>() == 4);
    assert!(offset_of!(CatchClause, span) == 0);
    assert!(offset_of!(CatchClause, node_id) == 8);
    assert!(offset_of!(CatchClause, scope_id) == 12);
    assert!(offset_of!(CatchClause, modifiers) == 16);
    assert!(offset_of!(CatchClause, types) == 32);
    assert!(offset_of!(CatchClause, name) == 48);
    assert!(offset_of!(CatchClause, block) == 72);

    // Padding: 0 bytes
    assert!(size_of::<SynchronizedStatement>() == 52);
    assert!(align_of::<SynchronizedStatement>() == 4);
    assert!(offset_of!(SynchronizedStatement, span) == 0);
    assert!(offset_of!(SynchronizedStatement, node_id) == 8);
    assert!(offset_of!(SynchronizedStatement, lock) == 12);
    assert!(offset_of!(SynchronizedStatement, body) == 20);

    // Padding: 0 bytes
    assert!(size_of::<LabeledStatement>() == 40);
    assert!(align_of::<LabeledStatement>() == 4);
    assert!(offset_of!(LabeledStatement, span) == 0);
    assert!(offset_of!(LabeledStatement, node_id) == 8);
    assert!(offset_of!(LabeledStatement, label) == 12);
    assert!(offset_of!(LabeledStatement, statement) == 32);

    // Padding: 3 bytes
    assert!(size_of::<ConstructorInvocation>() == 44);
    assert!(align_of::<ConstructorInvocation>() == 4);
    assert!(offset_of!(ConstructorInvocation, span) == 0);
    assert!(offset_of!(ConstructorInvocation, node_id) == 8);
    assert!(offset_of!(ConstructorInvocation, kind) == 12);
    assert!(offset_of!(ConstructorInvocation, qualifier) == 16);
    assert!(offset_of!(ConstructorInvocation, type_args) == 24);
    assert!(offset_of!(ConstructorInvocation, args) == 28);

    assert!(size_of::<ConstructorInvocationKind>() == 1);
    assert!(align_of::<ConstructorInvocationKind>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<ExpressionStatement>() == 20);
    assert!(align_of::<ExpressionStatement>() == 4);
    assert!(offset_of!(ExpressionStatement, span) == 0);
    assert!(offset_of!(ExpressionStatement, node_id) == 8);
    assert!(offset_of!(ExpressionStatement, expression) == 12);

    // Padding: 0 bytes
    assert!(size_of::<LocalVariableDeclaration>() == 52);
    assert!(align_of::<LocalVariableDeclaration>() == 4);
    assert!(offset_of!(LocalVariableDeclaration, span) == 0);
    assert!(offset_of!(LocalVariableDeclaration, node_id) == 8);
    assert!(offset_of!(LocalVariableDeclaration, modifiers) == 12);
    assert!(offset_of!(LocalVariableDeclaration, ty) == 28);
    assert!(offset_of!(LocalVariableDeclaration, declarators) == 36);

    // Padding: 0 bytes
    assert!(size_of::<ReturnStatement>() == 20);
    assert!(align_of::<ReturnStatement>() == 4);
    assert!(offset_of!(ReturnStatement, span) == 0);
    assert!(offset_of!(ReturnStatement, node_id) == 8);
    assert!(offset_of!(ReturnStatement, value) == 12);

    // Padding: 0 bytes
    assert!(size_of::<YieldStatement>() == 20);
    assert!(align_of::<YieldStatement>() == 4);
    assert!(offset_of!(YieldStatement, span) == 0);
    assert!(offset_of!(YieldStatement, node_id) == 8);
    assert!(offset_of!(YieldStatement, value) == 12);

    // Padding: 0 bytes
    assert!(size_of::<VariableDeclarator>() == 60);
    assert!(align_of::<VariableDeclarator>() == 4);
    assert!(offset_of!(VariableDeclarator, span) == 0);
    assert!(offset_of!(VariableDeclarator, node_id) == 8);
    assert!(offset_of!(VariableDeclarator, name) == 12);
    assert!(offset_of!(VariableDeclarator, dims) == 36);
    assert!(offset_of!(VariableDeclarator, init) == 52);

    assert!(size_of::<VariableInitializer>() == 8);
    assert!(align_of::<VariableInitializer>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<ArrayInitializer>() == 28);
    assert!(align_of::<ArrayInitializer>() == 4);
    assert!(offset_of!(ArrayInitializer, span) == 0);
    assert!(offset_of!(ArrayInitializer, node_id) == 8);
    assert!(offset_of!(ArrayInitializer, elements) == 12);

    assert!(size_of::<Expression>() == 8);
    assert!(align_of::<Expression>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<FieldAccess>() == 40);
    assert!(align_of::<FieldAccess>() == 4);
    assert!(offset_of!(FieldAccess, span) == 0);
    assert!(offset_of!(FieldAccess, node_id) == 8);
    assert!(offset_of!(FieldAccess, target) == 12);
    assert!(offset_of!(FieldAccess, name) == 20);

    // Padding: 0 bytes
    assert!(size_of::<MethodCall>() == 36);
    assert!(align_of::<MethodCall>() == 4);
    assert!(offset_of!(MethodCall, span) == 0);
    assert!(offset_of!(MethodCall, node_id) == 8);
    assert!(offset_of!(MethodCall, callee) == 12);
    assert!(offset_of!(MethodCall, args) == 20);

    assert!(size_of::<Callee>() == 8);
    assert!(align_of::<Callee>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<MethodMember>() == 44);
    assert!(align_of::<MethodMember>() == 4);
    assert!(offset_of!(MethodMember, span) == 0);
    assert!(offset_of!(MethodMember, node_id) == 8);
    assert!(offset_of!(MethodMember, target) == 12);
    assert!(offset_of!(MethodMember, type_args) == 20);
    assert!(offset_of!(MethodMember, name) == 24);

    // Padding: 0 bytes
    assert!(size_of::<ClassLiteral>() == 20);
    assert!(align_of::<ClassLiteral>() == 4);
    assert!(offset_of!(ClassLiteral, span) == 0);
    assert!(offset_of!(ClassLiteral, node_id) == 8);
    assert!(offset_of!(ClassLiteral, ty) == 12);

    // Padding: 0 bytes
    assert!(size_of::<ThisExpression>() == 12);
    assert!(align_of::<ThisExpression>() == 4);
    assert!(offset_of!(ThisExpression, span) == 0);
    assert!(offset_of!(ThisExpression, node_id) == 8);

    // Padding: 0 bytes
    assert!(size_of::<Super>() == 12);
    assert!(align_of::<Super>() == 4);
    assert!(offset_of!(Super, span) == 0);
    assert!(offset_of!(Super, node_id) == 8);

    // Padding: 0 bytes
    assert!(size_of::<QualifiedThis>() == 20);
    assert!(align_of::<QualifiedThis>() == 4);
    assert!(offset_of!(QualifiedThis, span) == 0);
    assert!(offset_of!(QualifiedThis, node_id) == 8);
    assert!(offset_of!(QualifiedThis, qualifier) == 12);

    // Padding: 0 bytes
    assert!(size_of::<QualifiedSuper>() == 20);
    assert!(align_of::<QualifiedSuper>() == 4);
    assert!(offset_of!(QualifiedSuper, span) == 0);
    assert!(offset_of!(QualifiedSuper, node_id) == 8);
    assert!(offset_of!(QualifiedSuper, qualifier) == 12);

    // Padding: 0 bytes
    assert!(size_of::<ClassInstanceCreation>() == 72);
    assert!(align_of::<ClassInstanceCreation>() == 4);
    assert!(offset_of!(ClassInstanceCreation, span) == 0);
    assert!(offset_of!(ClassInstanceCreation, node_id) == 8);
    assert!(offset_of!(ClassInstanceCreation, constructor_type_args) == 12);
    assert!(offset_of!(ClassInstanceCreation, ty) == 16);
    assert!(offset_of!(ClassInstanceCreation, args) == 24);
    assert!(offset_of!(ClassInstanceCreation, body) == 40);

    // Padding: 0 bytes
    assert!(size_of::<QualifiedClassInstanceCreation>() == 112);
    assert!(align_of::<QualifiedClassInstanceCreation>() == 4);
    assert!(offset_of!(QualifiedClassInstanceCreation, span) == 0);
    assert!(offset_of!(QualifiedClassInstanceCreation, node_id) == 8);
    assert!(offset_of!(QualifiedClassInstanceCreation, qualifier) == 12);
    assert!(offset_of!(QualifiedClassInstanceCreation, constructor_type_args) == 20);
    assert!(offset_of!(QualifiedClassInstanceCreation, annotations) == 24);
    assert!(offset_of!(QualifiedClassInstanceCreation, name) == 40);
    assert!(offset_of!(QualifiedClassInstanceCreation, type_args) == 60);
    assert!(offset_of!(QualifiedClassInstanceCreation, args) == 64);
    assert!(offset_of!(QualifiedClassInstanceCreation, body) == 80);

    // Padding: 0 bytes
    assert!(size_of::<ArrayCreation>() == 64);
    assert!(align_of::<ArrayCreation>() == 4);
    assert!(offset_of!(ArrayCreation, span) == 0);
    assert!(offset_of!(ArrayCreation, node_id) == 8);
    assert!(offset_of!(ArrayCreation, element) == 12);
    assert!(offset_of!(ArrayCreation, dims) == 20);
    assert!(offset_of!(ArrayCreation, init) == 36);

    // Padding: 0 bytes
    assert!(size_of::<ArrayDimension>() == 36);
    assert!(align_of::<ArrayDimension>() == 4);
    assert!(offset_of!(ArrayDimension, span) == 0);
    assert!(offset_of!(ArrayDimension, node_id) == 8);
    assert!(offset_of!(ArrayDimension, annotations) == 12);
    assert!(offset_of!(ArrayDimension, length) == 28);

    // Padding: 0 bytes
    assert!(size_of::<ArrayAccess>() == 28);
    assert!(align_of::<ArrayAccess>() == 4);
    assert!(offset_of!(ArrayAccess, span) == 0);
    assert!(offset_of!(ArrayAccess, node_id) == 8);
    assert!(offset_of!(ArrayAccess, array) == 12);
    assert!(offset_of!(ArrayAccess, index) == 20);

    // Padding: 2 bytes
    assert!(size_of::<UpdateExpression>() == 24);
    assert!(align_of::<UpdateExpression>() == 4);
    assert!(offset_of!(UpdateExpression, span) == 0);
    assert!(offset_of!(UpdateExpression, node_id) == 8);
    assert!(offset_of!(UpdateExpression, op) == 12);
    assert!(offset_of!(UpdateExpression, prefix) == 13);
    assert!(offset_of!(UpdateExpression, target) == 16);

    // Padding: 3 bytes
    assert!(size_of::<UnaryExpression>() == 24);
    assert!(align_of::<UnaryExpression>() == 4);
    assert!(offset_of!(UnaryExpression, span) == 0);
    assert!(offset_of!(UnaryExpression, node_id) == 8);
    assert!(offset_of!(UnaryExpression, op) == 12);
    assert!(offset_of!(UnaryExpression, operand) == 16);

    // Padding: 0 bytes
    assert!(size_of::<CastExpression>() == 44);
    assert!(align_of::<CastExpression>() == 4);
    assert!(offset_of!(CastExpression, span) == 0);
    assert!(offset_of!(CastExpression, node_id) == 8);
    assert!(offset_of!(CastExpression, ty) == 12);
    assert!(offset_of!(CastExpression, additional_bounds) == 20);
    assert!(offset_of!(CastExpression, operand) == 36);

    // Padding: 0 bytes
    assert!(size_of::<InstanceofExpression>() == 28);
    assert!(align_of::<InstanceofExpression>() == 4);
    assert!(offset_of!(InstanceofExpression, span) == 0);
    assert!(offset_of!(InstanceofExpression, node_id) == 8);
    assert!(offset_of!(InstanceofExpression, operand) == 12);
    assert!(offset_of!(InstanceofExpression, rhs) == 20);

    assert!(size_of::<InstanceofRhs>() == 8);
    assert!(align_of::<InstanceofRhs>() == 4);

    assert!(size_of::<Pattern>() == 8);
    assert!(align_of::<Pattern>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<TypePattern>() == 60);
    assert!(align_of::<TypePattern>() == 4);
    assert!(offset_of!(TypePattern, span) == 0);
    assert!(offset_of!(TypePattern, node_id) == 8);
    assert!(offset_of!(TypePattern, modifiers) == 12);
    assert!(offset_of!(TypePattern, ty) == 28);
    assert!(offset_of!(TypePattern, name) == 36);

    // Padding: 0 bytes
    assert!(size_of::<RecordPattern>() == 36);
    assert!(align_of::<RecordPattern>() == 4);
    assert!(offset_of!(RecordPattern, span) == 0);
    assert!(offset_of!(RecordPattern, node_id) == 8);
    assert!(offset_of!(RecordPattern, ty) == 12);
    assert!(offset_of!(RecordPattern, components) == 20);

    assert!(size_of::<ComponentPattern>() == 8);
    assert!(align_of::<ComponentPattern>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<MatchAllPattern>() == 12);
    assert!(align_of::<MatchAllPattern>() == 4);
    assert!(offset_of!(MatchAllPattern, span) == 0);
    assert!(offset_of!(MatchAllPattern, node_id) == 8);

    // Padding: 3 bytes
    assert!(size_of::<BinaryExpression>() == 32);
    assert!(align_of::<BinaryExpression>() == 4);
    assert!(offset_of!(BinaryExpression, span) == 0);
    assert!(offset_of!(BinaryExpression, node_id) == 8);
    assert!(offset_of!(BinaryExpression, op) == 12);
    assert!(offset_of!(BinaryExpression, lhs) == 16);
    assert!(offset_of!(BinaryExpression, rhs) == 24);

    // Padding: 3 bytes
    assert!(size_of::<AssignmentExpression>() == 32);
    assert!(align_of::<AssignmentExpression>() == 4);
    assert!(offset_of!(AssignmentExpression, span) == 0);
    assert!(offset_of!(AssignmentExpression, node_id) == 8);
    assert!(offset_of!(AssignmentExpression, op) == 12);
    assert!(offset_of!(AssignmentExpression, target) == 16);
    assert!(offset_of!(AssignmentExpression, value) == 24);

    assert!(size_of::<AssignmentTarget>() == 8);
    assert!(align_of::<AssignmentTarget>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<ConditionalExpression>() == 36);
    assert!(align_of::<ConditionalExpression>() == 4);
    assert!(offset_of!(ConditionalExpression, span) == 0);
    assert!(offset_of!(ConditionalExpression, node_id) == 8);
    assert!(offset_of!(ConditionalExpression, condition) == 12);
    assert!(offset_of!(ConditionalExpression, when_true) == 20);
    assert!(offset_of!(ConditionalExpression, when_false) == 28);

    // Padding: 0 bytes
    assert!(size_of::<SwitchExpression>() == 40);
    assert!(align_of::<SwitchExpression>() == 4);
    assert!(offset_of!(SwitchExpression, span) == 0);
    assert!(offset_of!(SwitchExpression, node_id) == 8);
    assert!(offset_of!(SwitchExpression, scope_id) == 12);
    assert!(offset_of!(SwitchExpression, selector) == 16);
    assert!(offset_of!(SwitchExpression, cases) == 24);

    // Padding: 0 bytes
    assert!(size_of::<ParenthesizedExpression>() == 20);
    assert!(align_of::<ParenthesizedExpression>() == 4);
    assert!(offset_of!(ParenthesizedExpression, span) == 0);
    assert!(offset_of!(ParenthesizedExpression, node_id) == 8);
    assert!(offset_of!(ParenthesizedExpression, expression) == 12);

    // Padding: 0 bytes
    assert!(size_of::<LambdaExpression>() == 56);
    assert!(align_of::<LambdaExpression>() == 4);
    assert!(offset_of!(LambdaExpression, span) == 0);
    assert!(offset_of!(LambdaExpression, node_id) == 8);
    assert!(offset_of!(LambdaExpression, scope_id) == 12);
    assert!(offset_of!(LambdaExpression, params) == 16);
    assert!(offset_of!(LambdaExpression, body) == 48);

    assert!(size_of::<LambdaBody>() == 8);
    assert!(align_of::<LambdaBody>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<MethodReference>() == 44);
    assert!(align_of::<MethodReference>() == 4);
    assert!(offset_of!(MethodReference, span) == 0);
    assert!(offset_of!(MethodReference, node_id) == 8);
    assert!(offset_of!(MethodReference, target) == 12);
    assert!(offset_of!(MethodReference, type_args) == 20);
    assert!(offset_of!(MethodReference, method) == 24);

    assert!(size_of::<MethodReferenceTarget>() == 8);
    assert!(align_of::<MethodReferenceTarget>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<TypeParameters>() == 28);
    assert!(align_of::<TypeParameters>() == 4);
    assert!(offset_of!(TypeParameters, span) == 0);
    assert!(offset_of!(TypeParameters, node_id) == 8);
    assert!(offset_of!(TypeParameters, params) == 12);

    // Padding: 0 bytes
    assert!(size_of::<TypeParameter>() == 68);
    assert!(align_of::<TypeParameter>() == 4);
    assert!(offset_of!(TypeParameter, span) == 0);
    assert!(offset_of!(TypeParameter, node_id) == 8);
    assert!(offset_of!(TypeParameter, annotations) == 12);
    assert!(offset_of!(TypeParameter, name) == 28);
    assert!(offset_of!(TypeParameter, bounds) == 52);

    assert!(size_of::<VariableType>() == 8);
    assert!(align_of::<VariableType>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<VarType>() == 12);
    assert!(align_of::<VarType>() == 4);
    assert!(offset_of!(VarType, span) == 0);
    assert!(offset_of!(VarType, node_id) == 8);

    assert!(size_of::<Type>() == 8);
    assert!(align_of::<Type>() == 4);

    assert!(size_of::<MethodResult>() == 8);
    assert!(align_of::<MethodResult>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<VoidType>() == 12);
    assert!(align_of::<VoidType>() == 4);
    assert!(offset_of!(VoidType, span) == 0);
    assert!(offset_of!(VoidType, node_id) == 8);

    // Padding: 3 bytes
    assert!(size_of::<PrimitiveType>() == 32);
    assert!(align_of::<PrimitiveType>() == 4);
    assert!(offset_of!(PrimitiveType, span) == 0);
    assert!(offset_of!(PrimitiveType, node_id) == 8);
    assert!(offset_of!(PrimitiveType, kind) == 12);
    assert!(offset_of!(PrimitiveType, annotations) == 16);

    assert!(size_of::<PrimitiveTypeKind>() == 1);
    assert!(align_of::<PrimitiveTypeKind>() == 1);

    assert!(size_of::<ClassType>() == 8);
    assert!(align_of::<ClassType>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<SimpleClassType>() == 56);
    assert!(align_of::<SimpleClassType>() == 4);
    assert!(offset_of!(SimpleClassType, span) == 0);
    assert!(offset_of!(SimpleClassType, node_id) == 8);
    assert!(offset_of!(SimpleClassType, annotations) == 12);
    assert!(offset_of!(SimpleClassType, name) == 28);
    assert!(offset_of!(SimpleClassType, args) == 52);

    // Padding: 0 bytes
    assert!(size_of::<QualifiedClassType>() == 60);
    assert!(align_of::<QualifiedClassType>() == 4);
    assert!(offset_of!(QualifiedClassType, span) == 0);
    assert!(offset_of!(QualifiedClassType, node_id) == 8);
    assert!(offset_of!(QualifiedClassType, qualifier) == 12);
    assert!(offset_of!(QualifiedClassType, annotations) == 20);
    assert!(offset_of!(QualifiedClassType, name) == 36);
    assert!(offset_of!(QualifiedClassType, args) == 56);

    // Padding: 0 bytes
    assert!(size_of::<ArrayType>() == 36);
    assert!(align_of::<ArrayType>() == 4);
    assert!(offset_of!(ArrayType, span) == 0);
    assert!(offset_of!(ArrayType, node_id) == 8);
    assert!(offset_of!(ArrayType, element) == 12);
    assert!(offset_of!(ArrayType, annotations) == 20);

    // Padding: 0 bytes
    assert!(size_of::<ArrayTypeDimension>() == 28);
    assert!(align_of::<ArrayTypeDimension>() == 4);
    assert!(offset_of!(ArrayTypeDimension, span) == 0);
    assert!(offset_of!(ArrayTypeDimension, node_id) == 8);
    assert!(offset_of!(ArrayTypeDimension, annotations) == 12);

    // Padding: 0 bytes
    assert!(size_of::<TypeArguments>() == 28);
    assert!(align_of::<TypeArguments>() == 4);
    assert!(offset_of!(TypeArguments, span) == 0);
    assert!(offset_of!(TypeArguments, node_id) == 8);
    assert!(offset_of!(TypeArguments, args) == 12);

    assert!(size_of::<TypeArgument>() == 8);
    assert!(align_of::<TypeArgument>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<WildcardTypeArgument>() == 52);
    assert!(align_of::<WildcardTypeArgument>() == 4);
    assert!(offset_of!(WildcardTypeArgument, span) == 0);
    assert!(offset_of!(WildcardTypeArgument, node_id) == 8);
    assert!(offset_of!(WildcardTypeArgument, annotations) == 12);
    assert!(offset_of!(WildcardTypeArgument, bound) == 28);

    // Padding: 3 bytes
    assert!(size_of::<WildcardBound>() == 24);
    assert!(align_of::<WildcardBound>() == 4);
    assert!(offset_of!(WildcardBound, span) == 0);
    assert!(offset_of!(WildcardBound, node_id) == 8);
    assert!(offset_of!(WildcardBound, kind) == 12);
    assert!(offset_of!(WildcardBound, ty) == 16);

    assert!(size_of::<WildcardBoundKind>() == 1);
    assert!(align_of::<WildcardBoundKind>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<IntLiteral>() == 20);
    assert!(align_of::<IntLiteral>() == 4);
    assert!(offset_of!(IntLiteral, span) == 0);
    assert!(offset_of!(IntLiteral, node_id) == 8);
    assert!(offset_of!(IntLiteral, raw) == 12);

    // Padding: 0 bytes
    assert!(size_of::<FloatLiteral>() == 20);
    assert!(align_of::<FloatLiteral>() == 4);
    assert!(offset_of!(FloatLiteral, span) == 0);
    assert!(offset_of!(FloatLiteral, node_id) == 8);
    assert!(offset_of!(FloatLiteral, raw) == 12);

    // Padding: 0 bytes
    assert!(size_of::<CharLiteral>() == 20);
    assert!(align_of::<CharLiteral>() == 4);
    assert!(offset_of!(CharLiteral, span) == 0);
    assert!(offset_of!(CharLiteral, node_id) == 8);
    assert!(offset_of!(CharLiteral, raw) == 12);

    // Padding: 0 bytes
    assert!(size_of::<StringLiteral>() == 20);
    assert!(align_of::<StringLiteral>() == 4);
    assert!(offset_of!(StringLiteral, span) == 0);
    assert!(offset_of!(StringLiteral, node_id) == 8);
    assert!(offset_of!(StringLiteral, raw) == 12);

    // Padding: 3 bytes
    assert!(size_of::<BooleanLiteral>() == 16);
    assert!(align_of::<BooleanLiteral>() == 4);
    assert!(offset_of!(BooleanLiteral, span) == 0);
    assert!(offset_of!(BooleanLiteral, node_id) == 8);
    assert!(offset_of!(BooleanLiteral, value) == 12);

    // Padding: 0 bytes
    assert!(size_of::<NullLiteral>() == 12);
    assert!(align_of::<NullLiteral>() == 4);
    assert!(offset_of!(NullLiteral, span) == 0);
    assert!(offset_of!(NullLiteral, node_id) == 8);

    // Padding: 0 bytes
    assert!(size_of::<IdentifierName>() == 20);
    assert!(align_of::<IdentifierName>() == 4);
    assert!(offset_of!(IdentifierName, span) == 0);
    assert!(offset_of!(IdentifierName, node_id) == 8);
    assert!(offset_of!(IdentifierName, name) == 12);

    // Padding: 0 bytes
    assert!(size_of::<IdentifierReference>() == 24);
    assert!(align_of::<IdentifierReference>() == 4);
    assert!(offset_of!(IdentifierReference, span) == 0);
    assert!(offset_of!(IdentifierReference, node_id) == 8);
    assert!(offset_of!(IdentifierReference, reference_id) == 12);
    assert!(offset_of!(IdentifierReference, name) == 16);

    // Padding: 0 bytes
    assert!(size_of::<BindingIdentifier>() == 24);
    assert!(align_of::<BindingIdentifier>() == 4);
    assert!(offset_of!(BindingIdentifier, span) == 0);
    assert!(offset_of!(BindingIdentifier, node_id) == 8);
    assert!(offset_of!(BindingIdentifier, symbol_id) == 12);
    assert!(offset_of!(BindingIdentifier, name) == 16);

    // Padding: 0 bytes
    assert!(size_of::<LabelIdentifier>() == 20);
    assert!(align_of::<LabelIdentifier>() == 4);
    assert!(offset_of!(LabelIdentifier, span) == 0);
    assert!(offset_of!(LabelIdentifier, node_id) == 8);
    assert!(offset_of!(LabelIdentifier, name) == 12);

    assert!(size_of::<Name>() == 8);
    assert!(align_of::<Name>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<QualifiedName>() == 40);
    assert!(align_of::<QualifiedName>() == 4);
    assert!(offset_of!(QualifiedName, span) == 0);
    assert!(offset_of!(QualifiedName, node_id) == 8);
    assert!(offset_of!(QualifiedName, qualifier) == 12);
    assert!(offset_of!(QualifiedName, name) == 20);

    assert!(size_of::<TypeName>() == 8);
    assert!(align_of::<TypeName>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<QualifiedTypeName>() == 40);
    assert!(align_of::<QualifiedTypeName>() == 4);
    assert!(offset_of!(QualifiedTypeName, span) == 0);
    assert!(offset_of!(QualifiedTypeName, node_id) == 8);
    assert!(offset_of!(QualifiedTypeName, qualifier) == 12);
    assert!(offset_of!(QualifiedTypeName, name) == 20);

    assert!(size_of::<Modifier>() == 16);
    assert!(align_of::<Modifier>() == 4);

    // Padding: 3 bytes
    assert!(size_of::<KeywordModifier>() == 12);
    assert!(align_of::<KeywordModifier>() == 4);
    assert!(offset_of!(KeywordModifier, span) == 0);
    assert!(offset_of!(KeywordModifier, kind) == 8);

    // Padding: 0 bytes
    assert!(size_of::<Annotation>() == 36);
    assert!(align_of::<Annotation>() == 4);
    assert!(offset_of!(Annotation, span) == 0);
    assert!(offset_of!(Annotation, node_id) == 8);
    assert!(offset_of!(Annotation, name) == 12);
    assert!(offset_of!(Annotation, args) == 20);

    assert!(size_of::<AnnotationArgument>() == 8);
    assert!(align_of::<AnnotationArgument>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<ElementValuePair>() == 40);
    assert!(align_of::<ElementValuePair>() == 4);
    assert!(offset_of!(ElementValuePair, span) == 0);
    assert!(offset_of!(ElementValuePair, node_id) == 8);
    assert!(offset_of!(ElementValuePair, name) == 12);
    assert!(offset_of!(ElementValuePair, value) == 32);

    assert!(size_of::<ElementValue>() == 8);
    assert!(align_of::<ElementValue>() == 4);

    // Padding: 3 bytes
    assert!(size_of::<ElementValueArrayInitializer>() == 32);
    assert!(align_of::<ElementValueArrayInitializer>() == 4);
    assert!(offset_of!(ElementValueArrayInitializer, span) == 0);
    assert!(offset_of!(ElementValueArrayInitializer, node_id) == 8);
    assert!(offset_of!(ElementValueArrayInitializer, trailing_comma) == 12);
    assert!(offset_of!(ElementValueArrayInitializer, values) == 16);

    assert!(size_of::<ModifierKind>() == 1);
    assert!(align_of::<ModifierKind>() == 1);

    assert!(size_of::<CommentKind>() == 1);
    assert!(align_of::<CommentKind>() == 1);

    assert!(size_of::<CommentPosition>() == 1);
    assert!(align_of::<CommentPosition>() == 1);

    assert!(size_of::<CommentContent>() == 1);
    assert!(align_of::<CommentContent>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<CommentNewlines>() == 1);
    assert!(align_of::<CommentNewlines>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<Comment>() == 16);
    assert!(align_of::<Comment>() == 4);
    assert!(offset_of!(Comment, span) == 0);
    assert!(offset_of!(Comment, attached_to) == 8);
    assert!(offset_of!(Comment, kind) == 12);
    assert!(offset_of!(Comment, position) == 13);
    assert!(offset_of!(Comment, newlines) == 14);
    assert!(offset_of!(Comment, content) == 15);

    // Padding: 0 bytes
    assert!(size_of::<Span>() == 8);
    assert!(align_of::<Span>() == 4);
    assert!(offset_of!(Span, start) == 0);
    assert!(offset_of!(Span, end) == 4);

    // Padding: 0 bytes
    assert!(size_of::<NonMaxU32>() == 4);
    assert!(align_of::<NonMaxU32>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<NodeId>() == 4);
    assert!(align_of::<NodeId>() == 4);

    assert!(size_of::<UnaryOp>() == 1);
    assert!(align_of::<UnaryOp>() == 1);

    assert!(size_of::<UpdateOp>() == 1);
    assert!(align_of::<UpdateOp>() == 1);

    assert!(size_of::<BinaryOp>() == 1);
    assert!(align_of::<BinaryOp>() == 1);

    assert!(size_of::<AssignOp>() == 1);
    assert!(align_of::<AssignOp>() == 1);

    // Padding: 0 bytes
    assert!(size_of::<ReferenceId>() == 4);
    assert!(align_of::<ReferenceId>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<ScopeId>() == 4);
    assert!(align_of::<ScopeId>() == 4);

    // Padding: 0 bytes
    assert!(size_of::<SymbolId>() == 4);
    assert!(align_of::<SymbolId>() == 4);
};

#[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
const _: () = panic!("Platforms with pointer width other than 64 or 32 bit are not supported");

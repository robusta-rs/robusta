// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/visit.rs`.

//! Visitor Pattern
//!
//! See:
//! * [visitor pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
//! * [rustc visitor](https://github.com/rust-lang/rust/blob/1.82.0/compiler/rustc_ast/src/visit.rs)

#![expect(unused_variables, clippy::semicolon_if_nothing_returned)]
#![allow(clippy::needless_pass_by_ref_mut)]

use std::cell::Cell;

use robusta_allocator::Vec;
use robusta_ast::*;
use robusta_syntax::scope::ScopeId;
use smallvec::SmallVec;

use walk::*;

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

    #[inline]
    fn enter_scope(&mut self, scope_id: &Cell<Option<ScopeId>>) {}
    #[inline]
    fn leave_scope(&mut self) {}

    #[inline]
    fn visit_compilation_unit(&mut self, it: &'a CompilationUnit<'a>) {
        walk_compilation_unit(self, it);
    }

    #[inline]
    fn visit_compilation_unit_content(&mut self, it: &'a CompilationUnitContent<'a>) {
        walk_compilation_unit_content(self, it);
    }

    #[inline]
    fn visit_ordinary_compilation_unit(&mut self, it: &'a OrdinaryCompilationUnit<'a>) {
        walk_ordinary_compilation_unit(self, it);
    }

    #[inline]
    fn visit_modular_compilation_unit(&mut self, it: &'a ModularCompilationUnit<'a>) {
        walk_modular_compilation_unit(self, it);
    }

    #[inline]
    fn visit_package_declaration(&mut self, it: &'a PackageDeclaration<'a>) {
        walk_package_declaration(self, it);
    }

    #[inline]
    fn visit_import_declaration(&mut self, it: &'a ImportDeclaration<'a>) {
        walk_import_declaration(self, it);
    }

    #[inline]
    fn visit_compact_compilation_unit(&mut self, it: &'a CompactCompilationUnit<'a>) {
        walk_compact_compilation_unit(self, it);
    }

    #[inline]
    fn visit_module_declaration(&mut self, it: &'a ModuleDeclaration<'a>) {
        walk_module_declaration(self, it);
    }

    #[inline]
    fn visit_module_directive(&mut self, it: &'a ModuleDirective<'a>) {
        walk_module_directive(self, it);
    }

    #[inline]
    fn visit_requires_directive(&mut self, it: &'a RequiresDirective<'a>) {
        walk_requires_directive(self, it);
    }

    #[inline]
    fn visit_package_directive(&mut self, it: &'a PackageDirective<'a>) {
        walk_package_directive(self, it);
    }

    #[inline]
    fn visit_uses_directive(&mut self, it: &'a UsesDirective<'a>) {
        walk_uses_directive(self, it);
    }

    #[inline]
    fn visit_provides_directive(&mut self, it: &'a ProvidesDirective<'a>) {
        walk_provides_directive(self, it);
    }

    #[inline]
    fn visit_type_declaration(&mut self, it: &'a TypeDeclaration<'a>) {
        walk_type_declaration(self, it);
    }

    #[inline]
    fn visit_local_type_declaration(&mut self, it: &'a LocalTypeDeclaration<'a>) {
        walk_local_type_declaration(self, it);
    }

    #[inline]
    fn visit_class_declaration(&mut self, it: &'a ClassDeclaration<'a>) {
        walk_class_declaration(self, it);
    }

    #[inline]
    fn visit_class_body(&mut self, it: &'a ClassBody<'a>) {
        walk_class_body(self, it);
    }

    #[inline]
    fn visit_interface_declaration(&mut self, it: &'a InterfaceDeclaration<'a>) {
        walk_interface_declaration(self, it);
    }

    #[inline]
    fn visit_interface_body(&mut self, it: &'a InterfaceBody<'a>) {
        walk_interface_body(self, it);
    }

    #[inline]
    fn visit_enum_declaration(&mut self, it: &'a EnumDeclaration<'a>) {
        walk_enum_declaration(self, it);
    }

    #[inline]
    fn visit_enum_body(&mut self, it: &'a EnumBody<'a>) {
        walk_enum_body(self, it);
    }

    #[inline]
    fn visit_enum_constant(&mut self, it: &'a EnumConstant<'a>) {
        walk_enum_constant(self, it);
    }

    #[inline]
    fn visit_record_declaration(&mut self, it: &'a RecordDeclaration<'a>) {
        walk_record_declaration(self, it);
    }

    #[inline]
    fn visit_record_header(&mut self, it: &'a RecordHeader<'a>) {
        walk_record_header(self, it);
    }

    #[inline]
    fn visit_record_component(&mut self, it: &'a RecordComponent<'a>) {
        walk_record_component(self, it);
    }

    #[inline]
    fn visit_annotation_interface_declaration(
        &mut self,
        it: &'a AnnotationInterfaceDeclaration<'a>,
    ) {
        walk_annotation_interface_declaration(self, it);
    }

    #[inline]
    fn visit_annotation_interface_body(&mut self, it: &'a AnnotationInterfaceBody<'a>) {
        walk_annotation_interface_body(self, it);
    }

    #[inline]
    fn visit_annotation_interface_member(&mut self, it: &'a AnnotationInterfaceMember<'a>) {
        walk_annotation_interface_member(self, it);
    }

    #[inline]
    fn visit_annotation_interface_element_declaration(
        &mut self,
        it: &'a AnnotationInterfaceElementDeclaration<'a>,
    ) {
        walk_annotation_interface_element_declaration(self, it);
    }

    #[inline]
    fn visit_class_member(&mut self, it: &'a ClassMember<'a>) {
        walk_class_member(self, it);
    }

    #[inline]
    fn visit_interface_member(&mut self, it: &'a InterfaceMember<'a>) {
        walk_interface_member(self, it);
    }

    #[inline]
    fn visit_initializer_block(&mut self, it: &'a InitializerBlock<'a>) {
        walk_initializer_block(self, it);
    }

    #[inline]
    fn visit_field_declaration(&mut self, it: &'a FieldDeclaration<'a>) {
        walk_field_declaration(self, it);
    }

    #[inline]
    fn visit_method_declaration(&mut self, it: &'a MethodDeclaration<'a>) {
        walk_method_declaration(self, it);
    }

    #[inline]
    fn visit_constructor_declaration(&mut self, it: &'a ConstructorDeclaration<'a>) {
        walk_constructor_declaration(self, it);
    }

    #[inline]
    fn visit_compact_constructor_declaration(&mut self, it: &'a CompactConstructorDeclaration<'a>) {
        walk_compact_constructor_declaration(self, it);
    }

    #[inline]
    fn visit_receiver_parameter(&mut self, it: &'a ReceiverParameter<'a>) {
        walk_receiver_parameter(self, it);
    }

    #[inline]
    fn visit_formal_parameters(&mut self, it: &'a FormalParameters<'a>) {
        walk_formal_parameters(self, it);
    }

    #[inline]
    fn visit_formal_parameter(&mut self, it: &'a FormalParameter<'a>) {
        walk_formal_parameter(self, it);
    }

    #[inline]
    fn visit_block(&mut self, it: &'a Block<'a>) {
        walk_block(self, it);
    }

    #[inline]
    fn visit_statement(&mut self, it: &'a Statement<'a>) {
        walk_statement(self, it);
    }

    #[inline]
    fn visit_empty_statement(&mut self, it: &'a EmptyStatement) {
        walk_empty_statement(self, it);
    }

    #[inline]
    fn visit_if_statement(&mut self, it: &'a IfStatement<'a>) {
        walk_if_statement(self, it);
    }

    #[inline]
    fn visit_switch_statement(&mut self, it: &'a SwitchStatement<'a>) {
        walk_switch_statement(self, it);
    }

    #[inline]
    fn visit_switch_item(&mut self, it: &'a SwitchItem<'a>) {
        walk_switch_item(self, it);
    }

    #[inline]
    fn visit_switch_rule(&mut self, it: &'a SwitchRule<'a>) {
        walk_switch_rule(self, it);
    }

    #[inline]
    fn visit_switch_rule_body(&mut self, it: &'a SwitchRuleBody<'a>) {
        walk_switch_rule_body(self, it);
    }

    #[inline]
    fn visit_switch_case(&mut self, it: &'a SwitchCase<'a>) {
        walk_switch_case(self, it);
    }

    #[inline]
    fn visit_switch_label(&mut self, it: &'a SwitchLabel<'a>) {
        walk_switch_label(self, it);
    }

    #[inline]
    fn visit_case_label(&mut self, it: &'a CaseLabel<'a>) {
        walk_case_label(self, it);
    }

    #[inline]
    fn visit_case_pattern_label(&mut self, it: &'a CasePatternLabel<'a>) {
        walk_case_pattern_label(self, it);
    }

    #[inline]
    fn visit_while_statement(&mut self, it: &'a WhileStatement<'a>) {
        walk_while_statement(self, it);
    }

    #[inline]
    fn visit_do_statement(&mut self, it: &'a DoStatement<'a>) {
        walk_do_statement(self, it);
    }

    #[inline]
    fn visit_for_statement(&mut self, it: &'a ForStatement<'a>) {
        walk_for_statement(self, it);
    }

    #[inline]
    fn visit_for_init(&mut self, it: &'a ForInit<'a>) {
        walk_for_init(self, it);
    }

    #[inline]
    fn visit_statement_expression_list(&mut self, it: &'a StatementExpressionList<'a>) {
        walk_statement_expression_list(self, it);
    }

    #[inline]
    fn visit_enhanced_for_statement(&mut self, it: &'a EnhancedForStatement<'a>) {
        walk_enhanced_for_statement(self, it);
    }

    #[inline]
    fn visit_break_statement(&mut self, it: &'a BreakStatement<'a>) {
        walk_break_statement(self, it);
    }

    #[inline]
    fn visit_continue_statement(&mut self, it: &'a ContinueStatement<'a>) {
        walk_continue_statement(self, it);
    }

    #[inline]
    fn visit_throw_statement(&mut self, it: &'a ThrowStatement<'a>) {
        walk_throw_statement(self, it);
    }

    #[inline]
    fn visit_assert_statement(&mut self, it: &'a AssertStatement<'a>) {
        walk_assert_statement(self, it);
    }

    #[inline]
    fn visit_try_statement(&mut self, it: &'a TryStatement<'a>) {
        walk_try_statement(self, it);
    }

    #[inline]
    fn visit_resource(&mut self, it: &'a Resource<'a>) {
        walk_resource(self, it);
    }

    #[inline]
    fn visit_resource_access(&mut self, it: &'a ResourceAccess<'a>) {
        walk_resource_access(self, it);
    }

    #[inline]
    fn visit_catch_clause(&mut self, it: &'a CatchClause<'a>) {
        walk_catch_clause(self, it);
    }

    #[inline]
    fn visit_catch_parameter(&mut self, it: &'a CatchParameter<'a>) {
        walk_catch_parameter(self, it);
    }

    #[inline]
    fn visit_catch_type(&mut self, it: &'a CatchType<'a>) {
        walk_catch_type(self, it);
    }

    #[inline]
    fn visit_union_type(&mut self, it: &'a UnionType<'a>) {
        walk_union_type(self, it);
    }

    #[inline]
    fn visit_synchronized_statement(&mut self, it: &'a SynchronizedStatement<'a>) {
        walk_synchronized_statement(self, it);
    }

    #[inline]
    fn visit_labeled_statement(&mut self, it: &'a LabeledStatement<'a>) {
        walk_labeled_statement(self, it);
    }

    #[inline]
    fn visit_constructor_invocation(&mut self, it: &'a ConstructorInvocation<'a>) {
        walk_constructor_invocation(self, it);
    }

    #[inline]
    fn visit_expression_statement(&mut self, it: &'a ExpressionStatement<'a>) {
        walk_expression_statement(self, it);
    }

    #[inline]
    fn visit_local_variable_declaration(&mut self, it: &'a LocalVariableDeclaration<'a>) {
        walk_local_variable_declaration(self, it);
    }

    #[inline]
    fn visit_return_statement(&mut self, it: &'a ReturnStatement<'a>) {
        walk_return_statement(self, it);
    }

    #[inline]
    fn visit_yield_statement(&mut self, it: &'a YieldStatement<'a>) {
        walk_yield_statement(self, it);
    }

    #[inline]
    fn visit_variable_declarator(&mut self, it: &'a VariableDeclarator<'a>) {
        walk_variable_declarator(self, it);
    }

    #[inline]
    fn visit_variable_initializer(&mut self, it: &'a VariableInitializer<'a>) {
        walk_variable_initializer(self, it);
    }

    #[inline]
    fn visit_array_initializer(&mut self, it: &'a ArrayInitializer<'a>) {
        walk_array_initializer(self, it);
    }

    #[inline]
    fn visit_expression(&mut self, it: &'a Expression<'a>) {
        walk_expression(self, it);
    }

    #[inline]
    fn visit_field_access(&mut self, it: &'a FieldAccess<'a>) {
        walk_field_access(self, it);
    }

    #[inline]
    fn visit_method_call(&mut self, it: &'a MethodCall<'a>) {
        walk_method_call(self, it);
    }

    #[inline]
    fn visit_callee(&mut self, it: &'a Callee<'a>) {
        walk_callee(self, it);
    }

    #[inline]
    fn visit_method_member(&mut self, it: &'a MethodMember<'a>) {
        walk_method_member(self, it);
    }

    #[inline]
    fn visit_class_literal(&mut self, it: &'a ClassLiteral<'a>) {
        walk_class_literal(self, it);
    }

    #[inline]
    fn visit_this_expression(&mut self, it: &'a ThisExpression) {
        walk_this_expression(self, it);
    }

    #[inline]
    fn visit_super(&mut self, it: &'a Super) {
        walk_super(self, it);
    }

    #[inline]
    fn visit_qualified_this(&mut self, it: &'a QualifiedThis<'a>) {
        walk_qualified_this(self, it);
    }

    #[inline]
    fn visit_qualified_super(&mut self, it: &'a QualifiedSuper<'a>) {
        walk_qualified_super(self, it);
    }

    #[inline]
    fn visit_class_instance_creation(&mut self, it: &'a ClassInstanceCreation<'a>) {
        walk_class_instance_creation(self, it);
    }

    #[inline]
    fn visit_qualified_class_instance_creation(
        &mut self,
        it: &'a QualifiedClassInstanceCreation<'a>,
    ) {
        walk_qualified_class_instance_creation(self, it);
    }

    #[inline]
    fn visit_array_creation(&mut self, it: &'a ArrayCreation<'a>) {
        walk_array_creation(self, it);
    }

    #[inline]
    fn visit_array_dimension(&mut self, it: &'a ArrayDimension<'a>) {
        walk_array_dimension(self, it);
    }

    #[inline]
    fn visit_array_access(&mut self, it: &'a ArrayAccess<'a>) {
        walk_array_access(self, it);
    }

    #[inline]
    fn visit_update_expression(&mut self, it: &'a UpdateExpression<'a>) {
        walk_update_expression(self, it);
    }

    #[inline]
    fn visit_unary_expression(&mut self, it: &'a UnaryExpression<'a>) {
        walk_unary_expression(self, it);
    }

    #[inline]
    fn visit_cast_expression(&mut self, it: &'a CastExpression<'a>) {
        walk_cast_expression(self, it);
    }

    #[inline]
    fn visit_instanceof_expression(&mut self, it: &'a InstanceofExpression<'a>) {
        walk_instanceof_expression(self, it);
    }

    #[inline]
    fn visit_instanceof_rhs(&mut self, it: &'a InstanceofRhs<'a>) {
        walk_instanceof_rhs(self, it);
    }

    #[inline]
    fn visit_pattern(&mut self, it: &'a Pattern<'a>) {
        walk_pattern(self, it);
    }

    #[inline]
    fn visit_type_pattern(&mut self, it: &'a TypePattern<'a>) {
        walk_type_pattern(self, it);
    }

    #[inline]
    fn visit_record_pattern(&mut self, it: &'a RecordPattern<'a>) {
        walk_record_pattern(self, it);
    }

    #[inline]
    fn visit_component_pattern(&mut self, it: &'a ComponentPattern<'a>) {
        walk_component_pattern(self, it);
    }

    #[inline]
    fn visit_match_all_pattern(&mut self, it: &'a MatchAllPattern) {
        walk_match_all_pattern(self, it);
    }

    #[inline]
    fn visit_binary_expression(&mut self, it: &'a BinaryExpression<'a>) {
        walk_binary_expression(self, it);
    }

    #[inline]
    fn visit_assignment_expression(&mut self, it: &'a AssignmentExpression<'a>) {
        walk_assignment_expression(self, it);
    }

    #[inline]
    fn visit_assignment_target(&mut self, it: &'a AssignmentTarget<'a>) {
        walk_assignment_target(self, it);
    }

    #[inline]
    fn visit_conditional_expression(&mut self, it: &'a ConditionalExpression<'a>) {
        walk_conditional_expression(self, it);
    }

    #[inline]
    fn visit_switch_expression(&mut self, it: &'a SwitchExpression<'a>) {
        walk_switch_expression(self, it);
    }

    #[inline]
    fn visit_parenthesized_expression(&mut self, it: &'a ParenthesizedExpression<'a>) {
        walk_parenthesized_expression(self, it);
    }

    #[inline]
    fn visit_lambda_expression(&mut self, it: &'a LambdaExpression<'a>) {
        walk_lambda_expression(self, it);
    }

    #[inline]
    fn visit_lambda_body(&mut self, it: &'a LambdaBody<'a>) {
        walk_lambda_body(self, it);
    }

    #[inline]
    fn visit_method_reference(&mut self, it: &'a MethodReference<'a>) {
        walk_method_reference(self, it);
    }

    #[inline]
    fn visit_method_reference_target(&mut self, it: &'a MethodReferenceTarget<'a>) {
        walk_method_reference_target(self, it);
    }

    #[inline]
    fn visit_type_parameters(&mut self, it: &'a TypeParameters<'a>) {
        walk_type_parameters(self, it);
    }

    #[inline]
    fn visit_type_parameter(&mut self, it: &'a TypeParameter<'a>) {
        walk_type_parameter(self, it);
    }

    #[inline]
    fn visit_variable_type(&mut self, it: &'a VariableType<'a>) {
        walk_variable_type(self, it);
    }

    #[inline]
    fn visit_var_type(&mut self, it: &'a VarType) {
        walk_var_type(self, it);
    }

    #[inline]
    fn visit_type(&mut self, it: &'a Type<'a>) {
        walk_type(self, it);
    }

    #[inline]
    fn visit_method_result(&mut self, it: &'a MethodResult<'a>) {
        walk_method_result(self, it);
    }

    #[inline]
    fn visit_void_type(&mut self, it: &'a VoidType) {
        walk_void_type(self, it);
    }

    #[inline]
    fn visit_primitive_type(&mut self, it: &'a PrimitiveType<'a>) {
        walk_primitive_type(self, it);
    }

    #[inline]
    fn visit_class_type(&mut self, it: &'a ClassType<'a>) {
        walk_class_type(self, it);
    }

    #[inline]
    fn visit_simple_class_type(&mut self, it: &'a SimpleClassType<'a>) {
        walk_simple_class_type(self, it);
    }

    #[inline]
    fn visit_qualified_class_type(&mut self, it: &'a QualifiedClassType<'a>) {
        walk_qualified_class_type(self, it);
    }

    #[inline]
    fn visit_array_type(&mut self, it: &'a ArrayType<'a>) {
        walk_array_type(self, it);
    }

    #[inline]
    fn visit_array_type_dimension(&mut self, it: &'a ArrayTypeDimension<'a>) {
        walk_array_type_dimension(self, it);
    }

    #[inline]
    fn visit_type_arguments(&mut self, it: &'a TypeArguments<'a>) {
        walk_type_arguments(self, it);
    }

    #[inline]
    fn visit_type_argument(&mut self, it: &'a TypeArgument<'a>) {
        walk_type_argument(self, it);
    }

    #[inline]
    fn visit_wildcard_type_argument(&mut self, it: &'a WildcardTypeArgument<'a>) {
        walk_wildcard_type_argument(self, it);
    }

    #[inline]
    fn visit_wildcard_bound(&mut self, it: &'a WildcardBound<'a>) {
        walk_wildcard_bound(self, it);
    }

    #[inline]
    fn visit_int_literal(&mut self, it: &'a IntLiteral<'a>) {
        walk_int_literal(self, it);
    }

    #[inline]
    fn visit_float_literal(&mut self, it: &'a FloatLiteral<'a>) {
        walk_float_literal(self, it);
    }

    #[inline]
    fn visit_char_literal(&mut self, it: &'a CharLiteral<'a>) {
        walk_char_literal(self, it);
    }

    #[inline]
    fn visit_string_literal(&mut self, it: &'a StringLiteral<'a>) {
        walk_string_literal(self, it);
    }

    #[inline]
    fn visit_boolean_literal(&mut self, it: &'a BooleanLiteral) {
        walk_boolean_literal(self, it);
    }

    #[inline]
    fn visit_null_literal(&mut self, it: &'a NullLiteral) {
        walk_null_literal(self, it);
    }

    #[inline]
    fn visit_identifier_name(&mut self, it: &'a IdentifierName<'a>) {
        walk_identifier_name(self, it);
    }

    #[inline]
    fn visit_identifier_reference(&mut self, it: &'a IdentifierReference<'a>) {
        walk_identifier_reference(self, it);
    }

    #[inline]
    fn visit_binding_identifier(&mut self, it: &'a BindingIdentifier<'a>) {
        walk_binding_identifier(self, it);
    }

    #[inline]
    fn visit_label_identifier(&mut self, it: &'a LabelIdentifier<'a>) {
        walk_label_identifier(self, it);
    }

    #[inline]
    fn visit_name(&mut self, it: &'a Name<'a>) {
        walk_name(self, it);
    }

    #[inline]
    fn visit_qualified_name(&mut self, it: &'a QualifiedName<'a>) {
        walk_qualified_name(self, it);
    }

    #[inline]
    fn visit_type_name(&mut self, it: &'a TypeName<'a>) {
        walk_type_name(self, it);
    }

    #[inline]
    fn visit_qualified_type_name(&mut self, it: &'a QualifiedTypeName<'a>) {
        walk_qualified_type_name(self, it);
    }

    #[inline]
    fn visit_modifier(&mut self, it: &'a Modifier<'a>) {
        walk_modifier(self, it);
    }

    #[inline]
    fn visit_keyword_modifier(&mut self, it: &'a KeywordModifier) {
        walk_keyword_modifier(self, it);
    }

    #[inline]
    fn visit_annotation(&mut self, it: &'a Annotation<'a>) {
        walk_annotation(self, it);
    }

    #[inline]
    fn visit_annotation_argument(&mut self, it: &'a AnnotationArgument<'a>) {
        walk_annotation_argument(self, it);
    }

    #[inline]
    fn visit_element_value_pair(&mut self, it: &'a ElementValuePair<'a>) {
        walk_element_value_pair(self, it);
    }

    #[inline]
    fn visit_element_value(&mut self, it: &'a ElementValue<'a>) {
        walk_element_value(self, it);
    }

    #[inline]
    fn visit_element_value_array_initializer(&mut self, it: &'a ElementValueArrayInitializer<'a>) {
        walk_element_value_array_initializer(self, it);
    }

    #[inline]
    fn visit_import_declarations(&mut self, it: &'a Vec<'a, ImportDeclaration<'a>>) {
        walk_import_declarations(self, it);
    }

    #[inline]
    fn visit_type_declarations(&mut self, it: &'a Vec<'a, TypeDeclaration<'a>>) {
        walk_type_declarations(self, it);
    }

    #[inline]
    fn visit_annotations(&mut self, it: &'a Vec<'a, Annotation<'a>>) {
        walk_annotations(self, it);
    }

    #[inline]
    fn visit_class_members(&mut self, it: &'a Vec<'a, ClassMember<'a>>) {
        walk_class_members(self, it);
    }

    #[inline]
    fn visit_module_directives(&mut self, it: &'a Vec<'a, ModuleDirective<'a>>) {
        walk_module_directives(self, it);
    }

    #[inline]
    fn visit_keyword_modifiers(&mut self, it: &'a Vec<'a, KeywordModifier>) {
        walk_keyword_modifiers(self, it);
    }

    #[inline]
    fn visit_names(&mut self, it: &'a Vec<'a, Name<'a>>) {
        walk_names(self, it);
    }

    #[inline]
    fn visit_type_names(&mut self, it: &'a Vec<'a, TypeName<'a>>) {
        walk_type_names(self, it);
    }

    #[inline]
    fn visit_modifiers(&mut self, it: &'a Vec<'a, Modifier<'a>>) {
        walk_modifiers(self, it);
    }

    #[inline]
    fn visit_types(&mut self, it: &'a Vec<'a, Type<'a>>) {
        walk_types(self, it);
    }

    #[inline]
    fn visit_interface_members(&mut self, it: &'a Vec<'a, InterfaceMember<'a>>) {
        walk_interface_members(self, it);
    }

    #[inline]
    fn visit_enum_constants(&mut self, it: &'a Vec<'a, EnumConstant<'a>>) {
        walk_enum_constants(self, it);
    }

    #[inline]
    fn visit_expressions(&mut self, it: &'a Vec<'a, Expression<'a>>) {
        walk_expressions(self, it);
    }

    #[inline]
    fn visit_record_components(&mut self, it: &'a Vec<'a, RecordComponent<'a>>) {
        walk_record_components(self, it);
    }

    #[inline]
    fn visit_annotation_interface_members(
        &mut self,
        it: &'a Vec<'a, AnnotationInterfaceMember<'a>>,
    ) {
        walk_annotation_interface_members(self, it);
    }

    #[inline]
    fn visit_array_type_dimensions(&mut self, it: &'a Vec<'a, ArrayTypeDimension<'a>>) {
        walk_array_type_dimensions(self, it);
    }

    #[inline]
    fn visit_variable_declarators(&mut self, it: &'a Vec<'a, VariableDeclarator<'a>>) {
        walk_variable_declarators(self, it);
    }

    #[inline]
    fn visit_formal_parameter_list(&mut self, it: &'a Vec<'a, FormalParameter<'a>>) {
        walk_formal_parameter_list(self, it);
    }

    #[inline]
    fn visit_statements(&mut self, it: &'a Vec<'a, Statement<'a>>) {
        walk_statements(self, it);
    }

    #[inline]
    fn visit_switch_items(&mut self, it: &'a Vec<'a, SwitchItem<'a>>) {
        walk_switch_items(self, it);
    }

    #[inline]
    fn visit_patterns(&mut self, it: &'a Vec<'a, Pattern<'a>>) {
        walk_patterns(self, it);
    }

    #[inline]
    fn visit_resources(&mut self, it: &'a Vec<'a, Resource<'a>>) {
        walk_resources(self, it);
    }

    #[inline]
    fn visit_catch_clauses(&mut self, it: &'a Vec<'a, CatchClause<'a>>) {
        walk_catch_clauses(self, it);
    }

    #[inline]
    fn visit_class_types(&mut self, it: &'a Vec<'a, ClassType<'a>>) {
        walk_class_types(self, it);
    }

    #[inline]
    fn visit_variable_initializers(&mut self, it: &'a Vec<'a, VariableInitializer<'a>>) {
        walk_variable_initializers(self, it);
    }

    #[inline]
    fn visit_array_dimensions(&mut self, it: &'a Vec<'a, ArrayDimension<'a>>) {
        walk_array_dimensions(self, it);
    }

    #[inline]
    fn visit_component_patterns(&mut self, it: &'a Vec<'a, ComponentPattern<'a>>) {
        walk_component_patterns(self, it);
    }

    #[inline]
    fn visit_type_parameter_list(&mut self, it: &'a Vec<'a, TypeParameter<'a>>) {
        walk_type_parameter_list(self, it);
    }

    #[inline]
    fn visit_type_argument_list(&mut self, it: &'a Vec<'a, TypeArgument<'a>>) {
        walk_type_argument_list(self, it);
    }

    #[inline]
    fn visit_annotation_arguments(&mut self, it: &'a Vec<'a, AnnotationArgument<'a>>) {
        walk_annotation_arguments(self, it);
    }

    #[inline]
    fn visit_element_values(&mut self, it: &'a Vec<'a, ElementValue<'a>>) {
        walk_element_values(self, it);
    }
}

pub mod walk {
    use super::*;

    #[inline]
    pub fn walk_compilation_unit<'a, V: Visit<'a>>(visitor: &mut V, it: &'a CompilationUnit<'a>) {
        let kind = AstKind::CompilationUnit(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_compilation_unit_content(&it.content);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_compilation_unit_content<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a CompilationUnitContent<'a>,
    ) {
        match it {
            CompilationUnitContent::Ordinary(it) => visitor.visit_ordinary_compilation_unit(it),
            CompilationUnitContent::Compact(it) => visitor.visit_compact_compilation_unit(it),
            CompilationUnitContent::Modular(it) => visitor.visit_modular_compilation_unit(it),
        }
    }

    #[inline]
    pub fn walk_ordinary_compilation_unit<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a OrdinaryCompilationUnit<'a>,
    ) {
        let kind = AstKind::OrdinaryCompilationUnit(it);
        visitor.enter_node(kind);
        if let Some(package) = &it.package {
            visitor.visit_package_declaration(package);
        }
        visitor.visit_import_declarations(&it.imports);
        visitor.visit_type_declarations(&it.types);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_modular_compilation_unit<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ModularCompilationUnit<'a>,
    ) {
        let kind = AstKind::ModularCompilationUnit(it);
        visitor.enter_node(kind);
        visitor.visit_import_declarations(&it.imports);
        visitor.visit_module_declaration(&it.module);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_package_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a PackageDeclaration<'a>,
    ) {
        let kind = AstKind::PackageDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_annotations(&it.annotations);
        visitor.visit_name(&it.name);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_import_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ImportDeclaration<'a>,
    ) {
        let kind = AstKind::ImportDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_name(&it.name);
        if let Some(local) = &it.local {
            visitor.visit_binding_identifier(local);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_compact_compilation_unit<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a CompactCompilationUnit<'a>,
    ) {
        let kind = AstKind::CompactCompilationUnit(it);
        visitor.enter_node(kind);
        visitor.visit_import_declarations(&it.imports);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_class_members(&it.members);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_module_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ModuleDeclaration<'a>,
    ) {
        let kind = AstKind::ModuleDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_annotations(&it.annotations);
        visitor.visit_name(&it.name);
        visitor.visit_module_directives(&it.directives);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_module_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ModuleDirective<'a>) {
        match it {
            ModuleDirective::Requires(it) => visitor.visit_requires_directive(it),
            ModuleDirective::Package(it) => visitor.visit_package_directive(it),
            ModuleDirective::Uses(it) => visitor.visit_uses_directive(it),
            ModuleDirective::Provides(it) => visitor.visit_provides_directive(it),
        }
    }

    #[inline]
    pub fn walk_requires_directive<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a RequiresDirective<'a>,
    ) {
        let kind = AstKind::RequiresDirective(it);
        visitor.enter_node(kind);
        visitor.visit_keyword_modifiers(&it.modifiers);
        visitor.visit_name(&it.name);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_package_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &'a PackageDirective<'a>) {
        let kind = AstKind::PackageDirective(it);
        visitor.enter_node(kind);
        visitor.visit_name(&it.name);
        visitor.visit_names(&it.targets);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_uses_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &'a UsesDirective<'a>) {
        let kind = AstKind::UsesDirective(it);
        visitor.enter_node(kind);
        visitor.visit_type_name(&it.name);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_provides_directive<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ProvidesDirective<'a>,
    ) {
        let kind = AstKind::ProvidesDirective(it);
        visitor.enter_node(kind);
        visitor.visit_type_name(&it.name);
        visitor.visit_type_names(&it.with);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_type_declaration<'a, V: Visit<'a>>(visitor: &mut V, it: &'a TypeDeclaration<'a>) {
        match it {
            TypeDeclaration::Empty(it) => visitor.visit_empty_statement(it),
            TypeDeclaration::AnnotationInterface(it) => {
                visitor.visit_annotation_interface_declaration(it)
            }
            match_local_type_declaration!(TypeDeclaration) => {
                visitor.visit_local_type_declaration(it.to_local_type_declaration())
            }
        }
    }

    #[inline]
    pub fn walk_local_type_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a LocalTypeDeclaration<'a>,
    ) {
        match it {
            LocalTypeDeclaration::Class(it) => visitor.visit_class_declaration(it),
            LocalTypeDeclaration::Interface(it) => visitor.visit_interface_declaration(it),
            LocalTypeDeclaration::Enum(it) => visitor.visit_enum_declaration(it),
            LocalTypeDeclaration::Record(it) => visitor.visit_record_declaration(it),
        }
    }

    pub fn walk_class_declaration<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ClassDeclaration<'a>) {
        let kind = AstKind::ClassDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_binding_identifier(&it.name);
        visitor.enter_scope(&it.scope_id);
        if let Some(type_params) = &it.type_params {
            visitor.visit_type_parameters(type_params);
        }
        if let Some(superclass) = &it.superclass {
            visitor.visit_type(superclass);
        }
        visitor.visit_types(&it.interfaces);
        visitor.visit_type_names(&it.permits);
        visitor.visit_class_body(&it.body);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_class_body<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ClassBody<'a>) {
        let kind = AstKind::ClassBody(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_class_members(&it.members);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    pub fn walk_interface_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a InterfaceDeclaration<'a>,
    ) {
        let kind = AstKind::InterfaceDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_binding_identifier(&it.name);
        visitor.enter_scope(&it.scope_id);
        if let Some(type_params) = &it.type_params {
            visitor.visit_type_parameters(type_params);
        }
        visitor.visit_types(&it.extends);
        visitor.visit_type_names(&it.permits);
        visitor.visit_interface_body(&it.body);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_interface_body<'a, V: Visit<'a>>(visitor: &mut V, it: &'a InterfaceBody<'a>) {
        let kind = AstKind::InterfaceBody(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_interface_members(&it.members);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_enum_declaration<'a, V: Visit<'a>>(visitor: &mut V, it: &'a EnumDeclaration<'a>) {
        let kind = AstKind::EnumDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_binding_identifier(&it.name);
        visitor.visit_types(&it.interfaces);
        visitor.visit_enum_body(&it.body);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_enum_body<'a, V: Visit<'a>>(visitor: &mut V, it: &'a EnumBody<'a>) {
        let kind = AstKind::EnumBody(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_enum_constants(&it.constants);
        visitor.visit_class_members(&it.members);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_enum_constant<'a, V: Visit<'a>>(visitor: &mut V, it: &'a EnumConstant<'a>) {
        let kind = AstKind::EnumConstant(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_binding_identifier(&it.name);
        if let Some(args) = &it.args {
            visitor.visit_expressions(args);
        }
        if let Some(body) = &it.body {
            visitor.visit_class_body(body);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_record_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a RecordDeclaration<'a>,
    ) {
        let kind = AstKind::RecordDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_binding_identifier(&it.name);
        visitor.enter_scope(&it.scope_id);
        if let Some(type_params) = &it.type_params {
            visitor.visit_type_parameters(type_params);
        }
        visitor.visit_record_header(&it.header);
        visitor.visit_types(&it.interfaces);
        visitor.visit_class_body(&it.body);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_record_header<'a, V: Visit<'a>>(visitor: &mut V, it: &'a RecordHeader<'a>) {
        let kind = AstKind::RecordHeader(it);
        visitor.enter_node(kind);
        visitor.visit_record_components(&it.components);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_record_component<'a, V: Visit<'a>>(visitor: &mut V, it: &'a RecordComponent<'a>) {
        let kind = AstKind::RecordComponent(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_type(&it.ty);
        visitor.visit_annotations(&it.varargs_annotations);
        visitor.visit_binding_identifier(&it.name);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_annotation_interface_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a AnnotationInterfaceDeclaration<'a>,
    ) {
        let kind = AstKind::AnnotationInterfaceDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_binding_identifier(&it.name);
        visitor.visit_annotation_interface_body(&it.body);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_annotation_interface_body<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a AnnotationInterfaceBody<'a>,
    ) {
        let kind = AstKind::AnnotationInterfaceBody(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_annotation_interface_members(&it.members);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_annotation_interface_member<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a AnnotationInterfaceMember<'a>,
    ) {
        match it {
            AnnotationInterfaceMember::Element(it) => {
                visitor.visit_annotation_interface_element_declaration(it)
            }
            AnnotationInterfaceMember::Field(it) => visitor.visit_field_declaration(it),
            match_type_declaration!(AnnotationInterfaceMember) => {
                visitor.visit_type_declaration(it.to_type_declaration())
            }
        }
    }

    #[inline]
    pub fn walk_annotation_interface_element_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a AnnotationInterfaceElementDeclaration<'a>,
    ) {
        let kind = AstKind::AnnotationInterfaceElementDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_type(&it.ty);
        visitor.visit_binding_identifier(&it.name);
        visitor.visit_array_type_dimensions(&it.post_paren_dimensions);
        if let Some(default_value) = &it.default_value {
            visitor.visit_element_value(default_value);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_class_member<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ClassMember<'a>) {
        match it {
            ClassMember::Field(it) => visitor.visit_field_declaration(it),
            ClassMember::Method(it) => visitor.visit_method_declaration(it),
            ClassMember::Constructor(it) => visitor.visit_constructor_declaration(it),
            ClassMember::CompactConstructor(it) => {
                visitor.visit_compact_constructor_declaration(it)
            }
            ClassMember::Initializer(it) => visitor.visit_initializer_block(it),
            match_type_declaration!(ClassMember) => {
                visitor.visit_type_declaration(it.to_type_declaration())
            }
        }
    }

    #[inline]
    pub fn walk_interface_member<'a, V: Visit<'a>>(visitor: &mut V, it: &'a InterfaceMember<'a>) {
        match it {
            InterfaceMember::Constant(it) => visitor.visit_field_declaration(it),
            InterfaceMember::Method(it) => visitor.visit_method_declaration(it),
            match_type_declaration!(InterfaceMember) => {
                visitor.visit_type_declaration(it.to_type_declaration())
            }
        }
    }

    #[inline]
    pub fn walk_initializer_block<'a, V: Visit<'a>>(visitor: &mut V, it: &'a InitializerBlock<'a>) {
        let kind = AstKind::InitializerBlock(it);
        visitor.enter_node(kind);
        visitor.visit_block(&it.block);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_field_declaration<'a, V: Visit<'a>>(visitor: &mut V, it: &'a FieldDeclaration<'a>) {
        let kind = AstKind::FieldDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_type(&it.ty);
        visitor.visit_variable_declarators(&it.declarators);
        visitor.leave_node(kind);
    }

    pub fn walk_method_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a MethodDeclaration<'a>,
    ) {
        let kind = AstKind::MethodDeclaration(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_modifiers(&it.modifiers);
        if let Some(type_params) = &it.type_params {
            visitor.visit_type_parameters(type_params);
        }
        visitor.visit_method_result(&it.return_type);
        visitor.visit_binding_identifier(&it.name);
        visitor.visit_formal_parameters(&it.params);
        visitor.visit_array_type_dimensions(&it.post_paren_dimensions);
        visitor.visit_types(&it.throws);
        if let Some(body) = &it.body {
            visitor.visit_block(body);
        }
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    pub fn walk_constructor_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ConstructorDeclaration<'a>,
    ) {
        let kind = AstKind::ConstructorDeclaration(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_modifiers(&it.modifiers);
        if let Some(type_params) = &it.type_params {
            visitor.visit_type_parameters(type_params);
        }
        visitor.visit_identifier_name(&it.name);
        visitor.visit_formal_parameters(&it.params);
        visitor.visit_types(&it.throws);
        visitor.visit_block(&it.body);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_compact_constructor_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a CompactConstructorDeclaration<'a>,
    ) {
        let kind = AstKind::CompactConstructorDeclaration(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_identifier_name(&it.name);
        visitor.visit_block(&it.body);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_receiver_parameter<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ReceiverParameter<'a>,
    ) {
        let kind = AstKind::ReceiverParameter(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_type(&it.ty);
        if let Some(qualifier) = &it.qualifier {
            visitor.visit_identifier_name(qualifier);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_formal_parameters<'a, V: Visit<'a>>(visitor: &mut V, it: &'a FormalParameters<'a>) {
        let kind = AstKind::FormalParameters(it);
        visitor.enter_node(kind);
        if let Some(receiver) = &it.receiver {
            visitor.visit_receiver_parameter(receiver);
        }
        visitor.visit_formal_parameter_list(&it.items);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_formal_parameter<'a, V: Visit<'a>>(visitor: &mut V, it: &'a FormalParameter<'a>) {
        let kind = AstKind::FormalParameter(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        if let Some(ty) = &it.ty {
            visitor.visit_variable_type(ty);
        }
        visitor.visit_annotations(&it.varargs_annotations);
        visitor.visit_binding_identifier(&it.name);
        visitor.visit_array_type_dimensions(&it.dims);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_block<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Block<'a>) {
        let kind = AstKind::Block(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_statements(&it.statements);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    pub fn walk_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Statement<'a>) {
        match it {
            Statement::Block(it) => visitor.visit_block(it),
            Statement::Empty(it) => visitor.visit_empty_statement(it),
            Statement::ConstructorInvocation(it) => visitor.visit_constructor_invocation(it),
            Statement::LocalVariable(it) => visitor.visit_local_variable_declaration(it),
            Statement::Expression(it) => visitor.visit_expression_statement(it),
            Statement::If(it) => visitor.visit_if_statement(it),
            Statement::Switch(it) => visitor.visit_switch_statement(it),
            Statement::While(it) => visitor.visit_while_statement(it),
            Statement::Do(it) => visitor.visit_do_statement(it),
            Statement::For(it) => visitor.visit_for_statement(it),
            Statement::EnhancedFor(it) => visitor.visit_enhanced_for_statement(it),
            Statement::Break(it) => visitor.visit_break_statement(it),
            Statement::Continue(it) => visitor.visit_continue_statement(it),
            Statement::Throw(it) => visitor.visit_throw_statement(it),
            Statement::Assert(it) => visitor.visit_assert_statement(it),
            Statement::Try(it) => visitor.visit_try_statement(it),
            Statement::Synchronized(it) => visitor.visit_synchronized_statement(it),
            Statement::Return(it) => visitor.visit_return_statement(it),
            Statement::Yield(it) => visitor.visit_yield_statement(it),
            Statement::Labeled(it) => visitor.visit_labeled_statement(it),
            match_local_type_declaration!(Statement) => {
                visitor.visit_local_type_declaration(it.to_local_type_declaration())
            }
        }
    }

    #[inline]
    pub fn walk_empty_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a EmptyStatement) {
        let kind = AstKind::EmptyStatement(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_if_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a IfStatement<'a>) {
        let kind = AstKind::IfStatement(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.condition);
        visitor.visit_statement(&it.then_branch);
        if let Some(else_branch) = &it.else_branch {
            visitor.visit_statement(else_branch);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_switch_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a SwitchStatement<'a>) {
        let kind = AstKind::SwitchStatement(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.selector);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_switch_items(&it.cases);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_switch_item<'a, V: Visit<'a>>(visitor: &mut V, it: &'a SwitchItem<'a>) {
        match it {
            SwitchItem::Rule(it) => visitor.visit_switch_rule(it),
            SwitchItem::Case(it) => visitor.visit_switch_case(it),
        }
    }

    #[inline]
    pub fn walk_switch_rule<'a, V: Visit<'a>>(visitor: &mut V, it: &'a SwitchRule<'a>) {
        let kind = AstKind::SwitchRule(it);
        visitor.enter_node(kind);
        if let Some(label) = &it.label {
            visitor.visit_switch_label(label);
        }
        visitor.visit_switch_rule_body(&it.body);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_switch_rule_body<'a, V: Visit<'a>>(visitor: &mut V, it: &'a SwitchRuleBody<'a>) {
        match it {
            SwitchRuleBody::Block(it) => visitor.visit_block(it),
            SwitchRuleBody::Throw(it) => visitor.visit_throw_statement(it),
            match_expression!(SwitchRuleBody) => visitor.visit_expression(it.to_expression()),
        }
    }

    #[inline]
    pub fn walk_switch_case<'a, V: Visit<'a>>(visitor: &mut V, it: &'a SwitchCase<'a>) {
        let kind = AstKind::SwitchCase(it);
        visitor.enter_node(kind);
        if let Some(label) = &it.label {
            visitor.visit_switch_label(label);
        }
        visitor.visit_statements(&it.statements);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_switch_label<'a, V: Visit<'a>>(visitor: &mut V, it: &'a SwitchLabel<'a>) {
        match it {
            SwitchLabel::Case(it) => visitor.visit_case_label(it),
            SwitchLabel::Pattern(it) => visitor.visit_case_pattern_label(it),
        }
    }

    #[inline]
    pub fn walk_case_label<'a, V: Visit<'a>>(visitor: &mut V, it: &'a CaseLabel<'a>) {
        let kind = AstKind::CaseLabel(it);
        visitor.enter_node(kind);
        visitor.visit_expressions(&it.constants);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_case_pattern_label<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a CasePatternLabel<'a>,
    ) {
        let kind = AstKind::CasePatternLabel(it);
        visitor.enter_node(kind);
        visitor.visit_patterns(&it.patterns);
        if let Some(guard) = &it.guard {
            visitor.visit_expression(guard);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_while_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a WhileStatement<'a>) {
        let kind = AstKind::WhileStatement(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.condition);
        visitor.visit_statement(&it.body);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_do_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a DoStatement<'a>) {
        let kind = AstKind::DoStatement(it);
        visitor.enter_node(kind);
        visitor.visit_statement(&it.body);
        visitor.visit_expression(&it.condition);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_for_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ForStatement<'a>) {
        let kind = AstKind::ForStatement(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        if let Some(init) = &it.init {
            visitor.visit_for_init(init);
        }
        if let Some(condition) = &it.condition {
            visitor.visit_expression(condition);
        }
        if let Some(update) = &it.update {
            visitor.visit_statement_expression_list(update);
        }
        visitor.visit_statement(&it.body);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_for_init<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ForInit<'a>) {
        match it {
            ForInit::Declaration(it) => visitor.visit_local_variable_declaration(it),
            ForInit::Expressions(it) => visitor.visit_statement_expression_list(it),
        }
    }

    #[inline]
    pub fn walk_statement_expression_list<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a StatementExpressionList<'a>,
    ) {
        let kind = AstKind::StatementExpressionList(it);
        visitor.enter_node(kind);
        visitor.visit_expressions(&it.expressions);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_enhanced_for_statement<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a EnhancedForStatement<'a>,
    ) {
        let kind = AstKind::EnhancedForStatement(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_local_variable_declaration(&it.declaration);
        visitor.visit_expression(&it.iterable);
        visitor.visit_statement(&it.body);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_break_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a BreakStatement<'a>) {
        let kind = AstKind::BreakStatement(it);
        visitor.enter_node(kind);
        if let Some(label) = &it.label {
            visitor.visit_label_identifier(label);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_continue_statement<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ContinueStatement<'a>,
    ) {
        let kind = AstKind::ContinueStatement(it);
        visitor.enter_node(kind);
        if let Some(label) = &it.label {
            visitor.visit_label_identifier(label);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_throw_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ThrowStatement<'a>) {
        let kind = AstKind::ThrowStatement(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.value);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_assert_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a AssertStatement<'a>) {
        let kind = AstKind::AssertStatement(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.condition);
        if let Some(message) = &it.message {
            visitor.visit_expression(message);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_try_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a TryStatement<'a>) {
        let kind = AstKind::TryStatement(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_resources(&it.resources);
        visitor.visit_block(&it.block);
        visitor.leave_scope();
        visitor.visit_catch_clauses(&it.catches);
        if let Some(finally_block) = &it.finally_block {
            visitor.visit_block(finally_block);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_resource<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Resource<'a>) {
        match it {
            Resource::Declaration(it) => visitor.visit_local_variable_declaration(it),
            match_resource_access!(Resource) => {
                visitor.visit_resource_access(it.to_resource_access())
            }
        }
    }

    #[inline]
    pub fn walk_resource_access<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ResourceAccess<'a>) {
        match it {
            ResourceAccess::Identifier(it) => visitor.visit_identifier_reference(it),
            ResourceAccess::Field(it) => visitor.visit_field_access(it),
        }
    }

    #[inline]
    pub fn walk_catch_clause<'a, V: Visit<'a>>(visitor: &mut V, it: &'a CatchClause<'a>) {
        let kind = AstKind::CatchClause(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_catch_parameter(&it.param);
        visitor.visit_block(&it.block);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_catch_parameter<'a, V: Visit<'a>>(visitor: &mut V, it: &'a CatchParameter<'a>) {
        let kind = AstKind::CatchParameter(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_catch_type(&it.ty);
        visitor.visit_binding_identifier(&it.name);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_catch_type<'a, V: Visit<'a>>(visitor: &mut V, it: &'a CatchType<'a>) {
        match it {
            CatchType::Union(it) => visitor.visit_union_type(it),
            match_class_type!(CatchType) => visitor.visit_class_type(it.to_class_type()),
        }
    }

    #[inline]
    pub fn walk_union_type<'a, V: Visit<'a>>(visitor: &mut V, it: &'a UnionType<'a>) {
        let kind = AstKind::UnionType(it);
        visitor.enter_node(kind);
        visitor.visit_class_types(&it.types);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_synchronized_statement<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a SynchronizedStatement<'a>,
    ) {
        let kind = AstKind::SynchronizedStatement(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.lock);
        visitor.visit_block(&it.body);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_labeled_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a LabeledStatement<'a>) {
        let kind = AstKind::LabeledStatement(it);
        visitor.enter_node(kind);
        visitor.visit_label_identifier(&it.label);
        visitor.visit_statement(&it.statement);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_constructor_invocation<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ConstructorInvocation<'a>,
    ) {
        let kind = AstKind::ConstructorInvocation(it);
        visitor.enter_node(kind);
        if let Some(qualifier) = &it.qualifier {
            visitor.visit_expression(qualifier);
        }
        if let Some(type_args) = &it.type_args {
            visitor.visit_type_arguments(type_args);
        }
        visitor.visit_expressions(&it.args);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_expression_statement<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ExpressionStatement<'a>,
    ) {
        let kind = AstKind::ExpressionStatement(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.expression);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_local_variable_declaration<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a LocalVariableDeclaration<'a>,
    ) {
        let kind = AstKind::LocalVariableDeclaration(it);
        visitor.enter_node(kind);
        visitor.visit_modifiers(&it.modifiers);
        visitor.visit_variable_type(&it.ty);
        visitor.visit_variable_declarators(&it.declarators);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_return_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ReturnStatement<'a>) {
        let kind = AstKind::ReturnStatement(it);
        visitor.enter_node(kind);
        if let Some(value) = &it.value {
            visitor.visit_expression(value);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_yield_statement<'a, V: Visit<'a>>(visitor: &mut V, it: &'a YieldStatement<'a>) {
        let kind = AstKind::YieldStatement(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.value);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_variable_declarator<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a VariableDeclarator<'a>,
    ) {
        let kind = AstKind::VariableDeclarator(it);
        visitor.enter_node(kind);
        visitor.visit_binding_identifier(&it.name);
        visitor.visit_array_type_dimensions(&it.dims);
        if let Some(init) = &it.init {
            visitor.visit_variable_initializer(init);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_variable_initializer<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a VariableInitializer<'a>,
    ) {
        match it {
            VariableInitializer::Array(it) => visitor.visit_array_initializer(it),
            match_expression!(VariableInitializer) => visitor.visit_expression(it.to_expression()),
        }
    }

    #[inline]
    pub fn walk_array_initializer<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ArrayInitializer<'a>) {
        let kind = AstKind::ArrayInitializer(it);
        visitor.enter_node(kind);
        visitor.visit_variable_initializers(&it.elements);
        visitor.leave_node(kind);
    }

    pub fn walk_expression<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Expression<'a>) {
        match it {
            Expression::IntLiteral(it) => visitor.visit_int_literal(it),
            Expression::FloatLiteral(it) => visitor.visit_float_literal(it),
            Expression::CharLiteral(it) => visitor.visit_char_literal(it),
            Expression::StringLiteral(it) => visitor.visit_string_literal(it),
            Expression::BooleanLiteral(it) => visitor.visit_boolean_literal(it),
            Expression::NullLiteral(it) => visitor.visit_null_literal(it),
            Expression::This(it) => visitor.visit_this_expression(it),
            Expression::Super(it) => visitor.visit_super(it),
            Expression::QualifiedThis(it) => visitor.visit_qualified_this(it),
            Expression::QualifiedSuper(it) => visitor.visit_qualified_super(it),
            Expression::Identifier(it) => visitor.visit_identifier_reference(it),
            Expression::FieldAccess(it) => visitor.visit_field_access(it),
            Expression::MethodCall(it) => visitor.visit_method_call(it),
            Expression::ClassLiteral(it) => visitor.visit_class_literal(it),
            Expression::ClassInstanceCreation(it) => visitor.visit_class_instance_creation(it),
            Expression::QualifiedClassInstanceCreation(it) => {
                visitor.visit_qualified_class_instance_creation(it)
            }
            Expression::ArrayCreation(it) => visitor.visit_array_creation(it),
            Expression::ArrayAccess(it) => visitor.visit_array_access(it),
            Expression::Update(it) => visitor.visit_update_expression(it),
            Expression::Unary(it) => visitor.visit_unary_expression(it),
            Expression::Cast(it) => visitor.visit_cast_expression(it),
            Expression::Instanceof(it) => visitor.visit_instanceof_expression(it),
            Expression::Binary(it) => visitor.visit_binary_expression(it),
            Expression::Assignment(it) => visitor.visit_assignment_expression(it),
            Expression::Conditional(it) => visitor.visit_conditional_expression(it),
            Expression::Switch(it) => visitor.visit_switch_expression(it),
            Expression::Parenthesized(it) => visitor.visit_parenthesized_expression(it),
            Expression::Lambda(it) => visitor.visit_lambda_expression(it),
            Expression::MethodReference(it) => visitor.visit_method_reference(it),
        }
    }

    #[inline]
    pub fn walk_field_access<'a, V: Visit<'a>>(visitor: &mut V, it: &'a FieldAccess<'a>) {
        walk_expression_chain(visitor, ExpressionChain::FieldAccess(it));
    }

    #[inline]
    pub fn walk_method_call<'a, V: Visit<'a>>(visitor: &mut V, it: &'a MethodCall<'a>) {
        walk_expression_chain(visitor, ExpressionChain::MethodCall(it));
    }

    #[inline]
    pub fn walk_callee<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Callee<'a>) {
        match it {
            Callee::Identifier(it) => visitor.visit_identifier_reference(it),
            Callee::Member(it) => visitor.visit_method_member(it),
        }
    }

    #[inline]
    pub fn walk_method_member<'a, V: Visit<'a>>(visitor: &mut V, it: &'a MethodMember<'a>) {
        let kind = AstKind::MethodMember(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.target);
        if let Some(type_args) = &it.type_args {
            visitor.visit_type_arguments(type_args);
        }
        visitor.visit_identifier_name(&it.name);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_class_literal<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ClassLiteral<'a>) {
        let kind = AstKind::ClassLiteral(it);
        visitor.enter_node(kind);
        visitor.visit_method_result(&it.ty);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_this_expression<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ThisExpression) {
        let kind = AstKind::ThisExpression(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_super<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Super) {
        let kind = AstKind::Super(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_qualified_this<'a, V: Visit<'a>>(visitor: &mut V, it: &'a QualifiedThis<'a>) {
        let kind = AstKind::QualifiedThis(it);
        visitor.enter_node(kind);
        visitor.visit_type_name(&it.qualifier);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_qualified_super<'a, V: Visit<'a>>(visitor: &mut V, it: &'a QualifiedSuper<'a>) {
        let kind = AstKind::QualifiedSuper(it);
        visitor.enter_node(kind);
        visitor.visit_type_name(&it.qualifier);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_class_instance_creation<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ClassInstanceCreation<'a>,
    ) {
        let kind = AstKind::ClassInstanceCreation(it);
        visitor.enter_node(kind);
        if let Some(constructor_type_args) = &it.constructor_type_args {
            visitor.visit_type_arguments(constructor_type_args);
        }
        visitor.visit_class_type(&it.ty);
        visitor.visit_expressions(&it.args);
        if let Some(body) = &it.body {
            visitor.visit_class_body(body);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_qualified_class_instance_creation<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a QualifiedClassInstanceCreation<'a>,
    ) {
        walk_expression_chain(visitor, ExpressionChain::QualifiedClassInstanceCreation(it));
    }

    #[inline]
    pub fn walk_array_creation<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ArrayCreation<'a>) {
        let kind = AstKind::ArrayCreation(it);
        visitor.enter_node(kind);
        visitor.visit_type(&it.element);
        visitor.visit_array_dimensions(&it.dims);
        if let Some(init) = &it.init {
            visitor.visit_array_initializer(init);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_array_dimension<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ArrayDimension<'a>) {
        let kind = AstKind::ArrayDimension(it);
        visitor.enter_node(kind);
        visitor.visit_annotations(&it.annotations);
        if let Some(length) = &it.length {
            visitor.visit_expression(length);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_array_access<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ArrayAccess<'a>) {
        walk_expression_chain(visitor, ExpressionChain::ArrayAccess(it));
    }

    #[inline]
    pub fn walk_update_expression<'a, V: Visit<'a>>(visitor: &mut V, it: &'a UpdateExpression<'a>) {
        let kind = AstKind::UpdateExpression(it);
        visitor.enter_node(kind);
        visitor.visit_assignment_target(&it.target);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_unary_expression<'a, V: Visit<'a>>(visitor: &mut V, it: &'a UnaryExpression<'a>) {
        let kind = AstKind::UnaryExpression(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.operand);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_cast_expression<'a, V: Visit<'a>>(visitor: &mut V, it: &'a CastExpression<'a>) {
        let kind = AstKind::CastExpression(it);
        visitor.enter_node(kind);
        visitor.visit_type(&it.ty);
        visitor.visit_types(&it.additional_bounds);
        visitor.visit_expression(&it.operand);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_instanceof_expression<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a InstanceofExpression<'a>,
    ) {
        walk_expression_chain(visitor, ExpressionChain::InstanceofExpression(it));
    }

    #[inline]
    pub fn walk_instanceof_rhs<'a, V: Visit<'a>>(visitor: &mut V, it: &'a InstanceofRhs<'a>) {
        match it {
            match_type!(InstanceofRhs) => visitor.visit_type(it.to_type()),
            match_pattern!(InstanceofRhs) => visitor.visit_pattern(it.to_pattern()),
        }
    }

    #[inline]
    pub fn walk_pattern<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Pattern<'a>) {
        match it {
            Pattern::TypePattern(it) => visitor.visit_type_pattern(it),
            Pattern::RecordPattern(it) => visitor.visit_record_pattern(it),
        }
    }

    #[inline]
    pub fn walk_type_pattern<'a, V: Visit<'a>>(visitor: &mut V, it: &'a TypePattern<'a>) {
        let kind = AstKind::TypePattern(it);
        visitor.enter_node(kind);
        visitor.visit_local_variable_declaration(&it.declaration);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_record_pattern<'a, V: Visit<'a>>(visitor: &mut V, it: &'a RecordPattern<'a>) {
        let kind = AstKind::RecordPattern(it);
        visitor.enter_node(kind);
        visitor.visit_type(&it.ty);
        visitor.visit_component_patterns(&it.components);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_component_pattern<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ComponentPattern<'a>) {
        match it {
            ComponentPattern::MatchAll(it) => visitor.visit_match_all_pattern(it),
            match_pattern!(ComponentPattern) => visitor.visit_pattern(it.to_pattern()),
        }
    }

    #[inline]
    pub fn walk_match_all_pattern<'a, V: Visit<'a>>(visitor: &mut V, it: &'a MatchAllPattern) {
        let kind = AstKind::MatchAllPattern(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_binary_expression<'a, V: Visit<'a>>(visitor: &mut V, it: &'a BinaryExpression<'a>) {
        walk_expression_chain(visitor, ExpressionChain::BinaryExpression(it));
    }

    #[inline]
    pub fn walk_assignment_expression<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a AssignmentExpression<'a>,
    ) {
        let kind = AstKind::AssignmentExpression(it);
        visitor.enter_node(kind);
        visitor.visit_assignment_target(&it.target);
        visitor.visit_expression(&it.value);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_assignment_target<'a, V: Visit<'a>>(visitor: &mut V, it: &'a AssignmentTarget<'a>) {
        match it {
            AssignmentTarget::Identifier(it) => visitor.visit_identifier_reference(it),
            AssignmentTarget::FieldAccess(it) => visitor.visit_field_access(it),
            AssignmentTarget::ArrayAccess(it) => visitor.visit_array_access(it),
            AssignmentTarget::Parenthesized(it) => visitor.visit_parenthesized_expression(it),
        }
    }

    #[inline]
    pub fn walk_conditional_expression<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ConditionalExpression<'a>,
    ) {
        let kind = AstKind::ConditionalExpression(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.condition);
        visitor.visit_expression(&it.when_true);
        visitor.visit_expression(&it.when_false);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_switch_expression<'a, V: Visit<'a>>(visitor: &mut V, it: &'a SwitchExpression<'a>) {
        let kind = AstKind::SwitchExpression(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.selector);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_switch_items(&it.cases);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_parenthesized_expression<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ParenthesizedExpression<'a>,
    ) {
        let kind = AstKind::ParenthesizedExpression(it);
        visitor.enter_node(kind);
        visitor.visit_expression(&it.expression);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_lambda_expression<'a, V: Visit<'a>>(visitor: &mut V, it: &'a LambdaExpression<'a>) {
        let kind = AstKind::LambdaExpression(it);
        visitor.enter_node(kind);
        visitor.enter_scope(&it.scope_id);
        visitor.visit_formal_parameters(&it.params);
        visitor.visit_lambda_body(&it.body);
        visitor.leave_scope();
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_lambda_body<'a, V: Visit<'a>>(visitor: &mut V, it: &'a LambdaBody<'a>) {
        match it {
            LambdaBody::Block(it) => visitor.visit_block(it),
            match_expression!(LambdaBody) => visitor.visit_expression(it.to_expression()),
        }
    }

    #[inline]
    pub fn walk_method_reference<'a, V: Visit<'a>>(visitor: &mut V, it: &'a MethodReference<'a>) {
        walk_expression_chain(visitor, ExpressionChain::MethodReference(it));
    }

    #[inline]
    pub fn walk_method_reference_target<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a MethodReferenceTarget<'a>,
    ) {
        match it {
            match_expression!(MethodReferenceTarget) => {
                visitor.visit_expression(it.to_expression())
            }
            match_type!(MethodReferenceTarget) => visitor.visit_type(it.to_type()),
        }
    }

    #[inline]
    pub fn walk_type_parameters<'a, V: Visit<'a>>(visitor: &mut V, it: &'a TypeParameters<'a>) {
        let kind = AstKind::TypeParameters(it);
        visitor.enter_node(kind);
        visitor.visit_type_parameter_list(&it.params);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_type_parameter<'a, V: Visit<'a>>(visitor: &mut V, it: &'a TypeParameter<'a>) {
        let kind = AstKind::TypeParameter(it);
        visitor.enter_node(kind);
        visitor.visit_annotations(&it.annotations);
        visitor.visit_binding_identifier(&it.name);
        visitor.visit_types(&it.bounds);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_variable_type<'a, V: Visit<'a>>(visitor: &mut V, it: &'a VariableType<'a>) {
        match it {
            VariableType::Var(it) => visitor.visit_var_type(it),
            match_type!(VariableType) => visitor.visit_type(it.to_type()),
        }
    }

    #[inline]
    pub fn walk_var_type<'a, V: Visit<'a>>(visitor: &mut V, it: &'a VarType) {
        let kind = AstKind::VarType(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_type<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Type<'a>) {
        match it {
            Type::Primitive(it) => visitor.visit_primitive_type(it),
            Type::Array(it) => visitor.visit_array_type(it),
            match_class_type!(Type) => visitor.visit_class_type(it.to_class_type()),
        }
    }

    #[inline]
    pub fn walk_method_result<'a, V: Visit<'a>>(visitor: &mut V, it: &'a MethodResult<'a>) {
        match it {
            MethodResult::Void(it) => visitor.visit_void_type(it),
            match_type!(MethodResult) => visitor.visit_type(it.to_type()),
        }
    }

    #[inline]
    pub fn walk_void_type<'a, V: Visit<'a>>(visitor: &mut V, it: &'a VoidType) {
        let kind = AstKind::VoidType(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_primitive_type<'a, V: Visit<'a>>(visitor: &mut V, it: &'a PrimitiveType<'a>) {
        let kind = AstKind::PrimitiveType(it);
        visitor.enter_node(kind);
        visitor.visit_annotations(&it.annotations);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_class_type<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ClassType<'a>) {
        match it {
            ClassType::Simple(it) => visitor.visit_simple_class_type(it),
            ClassType::Qualified(it) => visitor.visit_qualified_class_type(it),
        }
    }

    #[inline]
    pub fn walk_simple_class_type<'a, V: Visit<'a>>(visitor: &mut V, it: &'a SimpleClassType<'a>) {
        let kind = AstKind::SimpleClassType(it);
        visitor.enter_node(kind);
        visitor.visit_annotations(&it.annotations);
        visitor.visit_identifier_reference(&it.name);
        if let Some(args) = &it.args {
            visitor.visit_type_arguments(args);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_qualified_class_type<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a QualifiedClassType<'a>,
    ) {
        let mut node = it;
        let mut parents = SmallVec::<[&'a QualifiedClassType<'a>; 4]>::new();
        loop {
            visitor.enter_node(AstKind::QualifiedClassType(node));
            match &node.qualifier {
                ClassType::Qualified(next) => {
                    parents.push(node);
                    node = next;
                }
                qualifier @ ClassType::Simple(_) => {
                    visitor.visit_class_type(qualifier);
                    break;
                }
            }
        }
        loop {
            visitor.visit_annotations(&node.annotations);
            visitor.visit_identifier_name(&node.name);
            if let Some(args) = &node.args {
                visitor.visit_type_arguments(args);
            }
            visitor.leave_node(AstKind::QualifiedClassType(node));
            let Some(parent) = parents.pop() else { break };
            node = parent;
        }
    }

    #[inline]
    pub fn walk_array_type<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ArrayType<'a>) {
        let kind = AstKind::ArrayType(it);
        visitor.enter_node(kind);
        visitor.visit_type(&it.element);
        visitor.visit_array_type_dimensions(&it.dims);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_array_type_dimension<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ArrayTypeDimension<'a>,
    ) {
        let kind = AstKind::ArrayTypeDimension(it);
        visitor.enter_node(kind);
        visitor.visit_annotations(&it.annotations);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_type_arguments<'a, V: Visit<'a>>(visitor: &mut V, it: &'a TypeArguments<'a>) {
        let kind = AstKind::TypeArguments(it);
        visitor.enter_node(kind);
        visitor.visit_type_argument_list(&it.args);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_type_argument<'a, V: Visit<'a>>(visitor: &mut V, it: &'a TypeArgument<'a>) {
        match it {
            TypeArgument::Wildcard(it) => visitor.visit_wildcard_type_argument(it),
            match_type!(TypeArgument) => visitor.visit_type(it.to_type()),
        }
    }

    #[inline]
    pub fn walk_wildcard_type_argument<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a WildcardTypeArgument<'a>,
    ) {
        let kind = AstKind::WildcardTypeArgument(it);
        visitor.enter_node(kind);
        visitor.visit_annotations(&it.annotations);
        if let Some(bound) = &it.bound {
            visitor.visit_wildcard_bound(bound);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_wildcard_bound<'a, V: Visit<'a>>(visitor: &mut V, it: &'a WildcardBound<'a>) {
        let kind = AstKind::WildcardBound(it);
        visitor.enter_node(kind);
        visitor.visit_type(&it.ty);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_int_literal<'a, V: Visit<'a>>(visitor: &mut V, it: &'a IntLiteral<'a>) {
        let kind = AstKind::IntLiteral(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_float_literal<'a, V: Visit<'a>>(visitor: &mut V, it: &'a FloatLiteral<'a>) {
        let kind = AstKind::FloatLiteral(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_char_literal<'a, V: Visit<'a>>(visitor: &mut V, it: &'a CharLiteral<'a>) {
        let kind = AstKind::CharLiteral(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_string_literal<'a, V: Visit<'a>>(visitor: &mut V, it: &'a StringLiteral<'a>) {
        let kind = AstKind::StringLiteral(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_boolean_literal<'a, V: Visit<'a>>(visitor: &mut V, it: &'a BooleanLiteral) {
        let kind = AstKind::BooleanLiteral(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_null_literal<'a, V: Visit<'a>>(visitor: &mut V, it: &'a NullLiteral) {
        let kind = AstKind::NullLiteral(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_identifier_name<'a, V: Visit<'a>>(visitor: &mut V, it: &'a IdentifierName<'a>) {
        let kind = AstKind::IdentifierName(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_identifier_reference<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a IdentifierReference<'a>,
    ) {
        let kind = AstKind::IdentifierReference(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_binding_identifier<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a BindingIdentifier<'a>,
    ) {
        let kind = AstKind::BindingIdentifier(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_label_identifier<'a, V: Visit<'a>>(visitor: &mut V, it: &'a LabelIdentifier<'a>) {
        let kind = AstKind::LabelIdentifier(it);
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_name<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Name<'a>) {
        match it {
            Name::Identifier(it) => visitor.visit_identifier_name(it),
            Name::Qualified(it) => visitor.visit_qualified_name(it),
        }
    }

    pub fn walk_qualified_name<'a, V: Visit<'a>>(visitor: &mut V, it: &'a QualifiedName<'a>) {
        let mut node = it;
        let mut parents = SmallVec::<[&'a QualifiedName<'a>; 4]>::new();
        loop {
            visitor.enter_node(AstKind::QualifiedName(node));
            match &node.qualifier {
                Name::Qualified(next) => {
                    parents.push(node);
                    node = next;
                }
                qualifier @ Name::Identifier(_) => {
                    visitor.visit_name(qualifier);
                    break;
                }
            }
        }
        loop {
            visitor.visit_identifier_name(&node.name);
            visitor.leave_node(AstKind::QualifiedName(node));
            let Some(parent) = parents.pop() else { break };
            node = parent;
        }
    }

    #[inline]
    pub fn walk_type_name<'a, V: Visit<'a>>(visitor: &mut V, it: &'a TypeName<'a>) {
        match it {
            TypeName::Identifier(it) => visitor.visit_identifier_reference(it),
            TypeName::Qualified(it) => visitor.visit_qualified_type_name(it),
        }
    }

    pub fn walk_qualified_type_name<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a QualifiedTypeName<'a>,
    ) {
        let mut node = it;
        let mut parents = SmallVec::<[&'a QualifiedTypeName<'a>; 4]>::new();
        loop {
            visitor.enter_node(AstKind::QualifiedTypeName(node));
            match &node.qualifier {
                TypeName::Qualified(next) => {
                    parents.push(node);
                    node = next;
                }
                qualifier @ TypeName::Identifier(_) => {
                    visitor.visit_type_name(qualifier);
                    break;
                }
            }
        }
        loop {
            visitor.visit_identifier_name(&node.name);
            visitor.leave_node(AstKind::QualifiedTypeName(node));
            let Some(parent) = parents.pop() else { break };
            node = parent;
        }
    }

    #[inline]
    pub fn walk_modifier<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Modifier<'a>) {
        match it {
            Modifier::Keyword(it) => visitor.visit_keyword_modifier(it),
            Modifier::Annotation(it) => visitor.visit_annotation(it),
        }
    }

    #[inline]
    pub fn walk_keyword_modifier<'a, V: Visit<'a>>(visitor: &mut V, it: &'a KeywordModifier) {
        // No `AstKind` for this type
    }

    #[inline]
    pub fn walk_annotation<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Annotation<'a>) {
        let kind = AstKind::Annotation(it);
        visitor.enter_node(kind);
        visitor.visit_type_name(&it.name);
        if let Some(args) = &it.args {
            visitor.visit_annotation_arguments(args);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_annotation_argument<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a AnnotationArgument<'a>,
    ) {
        match it {
            AnnotationArgument::Pair(it) => visitor.visit_element_value_pair(it),
            match_element_value!(AnnotationArgument) => {
                visitor.visit_element_value(it.to_element_value())
            }
        }
    }

    #[inline]
    pub fn walk_element_value_pair<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ElementValuePair<'a>,
    ) {
        let kind = AstKind::ElementValuePair(it);
        visitor.enter_node(kind);
        visitor.visit_identifier_name(&it.name);
        visitor.visit_element_value(&it.value);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_element_value<'a, V: Visit<'a>>(visitor: &mut V, it: &'a ElementValue<'a>) {
        match it {
            ElementValue::Annotation(it) => visitor.visit_annotation(it),
            ElementValue::Array(it) => visitor.visit_element_value_array_initializer(it),
            match_expression!(ElementValue) => visitor.visit_expression(it.to_expression()),
        }
    }

    #[inline]
    pub fn walk_element_value_array_initializer<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a ElementValueArrayInitializer<'a>,
    ) {
        let kind = AstKind::ElementValueArrayInitializer(it);
        visitor.enter_node(kind);
        visitor.visit_element_values(&it.values);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_import_declarations<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, ImportDeclaration<'a>>,
    ) {
        for el in it {
            visitor.visit_import_declaration(el);
        }
    }

    #[inline]
    pub fn walk_type_declarations<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, TypeDeclaration<'a>>,
    ) {
        for el in it {
            visitor.visit_type_declaration(el);
        }
    }

    #[inline]
    pub fn walk_annotations<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, Annotation<'a>>) {
        for el in it {
            visitor.visit_annotation(el);
        }
    }

    #[inline]
    pub fn walk_class_members<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, ClassMember<'a>>) {
        for el in it {
            visitor.visit_class_member(el);
        }
    }

    #[inline]
    pub fn walk_module_directives<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, ModuleDirective<'a>>,
    ) {
        for el in it {
            visitor.visit_module_directive(el);
        }
    }

    #[inline]
    pub fn walk_keyword_modifiers<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, KeywordModifier>,
    ) {
        for el in it {
            visitor.visit_keyword_modifier(el);
        }
    }

    #[inline]
    pub fn walk_names<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, Name<'a>>) {
        for el in it {
            visitor.visit_name(el);
        }
    }

    #[inline]
    pub fn walk_type_names<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, TypeName<'a>>) {
        for el in it {
            visitor.visit_type_name(el);
        }
    }

    #[inline]
    pub fn walk_modifiers<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, Modifier<'a>>) {
        for el in it {
            visitor.visit_modifier(el);
        }
    }

    #[inline]
    pub fn walk_types<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, Type<'a>>) {
        for el in it {
            visitor.visit_type(el);
        }
    }

    #[inline]
    pub fn walk_interface_members<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, InterfaceMember<'a>>,
    ) {
        for el in it {
            visitor.visit_interface_member(el);
        }
    }

    #[inline]
    pub fn walk_enum_constants<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, EnumConstant<'a>>,
    ) {
        for el in it {
            visitor.visit_enum_constant(el);
        }
    }

    #[inline]
    pub fn walk_expressions<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, Expression<'a>>) {
        for el in it {
            visitor.visit_expression(el);
        }
    }

    #[inline]
    pub fn walk_record_components<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, RecordComponent<'a>>,
    ) {
        for el in it {
            visitor.visit_record_component(el);
        }
    }

    #[inline]
    pub fn walk_annotation_interface_members<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, AnnotationInterfaceMember<'a>>,
    ) {
        for el in it {
            visitor.visit_annotation_interface_member(el);
        }
    }

    #[inline]
    pub fn walk_array_type_dimensions<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, ArrayTypeDimension<'a>>,
    ) {
        for el in it {
            visitor.visit_array_type_dimension(el);
        }
    }

    #[inline]
    pub fn walk_variable_declarators<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, VariableDeclarator<'a>>,
    ) {
        for el in it {
            visitor.visit_variable_declarator(el);
        }
    }

    #[inline]
    pub fn walk_formal_parameter_list<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, FormalParameter<'a>>,
    ) {
        for el in it {
            visitor.visit_formal_parameter(el);
        }
    }

    #[inline]
    pub fn walk_statements<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, Statement<'a>>) {
        for el in it {
            visitor.visit_statement(el);
        }
    }

    #[inline]
    pub fn walk_switch_items<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, SwitchItem<'a>>) {
        for el in it {
            visitor.visit_switch_item(el);
        }
    }

    #[inline]
    pub fn walk_patterns<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, Pattern<'a>>) {
        for el in it {
            visitor.visit_pattern(el);
        }
    }

    #[inline]
    pub fn walk_resources<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, Resource<'a>>) {
        for el in it {
            visitor.visit_resource(el);
        }
    }

    #[inline]
    pub fn walk_catch_clauses<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, CatchClause<'a>>) {
        for el in it {
            visitor.visit_catch_clause(el);
        }
    }

    #[inline]
    pub fn walk_class_types<'a, V: Visit<'a>>(visitor: &mut V, it: &'a Vec<'a, ClassType<'a>>) {
        for el in it {
            visitor.visit_class_type(el);
        }
    }

    #[inline]
    pub fn walk_variable_initializers<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, VariableInitializer<'a>>,
    ) {
        for el in it {
            visitor.visit_variable_initializer(el);
        }
    }

    #[inline]
    pub fn walk_array_dimensions<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, ArrayDimension<'a>>,
    ) {
        for el in it {
            visitor.visit_array_dimension(el);
        }
    }

    #[inline]
    pub fn walk_component_patterns<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, ComponentPattern<'a>>,
    ) {
        for el in it {
            visitor.visit_component_pattern(el);
        }
    }

    #[inline]
    pub fn walk_type_parameter_list<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, TypeParameter<'a>>,
    ) {
        for el in it {
            visitor.visit_type_parameter(el);
        }
    }

    #[inline]
    pub fn walk_type_argument_list<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, TypeArgument<'a>>,
    ) {
        for el in it {
            visitor.visit_type_argument(el);
        }
    }

    #[inline]
    pub fn walk_annotation_arguments<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, AnnotationArgument<'a>>,
    ) {
        for el in it {
            visitor.visit_annotation_argument(el);
        }
    }

    #[inline]
    pub fn walk_element_values<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &'a Vec<'a, ElementValue<'a>>,
    ) {
        for el in it {
            visitor.visit_element_value(el);
        }
    }

    #[derive(Clone, Copy)]
    enum ExpressionChain<'a> {
        FieldAccess(&'a FieldAccess<'a>),
        MethodCall(&'a MethodCall<'a>),
        QualifiedClassInstanceCreation(&'a QualifiedClassInstanceCreation<'a>),
        ArrayAccess(&'a ArrayAccess<'a>),
        InstanceofExpression(&'a InstanceofExpression<'a>),
        BinaryExpression(&'a BinaryExpression<'a>),
        MethodReference(&'a MethodReference<'a>),
    }

    impl<'a> ExpressionChain<'a> {
        fn from_expression(expression: &'a Expression<'a>) -> Option<Self> {
            match expression {
                Expression::FieldAccess(it) => Some(Self::FieldAccess(it)),
                Expression::MethodCall(it) => Some(Self::MethodCall(it)),
                Expression::QualifiedClassInstanceCreation(it) => {
                    Some(Self::QualifiedClassInstanceCreation(it))
                }
                Expression::ArrayAccess(it) => Some(Self::ArrayAccess(it)),
                Expression::Instanceof(it) => Some(Self::InstanceofExpression(it)),
                Expression::Binary(it) => Some(Self::BinaryExpression(it)),
                Expression::MethodReference(it) => Some(Self::MethodReference(it)),
                _ => None,
            }
        }
    }

    fn walk_expression_chain<'a, V: Visit<'a>>(visitor: &mut V, mut node: ExpressionChain<'a>) {
        let mut parents = SmallVec::<[ExpressionChain<'a>; 4]>::new();
        loop {
            let next = match node {
                ExpressionChain::FieldAccess(it) => {
                    visitor.enter_node(AstKind::FieldAccess(it));
                    Some(&it.target)
                }
                ExpressionChain::MethodCall(it) => {
                    visitor.enter_node(AstKind::MethodCall(it));
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
                }
                ExpressionChain::QualifiedClassInstanceCreation(it) => {
                    visitor.enter_node(AstKind::QualifiedClassInstanceCreation(it));
                    Some(&it.qualifier)
                }
                ExpressionChain::ArrayAccess(it) => {
                    visitor.enter_node(AstKind::ArrayAccess(it));
                    Some(&it.array)
                }
                ExpressionChain::InstanceofExpression(it) => {
                    visitor.enter_node(AstKind::InstanceofExpression(it));
                    Some(&it.operand)
                }
                ExpressionChain::BinaryExpression(it) => {
                    visitor.enter_node(AstKind::BinaryExpression(it));
                    Some(&it.lhs)
                }
                ExpressionChain::MethodReference(it) => {
                    visitor.enter_node(AstKind::MethodReference(it));
                    if let Some(expression) = it.target.as_expression() {
                        Some(expression)
                    } else {
                        visitor.visit_method_reference_target(&it.target);
                        None
                    }
                }
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
                ExpressionChain::FieldAccess(it) => {
                    visitor.visit_identifier_name(&it.name);
                    visitor.leave_node(AstKind::FieldAccess(it));
                }
                ExpressionChain::MethodCall(it) => {
                    if let Callee::Member(member) = &it.callee {
                        if let Some(type_args) = &member.type_args {
                            visitor.visit_type_arguments(type_args);
                        }
                        visitor.visit_identifier_name(&member.name);
                        visitor.leave_node(AstKind::MethodMember(member));
                    }
                    visitor.visit_expressions(&it.args);
                    visitor.leave_node(AstKind::MethodCall(it));
                }
                ExpressionChain::QualifiedClassInstanceCreation(it) => {
                    if let Some(constructor_type_args) = &it.constructor_type_args {
                        visitor.visit_type_arguments(constructor_type_args);
                    }
                    visitor.visit_annotations(&it.annotations);
                    visitor.visit_identifier_name(&it.name);
                    if let Some(type_args) = &it.type_args {
                        visitor.visit_type_arguments(type_args);
                    }
                    visitor.visit_expressions(&it.args);
                    if let Some(body) = &it.body {
                        visitor.visit_class_body(body);
                    }
                    visitor.leave_node(AstKind::QualifiedClassInstanceCreation(it));
                }
                ExpressionChain::ArrayAccess(it) => {
                    visitor.visit_expression(&it.index);
                    visitor.leave_node(AstKind::ArrayAccess(it));
                }
                ExpressionChain::InstanceofExpression(it) => {
                    visitor.visit_instanceof_rhs(&it.rhs);
                    visitor.leave_node(AstKind::InstanceofExpression(it));
                }
                ExpressionChain::BinaryExpression(it) => {
                    visitor.visit_expression(&it.rhs);
                    visitor.leave_node(AstKind::BinaryExpression(it));
                }
                ExpressionChain::MethodReference(it) => {
                    if let Some(type_args) = &it.type_args {
                        visitor.visit_type_arguments(type_args);
                    }
                    if let Some(name) = &it.name {
                        visitor.visit_identifier_name(name);
                    }
                    visitor.leave_node(AstKind::MethodReference(it));
                }
            }
            let Some(parent) = parents.pop() else { break };
            node = parent;
        }
    }
}

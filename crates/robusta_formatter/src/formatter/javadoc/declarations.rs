use std::ops::Range;

use robusta_ast::{
    AstKind, Comment, CompilationUnit, FormalParameter, TypeDeclaration, TypeParameters,
};
use robusta_ast_visit::Visit;
use robusta_javadoc::Javadoc;
use robusta_span::GetSpan;

use super::DeclaredParameters;

/// The declarations a Javadoc comment can document.
#[derive(Debug, Default)]
pub struct JavadocDeclarations<'a> {
    /// Sorted start offsets, annotations and modifiers included.
    starts: Vec<u32>,
    /// Sorted by start: the declarations that take `@param` tags.
    parameters: Vec<ParameterList>,
    names: Vec<&'a str>,
}

/// A declaration's `@param` subjects in [`JavadocDeclarations::names`].
#[derive(Debug)]
struct ParameterList {
    start: u32,
    names: Range<usize>,
    type_parameters: usize,
}

impl<'a> JavadocDeclarations<'a> {
    /// Collect the declarations, or none when the file has no documentation
    /// comment.
    pub fn collect(unit: &'a CompilationUnit<'a>) -> Self {
        if !unit.comments.iter().any(Comment::is_documentation) {
            return Self::default();
        }
        let mut collector = DeclarationCollector { declarations: Self::default() };
        collector.visit_compilation_unit(unit);
        let mut declarations = collector.declarations;
        declarations.starts.sort_unstable();
        declarations.starts.dedup();
        declarations.parameters.sort_by_key(|list| list.start);
        declarations.parameters.dedup_by_key(|list| list.start);
        declarations
    }

    #[cfg(test)]
    pub(crate) fn with_starts(starts: Vec<u32>) -> Self {
        Self { starts, ..Self::default() }
    }

    pub(crate) fn starts(&self) -> &[u32] {
        &self.starts
    }

    /// The type parameters and parameters of the declaration `document`
    /// documents, if it takes `@param` tags.
    pub(crate) fn parameters(
        &self,
        document: Javadoc<'_>,
        source: &str,
        comments: &[Comment],
    ) -> Option<DeclaredParameters<'_>> {
        let start = document.documented_offset(source, comments)?;
        let index = self.parameters.binary_search_by_key(&start, |list| list.start).ok()?;
        let list = &self.parameters[index];
        let (type_parameters, parameters) =
            self.names[list.names.clone()].split_at(list.type_parameters);
        Some(DeclaredParameters { type_parameters, parameters })
    }

    fn push_parameters(
        &mut self,
        start: u32,
        type_parameters: Option<&'a TypeParameters<'a>>,
        parameters: impl Iterator<Item = &'a str>,
    ) {
        let type_parameters = type_parameters.map_or(&[][..], |list| &list.params);
        let first = self.names.len();
        self.names.extend(type_parameters.iter().map(|parameter| parameter.name.name));
        self.names.extend(parameters);
        let names = first..self.names.len();
        self.parameters.push(ParameterList {
            start,
            names,
            type_parameters: type_parameters.len(),
        });
    }
}

struct DeclarationCollector<'a> {
    declarations: JavadocDeclarations<'a>,
}

impl<'a> Visit<'a> for DeclarationCollector<'a> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        let start = match kind {
            // javac attaches a package's Javadoc only in a unit declaring no
            // types, as package-info.java is.
            AstKind::OrdinaryCompilationUnit(unit) => match &unit.package {
                Some(package)
                    if unit.types.iter().all(|ty| matches!(ty, TypeDeclaration::Empty(_))) =>
                {
                    package.span.start
                }
                _ => return,
            },
            AstKind::ModuleDeclaration(_)
            | AstKind::ClassDeclaration(_)
            | AstKind::InterfaceDeclaration(_)
            | AstKind::EnumDeclaration(_)
            | AstKind::EnumConstant(_)
            | AstKind::RecordDeclaration(_)
            | AstKind::AnnotationInterfaceDeclaration(_)
            | AstKind::AnnotationInterfaceElementDeclaration(_)
            | AstKind::FieldDeclaration(_)
            | AstKind::MethodDeclaration(_)
            | AstKind::ConstructorDeclaration(_)
            | AstKind::CompactConstructorDeclaration(_) => kind.span().start,
            _ => return,
        };
        let declarations = &mut self.declarations;
        declarations.starts.push(start);
        let names = |parameters: &'a [FormalParameter<'a>]| {
            parameters.iter().map(|parameter| parameter.name.name)
        };
        match kind {
            AstKind::MethodDeclaration(method) => {
                declarations.push_parameters(
                    start,
                    method.type_params.as_deref(),
                    names(&method.params.items),
                );
            }
            AstKind::ConstructorDeclaration(constructor) => declarations.push_parameters(
                start,
                constructor.type_params.as_deref(),
                names(&constructor.params.items),
            ),
            AstKind::ClassDeclaration(class) => {
                declarations.push_parameters(
                    start,
                    class.type_params.as_deref(),
                    std::iter::empty(),
                );
            }
            AstKind::InterfaceDeclaration(interface) => {
                declarations.push_parameters(
                    start,
                    interface.type_params.as_deref(),
                    std::iter::empty(),
                );
            }
            AstKind::RecordDeclaration(record) => declarations.push_parameters(
                start,
                record.type_params.as_deref(),
                record.header.components.iter().map(|component| component.name.name),
            ),
            _ => {}
        }
    }
}

use robusta_span::Span;

use crate::utils::{follows_documentation, removal_span};
use crate::{Fix, LintContext};

/// Delete an import, with its lines when nothing else is on them. Returns none
/// for an import with a comment inside it, which would go with it, or after a
/// documentation comment, which the deletion could hand to the next
/// declaration.
pub fn delete_import(context: &LintContext<'_, '_, '_>, import: Span) -> Option<Fix> {
    if context.has_comments_between(import) {
        return None;
    }
    // Even before another import, since a fix may delete that one too.
    if follows_documentation(context.comments(), import) {
        return None;
    }
    Some(Fix::delete(removal_span(context.source_text(), import)))
}

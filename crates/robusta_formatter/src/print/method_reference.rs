use robusta_ast::MethodReference;
use robusta_span::GetSpan;

use crate::ast_nodes::{AstNode, impls::ast_nodes::format_node_leading_comments};
use crate::formatter::prelude::*;
use crate::print::FormatWrite;
use crate::print::member_expression::{
    Member, MemberLayout, format_member_after_object, format_member_object,
};
use crate::print::type_parameters::format_type_arguments_before_name;
use crate::utils::is_header_condition;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, MethodReference<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let target = self.target();
        let target_span = target.span();
        let is_member_chain = format_member_object(&target, target_span.end, f);
        let operand_start = match (&self.type_args, &self.name) {
            (Some(type_args), _) => type_args.span.start,
            (None, Some(name)) => name.span.start,
            // `new` has no node.
            (None, None) => {
                let first_colon_end = f.comments().position_after_character(target_span.end, b':');
                let colons_end = f.comments().position_after_character(first_colon_end, b':');
                f.comments().token_start_after(colons_end)
            }
        };
        let name = format_with(|f| {
            if let Some(type_args) = self.type_args() {
                format_type_arguments_before_name(f, &type_args);
            }
            if let Some(name) = self.name() {
                name.fmt(f);
            } else {
                if let Some(type_args) = &self.type_args {
                    // `new` has no node, so it takes the leading pass here.
                    let new_start =
                        f.comments().position_after_character(type_args.span.end, b'n') - 1;
                    format_node_leading_comments(&self.as_ast_nodes(), new_start, f);
                }
                "new".fmt(f);
            }
        });
        let layout =
            MemberLayout::new(target.inner().as_expression(), self.parent(), is_member_chain);
        let in_header = is_header_condition(self.parent(), self.span);
        let member = Member { separator: "::", start: operand_start, layout, in_header };
        format_member_after_object(target_span, member, &name, f);
    }
}

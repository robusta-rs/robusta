// Adapted from https://github.com/oxc-project/oxc/blob/566b11b2db6b664e787e67e2199d4ffcaa31c30a/crates/oxc_formatter/src/utils/format_node_without_trailing_comments.rs

use crate::formatter::prelude::*;

pub fn format_content_without_comments_after<'a, T>(
    content: &T,
    end: u32,
    f: &mut JavaFormatter<'_, 'a>,
) where
    T: Format<'a, JavaFormatContext<'a>>,
{
    let previous = f.comments().limit_comments_up_to(end);
    content.fmt(f);
    f.comments().restore_view_limit(previous);
}

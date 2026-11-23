use std::ops::Range;

use markdown::mdast::Code;
use robusta_javadoc::markdown::{IndentedCode, block_end, continuation_prefix, quote_content};

use super::super::embedded::code_lines;
use super::{Edit, Normalizer};

fn indented_edit(code: &IndentedCode, fence: &str) -> Edit {
    let mut text = format!("{}{fence}", " ".repeat(code.padding));
    text.push_str(&code_lines(&code.value, &code.prefix));
    text.push('\n');
    text.push_str(&code.prefix);
    text.push_str(fence);
    if code.newline {
        text.push('\n');
    }
    Edit::new(code.range.clone(), text)
}

impl Normalizer<'_> {
    /// Fence every code block with more backticks than any run inside it.
    pub(super) fn code_block(&mut self, code: &Code, range: Range<usize>) {
        let run = code.value.split(|ch| ch != '`').map(str::len).max().unwrap_or(0);
        let fence = "`".repeat(run.max(2) + 1);
        let group = match self.content.as_bytes()[range.start] {
            b'`' | b'~' => {
                let prefix = continuation_prefix(self.content, range.start);
                self.fenced_code(range, &prefix, fence)
            }
            b' ' | b'\t' => IndentedCode::new(
                self.content,
                range,
                code.position.as_ref().map_or(0, |p| p.start.column - 1),
            )
            .map(|code| vec![indented_edit(&code, &fence)]),
            _ => None,
        };
        if let Some(group) = group {
            self.push(group);
        }
    }

    /// Replace both fences, or close a block that runs to the end.
    fn fenced_code(&self, range: Range<usize>, prefix: &str, fence: String) -> Option<Vec<Edit>> {
        let source = &self.content[range.clone()];
        let marker = source.as_bytes()[0];
        let first_line = source.split('\n').next().unwrap_or_default();
        let opening = first_line.bytes().take_while(|byte| *byte == marker).count();
        if first_line[opening..].contains('`') {
            return None;
        }
        let mut group = vec![Edit::new(range.start..range.start + opening, fence.clone())];
        let last_start = source.rfind('\n').map(|newline| range.start + newline + 1);
        let closing = last_start.and_then(|start| {
            let line = &self.content[start..range.end];
            let unquoted = quote_content(line);
            let quote_depth =
                line[..line.len() - unquoted.len()].bytes().filter(|byte| *byte == b'>').count();
            if quote_depth != prefix.bytes().filter(|byte| *byte == b'>').count() {
                return None;
            }
            let trimmed = unquoted.trim();
            let start = start + line.len() - unquoted.trim_start().len();
            (trimmed.len() >= opening && trimmed.bytes().all(|byte| byte == marker))
                .then_some(start..start + trimmed.len())
        });
        let end = block_end(self.content, &range);
        group.push(match closing {
            Some(closing) => Edit::new(closing, fence),
            None => Edit::new(end..end, format!("\n{prefix}{fence}")),
        });
        Some(group)
    }
}

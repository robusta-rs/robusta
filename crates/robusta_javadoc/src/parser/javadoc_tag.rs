use std::ops::Range;

use cow_utils::CowUtils;
use robusta_syntax::identifier::{
    identifier_part, is_horizontal_whitespace_char, is_identifier_start,
};

use crate::structure::{balanced_inline_end, block_tag_start};

/// Whether a tag uses `@name` at a line boundary or `{@name ...}` inline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TagForm {
    Block,
    Inline,
}

/// A parsed Javadoc tag and the name-like subject it documents or references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tag {
    form: TagForm,
    name: String,
    referenced_name: Option<String>,
    range: Range<usize>,
}

impl Tag {
    pub fn form(&self) -> TagForm {
        self.form
    }

    /// Where the tag is in the comment content: an inline tag with its
    /// braces, or a block tag's `@name`.
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn referenced_name(&self) -> Option<&str> {
        self.referenced_name.as_deref()
    }
}

pub fn parse_tags(content: &str, verbatim_ranges: &[Range<usize>]) -> Vec<Tag> {
    let mut tags = Vec::new();
    let mut line_start = 0usize;
    for line in content.split('\n') {
        let line_end = line_start + line.len();
        if let Some(start) = block_tag_start(content, line_start, line_end, verbatim_ranges)
            && let Some(tag) = parse_tag(&content[start + 1..line_end], TagForm::Block)
        {
            tags.push((start..start + 1 + tag.name.len(), tag));
        }
        line_start = line_end.saturating_add(1);
    }

    let mut position = 0usize;
    while position + 2 <= content.len() {
        let Some(offset) = content[position..].find("{@") else {
            break;
        };
        let start = position + offset;
        if verbatim_ranges.iter().any(|range| range.contains(&start)) {
            position = start + 2;
            continue;
        }
        let closed_end = balanced_inline_end(content, start);
        let end = closed_end.unwrap_or(content.len());
        let body_end = closed_end.map_or(end, |closed| closed - 1);
        if let Some(tag) = parse_tag(&content[start + 2..body_end], TagForm::Inline) {
            tags.push((start..end, tag));
        }
        position = end.max(start + 2);
    }

    tags.sort_by_key(|(range, _)| range.start);
    tags.into_iter().map(|(range, tag)| Tag { range, ..tag }).collect()
}

fn parse_tag(text: &str, form: TagForm) -> Option<Tag> {
    let name_end =
        text.find(|character: char| !tag_name_character(character)).unwrap_or(text.len());
    if name_end == 0 {
        return None;
    }
    let name = &text[..name_end];
    let body = text[name_end..].trim_start();
    let referenced_name = match name.cow_to_ascii_lowercase().as_ref() {
        "link" | "linkplain" | "value" | "see" | "throws" | "exception" => reference_target(body),
        "param" => parameter_name(body),
        _ => None,
    };

    Some(Tag {
        form,
        name: name.to_string(),
        referenced_name: referenced_name.map(str::to_string),
        range: 0..0,
    })
}

fn reference_target(body: &str) -> Option<&str> {
    let body = body.trim_start();
    if body.is_empty() || body.starts_with('"') || body.starts_with("<a") {
        return None;
    }

    let mut parentheses = 0usize;
    for (index, character) in body.char_indices() {
        match character {
            '(' => parentheses += 1,
            ')' => parentheses = parentheses.saturating_sub(1),
            character if character.is_whitespace() && parentheses == 0 => {
                return Some(&body[..index]);
            }
            _ => {}
        }
    }
    Some(body)
}

fn parameter_name(body: &str) -> Option<&str> {
    let body = body.trim_start();
    if let Some(len) = type_parameter_len(body) {
        return Some(&body[1..len - 1]);
    }
    let end = body.find(char::is_whitespace).unwrap_or(body.len());
    (end > 0).then_some(&body[..end])
}

/// The length of `@param <T>` at the start of `text`, indentation included.
/// Its `<T>` names a type parameter, even one spelled like block HTML such as
/// `<PRE>`, and may start a later line.
pub fn type_parameter_tag_len(text: &str) -> Option<usize> {
    let tag = text.trim_start_matches(is_horizontal_whitespace_char);
    let body = tag
        .strip_prefix("@param")?
        .trim_start_matches(|ch| ch == '\n' || is_horizontal_whitespace_char(ch));
    Some(text.len() - body.len() + type_parameter_len(body)?)
}

/// The length of the `<T>` that starts `body`, where `T` is a Java identifier.
fn type_parameter_len(body: &str) -> Option<usize> {
    let rest = body.strip_prefix('<')?;
    let name_len = rest.find(|ch| identifier_part(ch).is_none()).unwrap_or(rest.len());
    (rest.starts_with(is_identifier_start) && rest[name_len..].starts_with('>'))
        .then_some(name_len + 2)
}

fn tag_name_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '_' | '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_target_keeps_spaces_inside_a_member_signature() {
        assert_eq!(
            reference_target("Map#put(Object, Object) label"),
            Some("Map#put(Object, Object)")
        );
    }

    #[test]
    fn param_subject_strips_type_parameter_brackets() {
        assert_eq!(parameter_name("<T> the element type"), Some("T"));
        assert_eq!(parameter_name("value the value"), Some("value"));
    }
}

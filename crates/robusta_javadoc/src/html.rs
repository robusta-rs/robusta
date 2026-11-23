/// A block-level HTML tag that starts or ends a flow.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HtmlBlock {
    /// `<p>`
    Paragraph,
    /// `<ul>`, `<ol>`, or `<dl>`
    List,
    /// `</ul>`, `</ol>`, or `</dl>`
    ListEnd,
    /// `<li>`, `<dt>`, or `<dd>`
    ListItem,
    /// `<h1>` to `<h6>`
    Heading,
    /// `</h1>` to `</h6>`
    HeadingEnd,
    /// `<blockquote>` or `</blockquote>`
    Blockquote,
    /// `<pre>` or `<table>`, whose content keeps its line breaks and spacing.
    Preformatted,
}

impl HtmlBlock {
    /// Whether the tag starts a new flow.
    pub(crate) fn starts_flow(self) -> bool {
        self != Self::HeadingEnd
    }

    /// Whether the tag ends its flow.
    pub(crate) fn ends_flow(self) -> bool {
        matches!(self, Self::List | Self::ListEnd | Self::HeadingEnd | Self::Blockquote)
    }
}

/// Find the first block-level tag in `text`, returning its kind and byte range.
pub fn find_block_tag(text: &str) -> Option<(HtmlBlock, usize, usize)> {
    let mut from = 0;
    while let Some(offset) = text[from..].find('<') {
        let start = from + offset;
        if let Some((block, end)) = block_tag_at(text, start) {
            return Some((block, start, end));
        }
        from = start + 1;
    }
    None
}

const LISTS: &[&str] = &["ul", "ol", "dl"];
const HEADINGS: &[&str] = &["h1", "h2", "h3", "h4", "h5", "h6"];

/// The block-level HTML tag at `start`, with its end offset.
pub fn block_tag_at(text: &str, start: usize) -> Option<(HtmlBlock, usize)> {
    let rest = text.get(start..)?.strip_prefix('<')?;
    let (closing, rest) = match rest.strip_prefix('/') {
        Some(rest) => (true, rest),
        None => (false, rest),
    };
    let name_len = rest.bytes().take_while(u8::is_ascii_alphanumeric).count();
    let name = &rest[..name_len];
    let named = |names: &[&str]| names.iter().any(|candidate| name.eq_ignore_ascii_case(candidate));
    let block = match closing {
        false if named(&["p"]) => HtmlBlock::Paragraph,
        false if named(LISTS) => HtmlBlock::List,
        true if named(LISTS) => HtmlBlock::ListEnd,
        false if named(&["li", "dt", "dd"]) => HtmlBlock::ListItem,
        false if named(HEADINGS) => HtmlBlock::Heading,
        true if named(HEADINGS) => HtmlBlock::HeadingEnd,
        _ if named(&["blockquote"]) => HtmlBlock::Blockquote,
        _ => return None,
    };
    let after_name = &rest[name_len..];
    if after_name.starts_with('_') {
        return None;
    }
    let mut quote = None;
    let close = after_name.bytes().position(|byte| {
        if matches!(byte, b'\n' | b'\r') {
            return true;
        }
        if Some(byte) == quote {
            quote = None;
        } else if quote.is_none() && matches!(byte, b'\'' | b'"') {
            quote = Some(byte);
        }
        byte == b'>' && quote.is_none()
    })?;
    let close = if after_name.as_bytes()[close] == b'>' {
        close + 1
    } else {
        multiline_tag_end(after_name)?
    };
    let end = text.len() - after_name.len() + close;
    Some((block, end))
}

// Require complete attributes before crossing a physical line.
fn multiline_tag_end(mut rest: &str) -> Option<usize> {
    let length = rest.len();
    loop {
        let before_space = rest.len();
        rest = rest.trim_start_matches(|ch: char| ch.is_ascii_whitespace());
        if rest.starts_with('>') {
            return Some(length - rest.len() + 1);
        }
        if rest.starts_with("/>") {
            return Some(length - rest.len() + 2);
        }
        if rest.len() == before_space {
            return None;
        }
        let name_len = rest
            .bytes()
            .take_while(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':'))
            .count();
        if name_len == 0 {
            return None;
        }
        rest = rest[name_len..].trim_start_matches(|ch: char| ch.is_ascii_whitespace());
        rest = rest.strip_prefix('=')?.trim_start_matches(|ch: char| ch.is_ascii_whitespace());
        if let Some(quote @ ('\'' | '"')) = rest.chars().next() {
            let end = rest[1..].find([quote, '\n', '\r'])? + 1;
            if !rest[end..].starts_with(quote) {
                return None;
            }
            rest = &rest[end + 1..];
        } else {
            let end = rest.find(|ch: char| ch.is_ascii_whitespace() || ch == '>')?;
            if end == 0 || rest[..end].contains(['<', '\'', '"', '`', '=']) {
                return None;
            }
            rest = &rest[end..];
        }
    }
}

/// Whether `text` opens a `<pre>` or `<table>` element.
pub fn starts_preformatted(text: &str) -> bool {
    ["pre", "table"].into_iter().any(|name| opens_element(text, name))
}

/// The end of the `<pre>` or `<table>` element at `position`, or of `content`
/// when the element is not closed.
pub fn preformatted_end(content: &str, position: usize) -> Option<usize> {
    let text = &content[position..];
    let name = ["pre", "table"].into_iter().find(|name| opens_element(text, name))?;
    let open_end = position + text.find('>')? + 1;
    Some(close_end(content, open_end, name).unwrap_or(content.len()))
}

fn opens_element(text: &str, name: &str) -> bool {
    text.strip_prefix('<')
        .and_then(|rest| rest.get(..name.len()))
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(name))
        && text
            .as_bytes()
            .get(name.len() + 1)
            .is_some_and(|&byte| byte == b'>' || byte.is_ascii_whitespace())
}

fn close_end(content: &str, from: usize, name: &str) -> Option<usize> {
    let close = format!("</{name}");
    let offset = content.as_bytes()[from..]
        .windows(close.len())
        .position(|window| window.eq_ignore_ascii_case(close.as_bytes()))?;
    let name_end = from + offset + close.len();
    let boundary = content.as_bytes().get(name_end).copied()?;
    if boundary != b'>' && !boundary.is_ascii_whitespace() {
        return close_end(content, name_end, name);
    }
    content[name_end..].find('>').map(|offset| name_end + offset + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_start_offsets() {
        for (text, start) in [("<p>", 1), ("<p>", 3), ("<p>", 4), ("é<p>", 1), ("", 0)] {
            assert_eq!(block_tag_at(text, start), None);
        }
        assert_eq!(block_tag_at("é<p>", 2), Some((HtmlBlock::Paragraph, 5)));
    }

    #[test]
    fn multiline_tags_require_complete_attributes() {
        for text in [
            "<p class=\"note\"\n>",
            "<p\n class=\"note\"\n>",
            "<li\n data-kind = 'note > value'\n>",
            "<h2 class\n=\nheading\n>",
            "<blockquote\n cite=example\n/>",
            "</ul\n>",
        ] {
            assert_eq!(block_tag_at(text, 0).map(|(_, end)| end), Some(text.len()), "{text}");
        }
        for text in [
            "<p\n prose>",
            "<p class=\"note\"\n @param value later >",
            "<p class=\"unfinished\n @param value later >",
            "<p class=note\n <li>later >",
            "<p class=\n >",
            "<p class=\"note\"\n data=x=y>",
        ] {
            assert_eq!(block_tag_at(text, 0), None, "{text}");
        }
    }

    #[test]
    fn finds_block_tags_by_name_and_ignores_others() {
        fn find(text: &str) -> Option<(HtmlBlock, &str)> {
            find_block_tag(text).map(|(block, start, end)| (block, &text[start..end]))
        }

        assert_eq!(
            find("a <b>b</b> <P class=\"x\">c"),
            Some((HtmlBlock::Paragraph, "<P class=\"x\">"))
        );
        assert_eq!(find("one</li><LI>two"), Some((HtmlBlock::ListItem, "<LI>")));
        assert_eq!(find("x</ol>"), Some((HtmlBlock::ListEnd, "</ol>")));
        assert_eq!(find("</h3>"), Some((HtmlBlock::HeadingEnd, "</h3>")));
        assert_eq!(find("<blockquote>"), Some((HtmlBlock::Blockquote, "<blockquote>")));
        for text in ["</p>", "<pre>", "<param>", "<h7>", "a < b", "<p", "<ul_x>"] {
            assert_eq!(find(text), None, "{text}");
        }
        for text in ["<p\nprose>```", "<p\rprose>~~~", "<p class=\"note\"\nprose>```"] {
            assert_eq!(find(text), None, "{text}");
        }
    }
}

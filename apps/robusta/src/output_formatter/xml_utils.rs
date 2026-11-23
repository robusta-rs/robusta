use std::fmt::Write as _;

/// Escape text for an XML attribute value or element body.
///
/// Tab, LF, and CR become character references so attribute normalization
/// keeps them. XML 1.0 cannot carry the other C0 controls, U+FFFE, or U+FFFF
/// even as references, so they take their Java escape spelling, such as
/// `\u001B`.
pub(super) fn escape(value: &str, out: &mut String) {
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            '\t' => out.push_str("&#9;"),
            '\n' => out.push_str("&#10;"),
            '\r' => out.push_str("&#13;"),
            '\0'..='\u{1f}' | '\u{fffe}' | '\u{ffff}' => {
                let _ = write!(out, "\\u{:04X}", u32::from(ch));
            }
            _ => out.push(ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_markup_and_keeps_everything_else_verbatim() {
        let mut out = String::new();
        escape("café <b> & \"q\" 'a' 🎉", &mut out);
        assert_eq!(out, "café &lt;b&gt; &amp; &quot;q&quot; &apos;a&apos; 🎉");
    }

    #[test]
    fn appends_to_the_buffer_it_is_given() {
        let mut out = String::from("name=\"");
        escape("a<b", &mut out);
        out.push('"');
        assert_eq!(out, "name=\"a&lt;b\"");
    }

    #[test]
    fn spells_characters_xml_cannot_carry_as_java_escapes() {
        let mut out = String::new();
        escape("<a\u{1b}b> & \0\u{1f}\u{fffe}\u{ffff}", &mut out);
        assert_eq!(out, r"&lt;a\u001Bb&gt; &amp; \u0000\u001F\uFFFE\uFFFF");
    }

    #[test]
    fn keeps_whitespace_that_attribute_normalization_would_fold() {
        let mut out = String::new();
        escape("a\tb\nc\r\nd", &mut out);
        assert_eq!(out, "a&#9;b&#10;c&#13;&#10;d");
    }

    #[test]
    fn leaves_no_character_xml_forbids() {
        let c1: String = ('\u{7f}'..='\u{9f}').collect();
        let text: String = ('\0'..' ').chain(c1.chars()).chain(['\u{fffe}', '\u{ffff}']).collect();
        let mut out = String::new();
        escape(&text, &mut out);
        assert!(!out.chars().any(|c| c < ' ' || matches!(c, '\u{fffe}' | '\u{ffff}')), "{out:?}");
        assert_eq!(out.matches("\\u").count(), 31, "{out:?}");
        // XML 1.0 allows DEL and the C1 controls.
        assert!(out.contains(&c1), "{out:?}");
    }
}

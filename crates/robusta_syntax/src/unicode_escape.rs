//! Java source gaps after Unicode escape translation (JLS §3.3).

/// Iterates bytes with their raw end offsets, decoding whitespace escapes.
/// Gaps start at lexer token/comment boundaries, outside backslash runs.
pub struct SourceBytes<'a> {
    bytes: &'a [u8],
    position: usize,
    eligible: bool,
}

impl<'a> SourceBytes<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { bytes: source.as_bytes(), position: 0, eligible: true }
    }

    /// Like iteration, with a caller-selected set of escapes to decode.
    pub fn next_with_escapes(
        &mut self,
        decode_escape: impl FnOnce(u8) -> bool,
    ) -> Option<(u8, usize)> {
        let byte = *self.bytes.get(self.position)?;
        if byte == b'\\' {
            if self.eligible
                && let Some((value, end)) = unicode_escape(self.bytes, self.position)
                && let Ok(byte) = u8::try_from(value)
                && decode_escape(byte)
            {
                self.position = end;
                self.eligible = true;
                return Some((byte, end));
            }
            self.eligible = !self.eligible;
        } else {
            self.eligible = true;
        }
        self.position += 1;
        Some((byte, self.position))
    }
}

/// The value of the Unicode escape at `start`, and where it ends.
pub fn unicode_escape(bytes: &[u8], start: usize) -> Option<(u32, usize)> {
    if bytes.get(start) != Some(&b'\\') {
        return None;
    }
    let digits = start + 1 + bytes[start + 1..].iter().take_while(|&&byte| byte == b'u').count();
    if digits == start + 1 {
        return None;
    }
    let value = bytes
        .get(digits..digits + 4)?
        .iter()
        .try_fold(0, |value, &digit| Some(value * 16 + char::from(digit).to_digit(16)?))?;
    Some((value, digits + 4))
}

/// Where the whitespace `is_whitespace` accepts, escaped or not, ends after `from`.
#[expect(clippy::cast_possible_truncation, reason = "parsed source spans fit u32")]
pub fn skip_whitespace(source: &str, from: u32, is_whitespace: impl Fn(u8) -> bool) -> u32 {
    let mut position = 0;
    for (byte, end) in SourceBytes::new(&source[from as usize..]) {
        if !is_whitespace(byte) {
            break;
        }
        position = end;
    }
    from + position as u32
}

impl Iterator for SourceBytes<'_> {
    type Item = (u8, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_with_escapes(|byte| byte.is_ascii_whitespace())
    }
}

#[cfg(test)]
mod tests {
    use crate::identifier::is_horizontal_whitespace;

    use super::*;

    #[test]
    fn whitespace_scans_read_escapes_and_stop_at_tokens_and_ineligible_escapes() {
        for gap in [r"\u000a", r"\uu000D", r"\uuu000d\u000A", "\\u000d\n", "\r\\uu000a"] {
            assert_eq!(
                skip_whitespace(gap, 0, |byte| byte.is_ascii_whitespace()),
                u32::try_from(gap.len()).unwrap()
            );
            assert_eq!(skip_whitespace(gap, 0, is_horizontal_whitespace), 0);
        }
        let source = r"x \u0020\uu0009\uuu000c\u000a\u0029";
        assert_eq!(skip_whitespace(source, 1, is_horizontal_whitespace), 23);
        assert_eq!(skip_whitespace(source, 1, |byte| byte.is_ascii_whitespace()), 29);
        for gap in [r"\\u000a", r"\u005cu000a", r"\u000", r"\u2028", r"\u0029"] {
            assert_eq!(skip_whitespace(gap, 0, |byte| byte.is_ascii_whitespace()), 0);
        }
    }
}

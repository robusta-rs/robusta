use robusta_javadoc::Flow;

/// Rename a block tag synonym: `@exception` -> `@throws`.
pub fn normalize_tag_name(flow: &mut Flow) {
    if flow.tag_name() == Some("exception")
        && let Some(first) = flow.atoms_mut().first_mut()
    {
        first.replace_range(1.."@exception".len(), "throws");
    }
}

/// Insert a period at byte `end` of `atom` when the text before it ends in a
/// letter or digit.
pub fn append_trailing_dot(atom: &mut String, end: usize) {
    if atom[..end].chars().next_back().is_some_and(char::is_alphanumeric) {
        atom.insert(end, '.');
    }
}

pub fn capitalize_first(word: &mut str) {
    let is_url = ["http://", "https://"].iter().any(|scheme| {
        word.get(..scheme.len()).is_some_and(|prefix| prefix.eq_ignore_ascii_case(scheme))
    });
    if !is_url && let Some(first) = word.get_mut(..1) {
        first.make_ascii_uppercase();
    }
}

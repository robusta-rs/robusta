use super::{TagSection, tag_subject};

/// The names a declaration's `@param` tags document.
#[derive(Clone, Copy, Debug)]
pub struct DeclaredParameters<'a> {
    pub type_parameters: &'a [&'a str],
    pub parameters: &'a [&'a str],
}

impl DeclaredParameters<'_> {
    /// Position of `<T>` among the type parameters, or of `name` after them.
    fn position(self, subject: &str) -> Option<usize> {
        match subject.strip_prefix('<').and_then(|rest| rest.strip_suffix('>')) {
            Some(name) => self.type_parameters.iter().position(|declared| *declared == name),
            None => self
                .parameters
                .iter()
                .position(|declared| *declared == subject)
                .map(|index| self.type_parameters.len() + index),
        }
    }
}

/// Reorder `@param` tags to match the declaration, only when they name exactly
/// its type parameters and parameters.
pub fn reorder_param_tags(sections: &mut [TagSection], declared: DeclaredParameters<'_>) {
    if sections.len() < 2
        || sections.len() != declared.type_parameters.len() + declared.parameters.len()
    {
        return;
    }
    let mut positions = Vec::with_capacity(sections.len());
    for section in sections.iter() {
        let Some(position) = tag_subject(section).and_then(|subject| declared.position(subject))
        else {
            return;
        };
        positions.push(position);
    }
    if positions.is_sorted() {
        return;
    }
    let mut seen = vec![false; positions.len()];
    for &position in &positions {
        if std::mem::replace(&mut seen[position], true) {
            return;
        }
    }
    let mut keyed: Vec<_> =
        positions.into_iter().zip(sections.iter_mut().map(std::mem::take)).collect();
    keyed.sort_unstable_by_key(|(position, _)| *position);
    for (section, (_, sorted)) in sections.iter_mut().zip(keyed) {
        *section = sorted;
    }
}

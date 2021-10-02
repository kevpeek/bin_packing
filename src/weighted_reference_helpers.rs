use crate::{WeightUnit, WeightedReference};

// TODO -- create a trait for this so there can be default implementations on existing types.

/// Provide a way to create a Vec<WeightedReference> from an Iterable of things that
/// can already be converted to `WeightedReference`.
impl<'a, T: Into<WeightUnit> + Copy> WeightedReference<'a, T> {
    pub fn collect_from<S: Iterator<Item = &'a T>>(source: S) -> Vec<WeightedReference<'a, T>> {
        source.into_iter().map(WeightedReference::from).collect()
    }
}

/// Provide default conversion into a `WeightedReference` for anything that can
/// be converted into a usize.
impl<'a, T: Into<WeightUnit> + Copy> From<&'a T> for WeightedReference<'a, T> {
    fn from(source: &'a T) -> Self {
        WeightedReference {
            weight: (*source).into(),
            reference: source,
        }
    }
}

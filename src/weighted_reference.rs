use crate::{WeightUnit, Weighted, WeightedReference};

// TODO -- create a trait for this so there can be default implementations on existing types.

impl<'a, T> Weighted<'a, T> for WeightedReference<'a, T> {
    fn weight(&self) -> usize {
        self.weight
    }

    fn reference(&self) -> &'a T {
        self.reference
    }
}

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

#[cfg(test)]
mod tests {
    use crate::algorithms::first_fit;
    use crate::WeightedReference;

    #[test]
    fn pack_weighted_references() {
        let input: Vec<u8> = vec![1, 2, 3, 4];
        let weighted = WeightedReference::collect_from(input.iter());
        let bins = first_fit(5, weighted.into_iter()).unwrap();
        assert_eq!(vec![&1, &2], bins[0]);
        assert_eq!(vec![&3], bins[1]);
        assert_eq!(vec![&4], bins[2]);
    }
}

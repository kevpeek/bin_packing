use crate::WeightedReference;

/// Trait for converting something into an `Iterator` of `Weighted`.
/// The `weight_fn` will be applied to each element to produce that element's weight.
pub(crate) trait AsWeighted<'a, T, F>
where
    F: 'a + Fn(&T) -> usize,
{
    fn to_weighted(self, weight_fn: F) -> Box<dyn Iterator<Item = WeightedReference<'a, T>> + 'a>;
}

impl<'a, T, I, F> AsWeighted<'a, T, F> for I
where
    I: 'a + Iterator<Item = &'a T>,
    T: 'a,
    F: 'a + Fn(&T) -> usize,
{
    fn to_weighted(self, weight_fn: F) -> Box<dyn Iterator<Item = WeightedReference<'a, T>> + 'a> {
        Box::new(self.map(move |it| WeightedReference {
            weight: weight_fn(it),
            reference: it,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::packing_algorithms_trait::PackingAlgorithms;
    use crate::weighted_iterator::AsWeighted;

    #[test]
    fn test_first_fit() {
        let numbers: Vec<u64> = vec![1, 2, 3, 4];
        let bins = numbers.iter().to_weighted(|num| *num as usize).first_fit(4);
        assert_eq!(3, bins.unwrap().len());
    }

    #[test]
    fn test_first_fit_decreasing() {
        let numbers: Vec<u64> = vec![1, 2, 3, 4];
        let bins = numbers
            .iter()
            .to_weighted(|num| *num as usize)
            .first_fit_decreasing(4);
        assert_eq!(3, bins.unwrap().len());
    }
}

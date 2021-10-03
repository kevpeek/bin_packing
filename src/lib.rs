pub mod algorithms;
pub mod errors;
mod packing_algorithms_trait;
mod weighted_iterator;

type WeightUnit = usize;

/// Trait defining a weighted item. Bin packing algorithms pack `Weighted` things into bins.
pub trait Weighted<'a, T> {
    fn weight(&self) -> usize;
    fn reference(&self) -> &'a T;
}

/// A reference to anything that is `Into<usize>` + `Copy` can automatically be
/// use as a `Weighted` by using itself for its weight.
impl<'a, T: Into<usize> + Copy> Weighted<'a, T> for &'a T {
    fn weight(&self) -> usize {
        (**self).into()
    }

    fn reference(&self) -> &'a T {
        self
    }
}

/// Allow arbitrary values to be `Weighted` by combing a weight with a reference to the value.
pub struct WeightedReference<'a, T> {
    weight: usize,
    reference: &'a T,
}

impl<'a, T> Weighted<'a, T> for WeightedReference<'a, T> {
    fn weight(&self) -> usize {
        self.weight
    }

    fn reference(&self) -> &'a T {
        self.reference
    }
}

#[cfg(test)]
mod tests {
    use crate::packing_algorithms_trait::PackingAlgorithms;
    use crate::weighted_iterator::AsWeighted;

    #[test]
    fn using_usize_directly() {
        let numbers: Vec<u8> = vec![1, 2, 3, 4];
        let bins = numbers.iter().first_fit(5).unwrap();
        assert_eq!(3, bins.len());
    }

    #[test]
    fn using_to_weighted() {
        let words: Vec<&str> = vec!["hello", "world", "how", "are", "you"];
        let bins = words
            .iter()
            // Each items weight will be the length of the str.
            .to_weighted(|it| it.len())
            .first_fit_decreasing(11)
            .unwrap();
        assert_eq!(2, bins.len());
    }

    #[test]
    fn sandbox() {
        let words = vec!["hello", "world", "how", "are", "you"];
        let bin_capacity = 11;
        let bins = words.iter()
            // Each items weight will be the length of the str.
            .to_weighted(|word| word.len())
            .first_fit_decreasing(bin_capacity)
            .unwrap();
        println!("{:?}", bins);
    }
}

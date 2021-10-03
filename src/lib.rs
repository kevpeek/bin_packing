pub mod algorithms;
pub mod errors;
mod packing_algorithms_trait;
mod weighted_iterator;

type WeightUnit = usize;

pub trait Weighted<'a, T> {
    fn weight(&self) -> usize;
    fn reference(&self) -> &'a T;
}

// TODO -- can this be done for all T: Into<usize> ?
impl<'a> Weighted<'a, usize> for &'a usize {
    fn weight(&self) -> usize {
        **self
    }

    fn reference(&self) -> &'a usize {
        self
    }
}

/// Combines a reference to an item and the item's weight.
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
    fn sandbox() {
        let numbers: Vec<u64> = (1..10).collect();
        let bins = numbers
            .iter()
            .to_weighted(|it| *it as usize)
            .first_fit_decreasing(11)
            .unwrap();
        println!("{:?}", bins);
    }
}

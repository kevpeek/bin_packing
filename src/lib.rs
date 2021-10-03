pub mod algorithms;
pub mod errors;
mod weighted_iterator;
mod weighted_reference;

type WeightUnit = usize;

pub trait Weighted<'a, T> {
    fn weight(&self) -> usize;
    fn reference(&self) -> &'a T;
}

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

#[cfg(test)]
mod tests {
    use crate::algorithms;

    #[test]
    fn sandbox() {
        let items: Vec<usize> = (1..10).collect();
        let bins = algorithms::first_fit_decreasing(11, items.iter()).unwrap();
        println!("{:?}", bins);
    }
}

pub mod algorithms;
pub mod errors;
mod weighted_reference_helpers;

type WeightUnit = usize;

pub trait Weighted<'a, T> {
    fn weight(&self) -> usize;
    fn reference(&self) -> &'a T;
}

/// The input to the various packing algorithms. Combines a reference to the item
/// and the items weight.
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
    use crate::{algorithms, WeightedReference};

    #[test]
    fn sandbox() {
        let items: Vec<u8> = (1..10).collect();
        let bins =
            algorithms::first_fit_decreasing(11, WeightedReference::collect_from(items.iter()))
                .unwrap();
        println!("{:?}", bins);
    }
}

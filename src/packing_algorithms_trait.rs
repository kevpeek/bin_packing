use crate::errors::Error;
use crate::{algorithms, WeightUnit, Weighted};

/// Trait allowing packing functions to be called on an instance.
pub trait PackingAlgorithms<'a, T> {
    fn first_fit(self, capacity: WeightUnit) -> Result<Vec<Vec<&'a T>>, Error>;
    fn first_fit_decreasing(self, capacity: WeightUnit) -> Result<Vec<Vec<&'a T>>, Error>;
    // TODO -- add additional fns here
}

/// Blanket implementation for Iterators of `Weighted`.
impl<'a, T, I, W> PackingAlgorithms<'a, T> for I
where
    I: Iterator<Item = W>,
    W: Weighted<'a, T>,
{
    fn first_fit(self, capacity: WeightUnit) -> Result<Vec<Vec<&'a T>>, Error> {
        algorithms::first_fit(capacity, self)
    }

    fn first_fit_decreasing(self, capacity: WeightUnit) -> Result<Vec<Vec<&'a T>>, Error> {
        algorithms::first_fit_decreasing(capacity, self)
    }
}

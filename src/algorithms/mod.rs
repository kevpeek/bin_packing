mod first_fit;

use crate::errors::Error;
use crate::{WeightUnit, Weighted};
pub use first_fit::first_fit;
use std::cmp::Reverse;

/// Internal struct representing the current contents and weight of a bin.
struct Bin<'a, T> {
    capacity: WeightUnit,
    load: WeightUnit,
    contents: Vec<&'a T>,
}

/// Sorts the input in descending order and then applies first_fit.
pub fn first_fit_decreasing<'a, T, R>(
    capacity: WeightUnit,
    items: Vec<R>,
) -> Result<Vec<Vec<&'a T>>, Error>
where
    R: Weighted<'a, T>,
{
    first_fit(capacity, sort_descending(items))
}

#[must_use]
pub fn sort_descending<'a, T, R>(mut items: Vec<R>) -> Vec<R>
where
    R: Weighted<'a, T>,
{
    items.sort_by_key(|it| Reverse(it.weight()));
    items
}

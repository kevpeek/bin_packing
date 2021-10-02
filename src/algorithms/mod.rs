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

/// Input needs to be something iterable over &T...
/// AND you need to be able to sort those &T
/// AND then pass the sorted &Ts into the subsequent packing algorithm.
///
/// # Errors
/// Returns Err if any item is too large to fit in a bin.
pub fn first_fit_decreasing<T: Weighted>(
    capacity: WeightUnit,
    items: Vec<&T>,
) -> Result<Vec<Vec<&T>>, Error> {
    first_fit(capacity, sort_descending(items))
}

#[must_use]
pub fn sort_descending<T: Weighted>(mut items: Vec<&T>) -> Vec<&T> {
    items.sort_by_key(|it| Reverse(it.weight()));
    items
}

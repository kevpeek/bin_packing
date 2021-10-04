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

impl<'a, T> Bin<'a, T> {
    fn new_from_item<W: Weighted<'a, T>>(capacity: usize, item: &W) -> Bin<'a, T> {
        Bin {
            capacity,
            load: item.weight(),
            contents: vec![item.reference()],
        }
    }

    fn has_room_for<W: Weighted<'a, T>>(&self, item: &W) -> bool {
        self.load + item.weight() <= self.capacity
    }

    fn add<W: Weighted<'a, T>>(&mut self, item: &W) {
        self.load += item.weight();
        self.contents.push(item.reference());
    }
}

/// Sorts the input in descending order and then applies `first_fit`.
///
/// # Errors
/// Returns Err if an item is too large to fit in a bin.
pub fn first_fit_decreasing<'a, T, R, I>(
    capacity: WeightUnit,
    items: I,
) -> Result<Vec<Vec<&'a T>>, Error>
where
    I: Iterator<Item = R>,
    R: Weighted<'a, T>,
{
    first_fit(capacity, sort_descending(items).into_iter())
}

#[must_use]
fn sort_descending<'a, T, R, I>(items: I) -> Vec<R>
where
    I: Iterator<Item = R>,
    R: Weighted<'a, T>,
{
    let mut items: Vec<R> = items.collect();
    items.sort_by_key(|it| Reverse(it.weight()));
    items
}

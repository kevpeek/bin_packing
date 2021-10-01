mod first_fit;

use crate::WeightUnit;
pub use first_fit::first_fit;

/// Internal struct representing the current contents and weight of a bin.
struct Bin<'a, T> {
    capacity: WeightUnit,
    load: WeightUnit,
    contents: Vec<&'a T>,
}

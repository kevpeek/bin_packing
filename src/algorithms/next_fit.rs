use crate::algorithms::Bin;
use crate::errors::Error;
use crate::errors::Error::ItemTooLargeError;
use crate::{WeightUnit, Weighted};

/// There is one bin open at a time. Each item is placed into that bin until an item doesn't fit.
/// Then, the bin is closed and a new bin is opened and the process continues.
///
/// # Errors
/// Returns Err if any item is too large to fit in a bin.
///
/// ```
///# use bin_packing::WeightedReference;
///# fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let items: Vec<usize> = vec![1, 2, 3, 4];
/// let bins = bin_packing::algorithms::next_fit(5, items.iter())?;
/// assert_eq!(3, bins.len());
///# Ok(())
///# }
/// ```
pub fn next_fit<'a, T, R, I>(capacity: WeightUnit, items: I) -> Result<Vec<Vec<&'a T>>, Error>
where
    I: Iterator<Item = R>,
    R: Weighted<'a, T>,
{
    let mut bins: Vec<Bin<T>> = Vec::new();
    let mut current_bin = Bin::with_capacity(capacity);
    'item_loop: for item in items {
        if item.weight() > capacity {
            return Err(ItemTooLargeError);
        }

        if current_bin.has_room_for(&item) {
            current_bin.add(&item);
            continue 'item_loop;
        }

        bins.push(current_bin);
        current_bin = Bin::new_from_item(capacity, &item);
    }
    if !current_bin.contents.is_empty() {
        bins.push(current_bin);
    }

    // Extract just the bin contents and return those.
    Ok(bins.into_iter().map(|bin| bin.contents).collect())
}

#[cfg(test)]
mod tests {
    use crate::algorithms::next_fit::next_fit;

    #[test]
    fn empty() {
        let input: Vec<usize> = Vec::new();
        let bins = next_fit(1, input.iter()).unwrap();
        assert!(bins.is_empty());
    }

    #[test]
    fn ordered() {
        let input: Vec<usize> = vec![1, 2, 3, 4];
        let bins = next_fit(5, input.iter()).unwrap();
        assert_eq!(vec![&1, &2], bins[0]);
        assert_eq!(vec![&3], bins[1]);
        assert_eq!(vec![&4], bins[2]);
    }

    #[test]
    fn item_too_large() {
        let input: Vec<usize> = vec![2];
        assert!(next_fit(1, input.iter()).is_err());
    }
}

use crate::algorithms::Bin;
use crate::errors::Error;
use crate::errors::Error::ItemTooLargeError;
use crate::{WeightUnit, Weighted};

/// Keeps all bins open. Places each item into the first bin with room. If no bin
/// has room, a new bin will be opened and placed at the end of the Vec of bins.
/// <https://en.wikipedia.org/wiki/First-fit_bin_packing>
///
/// ```
/// let items: Vec<u8> = (1..5).collect();
/// let bins = bin_packing::algorithms::first_fit(5, &items).unwrap();
/// assert_eq!(3, bins.len());
/// ```
pub fn first_fit<T: Weighted>(capacity: WeightUnit, items: &[T]) -> Result<Vec<Vec<&T>>, Error> {
    let mut bins: Vec<Bin<T>> = Vec::new();
    'item_loop: for item in items {
        if item.weight() > capacity {
            return Err(ItemTooLargeError);
        }
        for bin in bins.iter_mut() {
            if bin.load + item.weight() <= bin.capacity {
                bin.load += item.weight();
                bin.contents.push(item);
                continue 'item_loop;
            }
        }

        // We didn't find a bin with enough room, so add a new one.
        let new_bin = Bin {
            capacity,
            load: item.weight(),
            contents: vec![item],
        };
        bins.push(new_bin)
    }

    // Extract just the bin contents and return those.
    Ok(bins.into_iter().map(|bin| bin.contents).collect())
}

#[cfg(test)]
mod tests {
    use crate::algorithms::first_fit;

    #[test]
    fn empty() {
        let input: Vec<usize> = Vec::new();
        let bins = first_fit(1, &input).unwrap();
        assert!(bins.is_empty());
    }

    #[test]
    fn ordered() {
        let input: Vec<usize> = (1..=4).collect();
        let bins = first_fit(5, &input).unwrap();
        assert_eq!(vec![&1, &2], bins[0]);
        assert_eq!(vec![&3], bins[1]);
        assert_eq!(vec![&4], bins[2]);
    }

    #[test]
    fn item_too_large() {
        let input: Vec<usize> = vec![2];
        assert!(first_fit(1, &input).is_err());
    }
}

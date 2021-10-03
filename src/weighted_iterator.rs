use crate::algorithms;
use crate::errors::Error;
use crate::{WeightUnit, Weighted, WeightedReference};

trait AsWeighted<'a, T, F: Fn(&T) -> usize> {
    fn to_weighted(self, weight_fn: F) -> std::vec::IntoIter<WeightedReference<'a, T>>;
}

impl<'a, T, I, F> AsWeighted<'a, T, F> for I
where
    I: Iterator<Item = &'a T>,
    T: 'a,
    F: Fn(&T) -> usize,
{
    fn to_weighted(self, weight_fn: F) -> std::vec::IntoIter<WeightedReference<'a, T>> {
        let items_vec: Vec<WeightedReference<'a, T>> = self
            .map(|it| WeightedReference {
                weight: weight_fn(it),
                reference: it,
            })
            .collect();
        items_vec.into_iter()
    }
}

trait Packing<'a, T> {
    fn first_fit(self, capacity: WeightUnit) -> Result<Vec<Vec<&'a T>>, Error>;
}

impl<'a, T, I, W> Packing<'a, T> for I
where
    I: Iterator<Item = W>,
    W: Weighted<'a, T>,
{
    fn first_fit(self, capacity: WeightUnit) -> Result<Vec<Vec<&'a T>>, Error> {
        algorithms::first_fit(capacity, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::weighted_iterator::{AsWeighted, Packing};

    #[test]
    fn test() {
        let numbers: Vec<u64> = vec![1, 2, 3, 4];
        let bins = numbers.iter().to_weighted(|num| *num as usize).first_fit(4);
        assert_eq!(3, bins.unwrap().len());
    }
}

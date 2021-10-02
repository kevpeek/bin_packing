pub mod algorithms;
pub mod errors;

type WeightUnit = usize;

/// Trait for defining the weight of an object. Bin Packing algorithms run on instances of Weighted.
///
/// ```
/// use bin_packing::Weighted;
/// assert_eq!(123, 123u8.weight())
/// ```
pub trait Weighted {
    fn weight(&self) -> WeightUnit;
}

/// Default implementation for anything that can be converted into `WeightUnit`
impl<T: Into<WeightUnit> + Copy> Weighted for T {
    fn weight(&self) -> WeightUnit {
        (*self).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms;

    #[test]
    fn sandbox() {
        let items: Vec<u8> = (1..10).collect();
        let bins = algorithms::first_fit(11, items.iter().collect()).unwrap();
        println!("{:?}", bins);
    }
}

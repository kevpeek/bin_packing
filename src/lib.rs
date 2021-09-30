
struct Bin<'a, T> {
    capacity: WeightUnit,
    load: WeightUnit,
    contents: Vec<&'a T>
}

type WeightUnit = usize;
pub trait Weighted {
    fn weight(&self) -> WeightUnit;
}

// Default implementation for anything that can be converted into WeightUnit
impl<T: Into<WeightUnit> + Copy> Weighted for T
{
    fn weight(&self) -> WeightUnit {
        (*self).into()
    }
}

// TODO - rename and move to own module
pub fn pack<T: Weighted>(capacity: WeightUnit, items: &[T]) -> Vec<Vec<&T>> {
    let mut bins: Vec<Bin<T>> = Vec::new();
    'item_loop: for item in items {
        for bin in bins.iter_mut() {
            if bin.load + item.weight() <= bin.capacity {
                bin.load += item.weight();
                bin.contents.push(item);
                continue 'item_loop
            }
        }

        // We didn't find a bin with enough room, so add a new one.
        // TODO -- handle item that is too large for empty bin
        let new_bin = Bin {
            capacity,
            load: item.weight(),
            contents: vec![item]
        };
        bins.push(new_bin)
    }

    // Extract just the bin contents and return those.
    bins.into_iter()
        .map(|bin| bin.contents)
        .collect()
}


#[cfg(test)]
mod tests {
    use crate::pack;

    #[test]
    fn sandbox() {
        let items: Vec<usize> = (1..10).collect();
        let bins = pack(11, &items);
        println!("{:?}", bins);
    }
}

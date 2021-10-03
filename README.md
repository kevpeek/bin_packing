# bin_packing
Bin Packing Algorithms in Rust

Implementation of various bin packing algorithms wrapped up in an easy to use API.

### Usage

Bin packing functions can be called directly on an Iterator of usize.
```
let numbers: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let bin_capacity = 11;
let bins = numbers.iter()
    .first_fit_decreasing(bin_capacity).unwrap();
println!("{:?}", bins);
// [[10, 1], [9, 2], [8, 3], [7, 4], [6, 5]]
```

For other types, simply provide a mapping function from an item to its weight.
```
let words = vec!["hello", "world", "how", "are", "you"];
let bin_capacity = 9;
let bins = words.iter()
    // Each items weight will be the length of the str.
    .to_weighted(|word| word.len())
    .first_fit_decreasing(bin_capacity)
    .unwrap();
println!("{:?}", bins);
// [["hello", "how"], ["world", "are"], ["you"]]
```

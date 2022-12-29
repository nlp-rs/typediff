# Differ.rs
[![License](https://img.shields.io/badge/license-MIT%20%26%20Apache%202.0-green)](#license)
[![CI](https://github.com/nlp-rs/differ.rs/actions/workflows/main.yml/badge.svg)](https://github.com/nlp-rs/differ.rs/actions/workflows/main.yml)
[![Security audit](https://github.com/nlp-rs/differ.rs/actions/workflows/security-audit.yml/badge.svg)](https://github.com/nlp-rs/differ.rs/actions/workflows/security-audit.yml)
> warning: **Differ.rs is currently experimental**
This crate provides edit distance, delta vectors between 2 words, and lets you apply delta vectors in order to transform words.

## Install
```shell
cargo add differ-rs
```
or, simply add the following string to your Cargo.toml:
```
differ-rs = "0.0.0"
```

## Features
* `apply_diff`: Allows users to apply delta vectors in order to transform a words.
* `extra_traits`: all `struct`s implemented in `differ-rs` are `HammingDistance` and `LevenshteinDistance`. Each Struct implements the `diff` and `distance` methods. 

## How it works
* `apply_diff` works by giving a string and a transformation vector to the method. Then the transformation vector is applied to the string given in the first argument.
* `StringDiffAlgorithm` provides two methods `diff` which gives you a transformation vector from the first to second string. The `distance` method gives you the edit distance from the frist argument to the second argument. The structs `HammingDistance` and `LevenshteinDistance` have their own implementations for each method.

## Examples

Getting the edit distance between two words using Levenshtein algorithm 
```rs
use differ_rs::{LevenshteinDistance, StringDiffAlgorithm};
fn main(){
    let my_levensthein = LevenshteinDistance {};

    let edit_distance = my_levensthein.distance("Sitting", "Kitten");
    
    assert_eq!(3, edit_distance)
}
```
>Note: We are getting the edit distance to get from "Sitting" to "Kitten".

Getting the delta vectors between two words using Levenshtein algorithm 
```rs
use differ_rs::{LevenshteinDistance, StringDiffAlgorithm};
fn main(){
    let my_levensthein = LevenshteinDistance {};

    let delta_vec = my_levensthein.diff("Sitting", "Kitten");
    
    for i in delta_vec.iter(){
        println!("{:?}", i);
    }
}
```

This example outputs:

```text
StringDiffOp { kind: Delete('g'), index: 6 }
StringDiffOp { kind: Substitute('i', 'e'), index: 4 }
StringDiffOp { kind: Substitute('S', 'K'), index: 0 }
```

Getting the edit distance between two words using Hamming algorithm 
```rs
use differ_rs::{HammingDistance, StringDiffAlgorithm};
fn main(){
    let my_hamming = HammingDistance {};

    let edit_distance = my_hamming.distance("karolin", "kathrin");
    
    assert_eq!(3, edit_distance);
}
```
Note: We are getting the edit distance to get from "karolin" to "kathrin",
additionally the first string and second string must be the same length, or
will cause a panic to be triggered. 


Getting the delta vectors between two words using Hamming algorithm 
```rs
use differ_rs::{HammingDistance, StringDiffAlgorithm};
fn main(){
    let my_hamming = HammingDistance {};

    let delta_vec = my_hamming.diff("karolin", "kathrin");
    
    for i in delta_vec.iter(){
        println!("{:?}", i);
    }
}
```
This example outputs:

```text
StringDiffOp { kind: Substitute('r', 't'), index: 2 }
StringDiffOp { kind: Substitute('o', 'h'), index: 3 }
StringDiffOp { kind: Substitute('l', 'r'), index: 4 }
```

Applying delta vectors to words
```rs
use differ_rs::{HammingDistance, LevenshteinDistance, StringDiffAlgorithm,apply_diff};
fn main(){
    let my_levensthein = LevenshteinDistance {};
    let levensthein_delta_vec = my_levensthein.diff("sitting", "kitten");
    let delta_applied_v1 = apply_diff("sitting", levensthein_delta_vec);

    let my_hamming = HammingDistance {};
    let hamming_delta_vec = my_hamming.diff("karolin", "kathrin");
    let delta_applied_v2 = apply_diff("karolin", hamming_delta_vec);

    assert_eq!("kitten", delta_applied_v1);
    assert_eq!("kathrin", delta_applied_v2);
}
```

## License
Licensed under either of
 * Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([`LICENSE-MIT`](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

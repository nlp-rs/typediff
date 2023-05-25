# Differ.rs
[![License](https://img.shields.io/badge/license-MIT%20%26%20Apache%202.0-green)](#license)
[![CI](https://github.com/nlp-rs/differ.rs/actions/workflows/main.yml/badge.svg)](https://github.com/nlp-rs/differ.rs/actions/workflows/main.yml)
[![Security audit](https://github.com/nlp-rs/differ.rs/actions/workflows/security-audit.yml/badge.svg)](https://github.com/nlp-rs/differ.rs/actions/workflows/security-audit.yml)
> warning: **Differ.rs is currently experimental**
This crate provides edit distance, deltas between 2 words, lets you apply deltas in order to transform words, and lets you get the similarity and difference between two words.

## Install
```shell
cargo add differ-rs
```
or, simply add the following string to your Cargo.toml:

```toml
differ-rs = "0.0.0"
```

## Features
* `Diff` struct: Contains a Box<> of operations between two strings. Also keeps track of length of longest string. Has methods that allows users to get the edit distance between two words, and view delta operations. 
* `DiffScoreConfig` struct: implements a default trait which the user can adjust the sub_cost, lowercase_sub_cost, indel_cost, and transpose_cost. By default they are all set to 1. You can pass this struct to `similarity()` and `difference()` both which are methods of the `Diff` struct.
* `apply_diff()`: Allows users to apply deltas in order to transform a words.
* `levenshtein()`: Returns a Diff struct between string 1 and string 2. Levenshtein algorithm can detect insertions, deletions, and substitutions. 
* `hamming()`: Returns a Diff struct between string 1 and string 2. Hamming algorithm can only detect substitutions, and string 1 and string 2 must me equal length.
* `similarity()`: Tells you how similarity two words are.
* `difference()`: Tells you how different two words are.

## How it works
* `apply_diff()` works by giving a string and a transformation vector to the method. Then the transformation vector is applied to the string given in the first argument.
* `Diff` holds a `Box<StringDiffOp>`, and the longest length of any two strings. Both `levenshtein()`, and `hamming()`  return this struct.
* `similarity()`: works by taking each operation inside `Box<[StringDiffOp]>`. For each operation inside `Box<StringDiffOp>` it applies the `DiffScoreConfig` penalties.
* `difference()`: works by perorming the following formula `1 - similarity()`

## Examples

Getting the edit distance between two words using Levenshtein algorithm 
```rs
use differ_rs::levenshtein;

fn main(){
    let levensthein_edit_distance = levenshtein("Sitting", "Kitten").distance();

    assert_eq!(3, levensthein_edit_distance);
}
```
> **Note**: We are getting the edit distance to get from "Sitting" to "Kitten".

To view the delta between two words using Levenshtein algorithm 
```rs
use differ_rs::levenshtein;

fn main(){
    let my_levensthein = levenshtein("Sitting", "Kitten");

    for diff_op in my_levensthein.ops.iter() {
        println!("{:?}", diff_op);
    }
}
```

This example outputs:

```text
StringDiffOp { kind: Delete, index: 6 }
StringDiffOp { kind: Substitute('i', 'e'), index: 4 }
StringDiffOp { kind: Substitute('S', 'K'), index: 0 }
```

Getting the edit distance between two words using Hamming algorithm 
```rs
use differ_rs::hamming;

fn main(){
	let kathrin_edit_distance = hamming("karolin", "kathrin").distance();

    assert_eq!(3, kathrin_edit_distance);
}
```
> **Note**: We are getting the edit distance to get from "karolin" to "kathrin",
additionally the first string and second string must be the same length, or
will cause a panic to be triggered. 


Getting the deltas between two words using Hamming algorithm 
```rs
use differ_rs::hamming;

fn main(){
	let kathrin_edit_distance = hamming("karolin", "kathrin");

    for diff_op in kathrin_edit_distance.ops.iter() {
		println!("{:?}", diff_op);
	}
}
```
This example outputs:

```text
StringDiffOp { kind: Substitute('r', 't'), index: 2 }
StringDiffOp { kind: Substitute('o', 'h'), index: 3 }
StringDiffOp { kind: Substitute('l', 'r'), index: 4 }
```

Applying deltas to words
```rs
use differ_rs::{hamming, levenshtein, apply_diff};

fn main(){
    let my_levensthein = levenshtein("sitting", "kitten");
    let delta_applied_v1 = apply_diff("sitting", my_levensthein.ops.to_vec());


    let my_hamming = hamming("karolin", "kathrin");
    let delta_applied_v2 = apply_diff("karolin", my_hamming.ops.to_vec());

    assert_eq!("kitten", delta_applied_v1);
    assert_eq!("kathrin", delta_applied_v2);
}
```

Getting the similarity between two words
```rs
use differ_rs::{DiffScoreConfig, levenshtein, hamming};

fn main(){
    let levenshtein_diff = levenshtein("Kittens", "kitten");
    let hamming_diff = hamming("karolin", "kathrin");
    let mut config = DiffScoreConfig::default();

    assert_eq!(0.71428573, levenshtein_diff.similarity(&config));
    assert_eq!(0.5714286, hamming_diff.similarity(&config));

    config.lowercase_sub_cost = 0.5;

    assert_eq!(0.78571427, levenshtein_diff.similarity(&config));
}
```

Getting the difference between two words
```rs
use differ_rs::{DiffScoreConfig, levenshtein, hamming};
fn main(){
    let levenshtein_diff = levenshtein("Kittens", "kitten");
    let hamming_diff = hamming("karolin", "kathrin");
    let mut config = DiffScoreConfig::default();

    assert_eq!(1.0 - 0.71428573, levenshtein_diff.difference(&config));
    assert_eq!(1.0 - 0.5714286, hamming_diff.difference(&config));

    config.lowercase_sub_cost = 0.5;

    assert_eq!(1.0 - 0.78571427, levenshtein_diff.difference(&config));
}
```


## License
Licensed under either of
 * Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([`LICENSE-MIT`](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

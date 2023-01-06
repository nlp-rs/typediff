# Differ.rs
[![License](https://img.shields.io/badge/license-MIT%20%26%20Apache%202.0-green)](#license)
[![CI](https://github.com/nlp-rs/differ.rs/actions/workflows/main.yml/badge.svg)](https://github.com/nlp-rs/differ.rs/actions/workflows/main.yml)
[![Security audit](https://github.com/nlp-rs/differ.rs/actions/workflows/security-audit.yml/badge.svg)](https://github.com/nlp-rs/differ.rs/actions/workflows/security-audit.yml)
> warning: **Differ.rs is currently experimental**
This crate provides edit distance, deltas between 2 words, and lets you apply deltas in order to transform words.

## Install
```shell
cargo add differ-rs
```
or, simply add the following string to your Cargo.toml:

```toml
differ-rs = "0.0.0"
```

## Features
* `apply_diff` function: Allows users to apply deltas in order to transform a words.
* `Diff` struct: Contains a Box<> of operations between two strings. Also keeps track of length of longest string. Has methods that allows users to get the edit distance between two words, and view delta operations. 
* `levenshtein` function: Returns a Diff struct between string 1 and string 2 using levenshtein algorithm. 
* `hamming` function: Returns a Diff struct between string 1 and string 2 hamming algorithm. 

## How it works
* `apply_diff` works by giving a string and a transformation vector to the method. Then the transformation vector is applied to the string given in the first argument.
* `Diff` works by hodling a Box<> of operations, and longest length between any two strings. Both the `levenshtein`, `hamming` algorithm return this struct.

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

    my_levensthein.operations();
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

    kathrin_edit_distance.operations();
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
    let delta_applied_v1 = apply_diff("sitting", &my_levensthein.ops);


    let my_hamming = hamming("karolin", "kathrin");
    let delta_applied_v2 = apply_diff("karolin", &my_hamming.ops);

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

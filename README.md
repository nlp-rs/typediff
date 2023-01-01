# differ.rs

Differ.rs is a collection of natural langauge processing crates, all written in Rust.

# *Brief overview*
### Crates
 - [x] [`differ`](./crates/differ): Provides edit distance, delta vectors between 2 words, and word transformation 
   - [x]  `apply_diff`: Allows users to apply delta vectors in order to transform a words.
   - [x]  `extra_traits`: all `struct`s implemented in `differ-rs` are `HammingDistance` and `LevenshteinDistance`. Each Struct implements the `diff` and `distance` methods.
 - [ ] `bio_dif`: Proivdes functions in order to to align protein or nucleotide sequences.



## License
Licensed under either of
 * Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([`LICENSE-MIT`](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
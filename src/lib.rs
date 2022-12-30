pub mod apply_diff;
pub use  crate::apply_diff::{apply_diff};
pub mod hamming;
pub use  crate::hamming::{HammingDistance};
pub mod levenshtein;
pub use crate::levenshtein::{LevenshteinDistance};


#[doc = include_str!("../README.md")]
#[derive(PartialEq, Eq, Debug)]
pub enum StringDiffOpKind {
	Substitute(char, char),
	Insert(char),
	Delete(char),
	Transpose(usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct StringDiffOp {
	pub kind: StringDiffOpKind,
	pub index: usize,
}

impl StringDiffOp {
	pub fn new(kind: StringDiffOpKind, index: usize) -> Self {
		Self { kind, index }
	}

	pub fn new_delete(c: char, index: usize) -> Self {
		Self::new(StringDiffOpKind::Delete(c), index)
	}

	pub fn new_insert(c: char, index: usize) -> Self {
		Self::new(StringDiffOpKind::Insert(c), index)
	}

	pub fn new_substitute(b: char, c: char, index: usize) -> Self {
		Self::new(StringDiffOpKind::Substitute(b, c), index)
	}
}



pub trait StringDiffAlgorithm {
	fn diff<'a>(&self, s1: &'a str, s2: &'a str) -> Vec<StringDiffOp>;
	fn distance<'a>(&self, s1: &'a str, s2: &'a str) -> usize;
}

#![doc = include_str!("../README.md")]
mod apply_diff;
pub use crate::apply_diff::apply_diff;
mod hamming;
pub use crate::hamming::hamming;
mod levenshtein;
pub use crate::levenshtein::levenshtein;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum StringDiffOpKind {
	Substitute(char, char),
	Insert(char),
	Delete,
	Transpose,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StringDiffOp {
	pub kind: StringDiffOpKind,
	pub index: usize,
}

impl StringDiffOp {
	pub fn new(kind: StringDiffOpKind, index: usize) -> Self {
		Self { kind, index }
	}

	pub fn new_delete(index: usize) -> Self {
		Self::new(StringDiffOpKind::Delete, index)
	}

	pub fn new_insert(c: char, index: usize) -> Self {
		Self::new(StringDiffOpKind::Insert(c), index)
	}

	pub fn new_substitute(b: char, c: char, index: usize) -> Self {
		Self::new(StringDiffOpKind::Substitute(b, c), index)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Diff {
	pub ops: Box<[StringDiffOp]>,
	pub total_len: usize,
}

impl Diff {
	pub fn new(diffs: Vec<StringDiffOp>, total_len: usize) -> Self {
		Self {
			ops: diffs.into_boxed_slice(),
			total_len: total_len,
		}
	}

	pub fn distance(&self) -> usize {
		self.ops.len()
	}
}

pub(crate) fn get_operation_matrix(
	s1: &str,
	s2: &str,
	dist_with_dir: fn(isize, isize, isize) -> (isize, char),
	init_vec: fn(&mut Vec<Vec<isize>>, usize, usize),
	char_match: isize,
	not_char_match: isize,
	indent_cost: isize,
) -> Vec<Vec<char>> {
	let first_string_len: usize = s1.len();
	let second_string_len: usize = s2.len();

	let mut dist_vector = vec![vec![0isize; first_string_len + 1]; second_string_len + 1];
	let mut dir_vector: Vec<Vec<char>> =
		vec![vec![' '; first_string_len + 1]; second_string_len + 1];

	init_vec(
		&mut dist_vector,
		first_string_len + 1,
		second_string_len + 1,
	);

	dir_vector[0][0] = '\\';
	for j in 1..second_string_len + 1 {
		dir_vector[j][0] = '^';
	}
	for i in 1..first_string_len + 1 {
		dir_vector[0][i] = '<';
	}

	for i in 1..second_string_len + 1 {
		for j in 1..first_string_len + 1 {
			let diagonal_gap_cost: isize;
			if s1.chars().nth(j - 1).unwrap() == s2.chars().nth(i - 1).unwrap() {
				diagonal_gap_cost = char_match;
			} else {
				diagonal_gap_cost = not_char_match;
			}
			(dist_vector[i][j], dir_vector[i][j]) = dist_with_dir(
				dist_vector[i - 1][j] + indent_cost, //deletion
				dist_vector[i][j - 1] + indent_cost, //insertion
				dist_vector[i - 1][j - 1] + diagonal_gap_cost,
			); //substitution
		}
	}
	dir_vector
}

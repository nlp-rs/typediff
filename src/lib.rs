use core::panic;
use std::cmp;
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

pub(crate) fn remove(start: usize, stop: usize, s: &str) -> String {
	assert!(stop > start);
	let mut result = String::from("");
	for (i, c) in s.chars().enumerate() {
		if start > i || stop < i + 1 {
			result.push(c);
		}
	}
	result
}

pub fn apply_diff(s: &str, diffs: Vec<StringDiffOp>) -> String {
	let mut new_string: String = s.into();

	for i in diffs.iter() {
		match i.kind {
			StringDiffOpKind::Delete(_x) => {
				new_string = remove(i.index, i.index + 1, &new_string);
			}
			StringDiffOpKind::Insert(_x) => {
				new_string.push(_x);
			}
			StringDiffOpKind::Substitute(_x, _y) => {
				new_string.replace_range((i.index)..(i.index + 1), &_y.to_string())
			}
			StringDiffOpKind::Transpose(_x, _y) => {
				panic!("apply_diff does not currently support the transpose operation yet")
			}
		}
	}
	new_string.clone()
}

pub trait StringDiffAlgorithm {
	fn diff<'a>(&self, s1: &'a str, s2: &'a str) -> Vec<StringDiffOp>;
	fn distance<'a>(&self, s1: &'a str, s2: &'a str) -> usize;
}

pub struct HammingDistance {}
impl StringDiffAlgorithm for HammingDistance {
	fn diff<'a>(&self, s1: &'a str, s2: &'a str) -> Vec<StringDiffOp> {
		if s1.len() != s2.len() {
			panic!("Strings must be same length");
		} else {
			let mut opp_vec: Vec<StringDiffOp> = Vec::new();
			for i in 0..s1.len() {
				if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap() {
					let new_opp = StringDiffOp::new_substitute
						(s1.chars().nth(i).unwrap(),
						s2.chars().nth(i).unwrap(),
						i);
					opp_vec.push(new_opp)
				}
			}
		    opp_vec
		}
	}

	fn distance<'a>(&self, s1: &'a str, s2: &'a str) -> usize {
		if s1.len() != s2.len() {
			panic!("Strings must be same length");
		} else {
			let mut edit_distance: usize = 0;
			for i in 0..s1.len() {
				if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap() {
					edit_distance += 1;
				}
			}
			edit_distance
		}
	}
}

pub struct LevenshteinDistance {}
impl LevenshteinDistance {
	pub(crate) fn min_dist(x: usize, y: usize, z: usize) -> usize {
		cmp::min(x, cmp::min(y, z))
	}
	/// At a given (x,y) we must choose the minimum value between a cells
	/// Top, Left, and Diagonal value. Depending on which cell is chosen between
	/// the three it will tell us if its a deletion, insertion or substitution operation.
	/// if we chooze x(The value above the cell) as the min value its a insertion operation (symbolized by '^')
	/// if we choose y(The value left of the cell) as the min value its a deletion operation(symbolized by '<')
	/// if we choose z(The value diagnal of the cell) as the min value its a substitution operation( sybmolized by '\' )
	/// we should always return either x,y,z if somehow we dont we panic with the unrechable macro.
	pub(crate) fn min_dist_with_dir(x: usize, y: usize, z: usize) -> (usize, char) {
		if x <= y && x <= z {
			return (x, '^');
		}
		if y <= x && y <= z {
			return (y, '<');
		}
		if z <= x && z <= y {
			return (z, '\\');
		}
		unreachable!()
	}

	pub(crate) fn print_vector<T: std::fmt::Debug>(my_vector: &[T]) {
		for i in my_vector.iter() {
			println!("{:?}", i);
		}
	}
	pub(crate) fn reverse_vec_and_indexes(
		my_vec: &mut Vec<StringDiffOp>,
		mut top_string_len: usize,
	) {
		my_vec.reverse();
		for i in my_vec.iter_mut() {
			i.index = top_string_len;
			top_string_len += 1;
		}
	}
	pub(crate) fn get_operations(
		my_opp: &Vec<Vec<char>>,
		left_string: &str,
		top_string: &str,
	) -> Vec<StringDiffOp> {
		let mut diff_ops: Vec<StringDiffOp> = Vec::new();
		let mut top_str_len = top_string.len();
		let mut left_str_len = left_string.len();
		let mut prev_char: char = ' ';

		loop {

			if top_str_len == 0 && left_str_len == 0 {
				break;
			}

			//Rows               Columns
			match my_opp[left_str_len][top_str_len] {
				//insertion
				'^' => {
					let insertion_op = StringDiffOp::new_insert(
						left_string.chars().nth(left_str_len - 1).unwrap(),
						0,
					);

					left_str_len -= 1;
					diff_ops.push(insertion_op);
					prev_char = '^';
				}
				//substitution
				'\\' => {
					if prev_char == '^' {
						Self::reverse_vec_and_indexes(&mut diff_ops, top_str_len);
					}

					if left_string.chars().nth(left_str_len - 1).unwrap()
						!= top_string.chars().nth(top_str_len - 1).unwrap()
					{
						let substitution_op = StringDiffOp::new_substitute(
							top_string.chars().nth(top_str_len - 1).unwrap(),
							left_string.chars().nth(left_str_len - 1).unwrap(),
							top_str_len - 1,
						);

						diff_ops.push(substitution_op);
					}
					left_str_len -= 1;
					top_str_len -= 1;
					prev_char = '\\';
				}
				//deletion
				'<' => {
					if prev_char == '^' {
						Self::reverse_vec_and_indexes(&mut diff_ops, top_str_len)
					}

					let deletion_op = StringDiffOp::new_delete(
						top_string.chars().nth(top_str_len - 1).unwrap(),
						top_str_len - 1,
					);

					top_str_len -= 1;
					diff_ops.push(deletion_op);
					prev_char = '<';
				}
				_ => {
					panic!("UNRECOGNIZED SYMBOL OPERATION !")
				}
			}
		}

		diff_ops
	}
}
impl StringDiffAlgorithm for LevenshteinDistance {
	fn diff<'a>(&self, s1: &'a str, s2: &'a str) -> Vec<StringDiffOp> {
		let first_string_len: usize = s1.len();
		let second_string_len: usize = s2.len();

		let mut dist_vector = vec![vec![0usize; first_string_len + 1]; second_string_len + 1];
		let mut dir_vector: Vec<Vec<char>> =
			vec![vec![' '; first_string_len + 1]; second_string_len + 1];

		for i in 0..first_string_len + 1 {
			dist_vector[0][i] = i;
		}
		for j in 0..second_string_len + 1 {
			dist_vector[j][0] = j;
		}

		dir_vector[0][0] = '\\';
		for j in 1..second_string_len + 1 {
			dir_vector[j][0] = '^';
		}
		for i in 1..first_string_len + 1 {
			dir_vector[0][i] = '<';
		}

		let mut sub_cost: usize = 0;
		for i in 1..second_string_len + 1 {
			for j in 1..first_string_len + 1 {
				if s1.chars().nth(j - 1).unwrap() == s2.chars().nth(i - 1).unwrap() {
					sub_cost = 0;
				} else {
					sub_cost = 1;
				}
				(dist_vector[i][j], dir_vector[i][j]) = LevenshteinDistance::min_dist_with_dir(
					dist_vector[i - 1][j] + 1, //deletion
					dist_vector[i][j - 1] + 1, //insertion
					dist_vector[i - 1][j - 1] + sub_cost,
				); //substitution
			}
		}

		LevenshteinDistance::get_operations(&dir_vector, s2, s1)
	}
	fn distance<'a>(&self, s1: &'a str, s2: &'a str) -> usize {
		let first_string_len: usize = s1.len();
		let second_string_len: usize = s2.len();

		let mut dist_vector = vec![vec![0usize; first_string_len + 1]; second_string_len + 1];

		for i in 0..first_string_len + 1 {
			dist_vector[0][i] = i;
		}

		for i in 0..second_string_len + 1 {
			dist_vector[i][0] = i;
		}

		let mut sub_cost: usize = 0;
		for i in 1..second_string_len + 1 {
			for j in 1..first_string_len + 1 {
				if s1.chars().nth(j - 1).unwrap() == s2.chars().nth(i - 1).unwrap() {
					sub_cost = 0;
				} else {
					sub_cost = 1;
				}
				dist_vector[i][j] = LevenshteinDistance::min_dist(
					dist_vector[i - 1][j] + 1,
					dist_vector[i][j - 1] + 1,
					dist_vector[i - 1][j - 1] + sub_cost,
				);
			}
		}
		dist_vector[second_string_len][first_string_len]
	}
}

#[cfg(test)]
mod dcode_tests {

	fn vec_compare(va: &Vec<StringDiffOp>, vb: &Vec<StringDiffOp>) -> bool {
		(va.len() == vb.len()) &&  // zip stops at the shortest
        va.iter()
        .zip(vb)
        .all(|(a,b)| a == b)
	}

	use crate::{HammingDistance, StringDiffAlgorithm, StringDiffOp};

	#[test]
	fn test_levenshtein_distance_edit_distance() {
		let test_struct = super::LevenshteinDistance {};

		assert_eq!(3, test_struct.distance("reset", "sets"));
		assert_eq!(3, test_struct.distance("kitten", "sitting"));
		assert_eq!(3, test_struct.distance("Saturday", "Sunday"));
	}

	#[test]
	fn test_levenshtein_distance_op_distance() {
		let test_struct = super::LevenshteinDistance {};

		let mut test_vec: Vec<StringDiffOp> = Vec::new();
		test_vec.push(super::StringDiffOp::new_insert('g', 6));
		test_vec.push(super::StringDiffOp::new_substitute('e', 'i', 4));
		test_vec.push(super::StringDiffOp::new_substitute('k', 's', 0));

		let mut test_vec_2: Vec<StringDiffOp> = Vec::new();
		test_vec_2.push(super::StringDiffOp::new_substitute('r', 'n', 4));
		test_vec_2.push(super::StringDiffOp::new_delete('t', 2));
		test_vec_2.push(super::StringDiffOp::new_delete('a', 1));

		let mut test_vec_3: Vec<StringDiffOp> = Vec::new();
		test_vec_3.push(super::StringDiffOp::new_insert('S', 5));
		test_vec_3.push(super::StringDiffOp::new_delete('E', 1));
		test_vec_3.push(super::StringDiffOp::new_delete('R', 0));

		let mut test_vec_4: Vec<StringDiffOp> = Vec::new();
		test_vec_4.push(super::StringDiffOp::new_insert('E', 5));
		test_vec_4.push(super::StringDiffOp::new_insert('R', 6));

		assert_eq!(
			vec_compare(&test_vec, &test_struct.diff("kitten", "sitting")),
			true
		);
		assert_eq!(
			vec_compare(&test_vec_2, &test_struct.diff("Saturday", "Sunday")),
			true
		);
		assert_eq!(
			vec_compare(&test_vec_3, &test_struct.diff("RESET", "SETS")),
			true
		);
		assert_eq!(
			vec_compare(&test_vec_4, &test_struct.diff("RESET", "RESETER")),
			true
		);
	}

	#[test]
	fn test_hamming_distance_edit_distance() {
		let test_struct = super::HammingDistance {};

		assert_eq!(3, test_struct.distance("karolin", "kathrin"));
		assert_eq!(3, test_struct.distance("karolin", "kerstin"));
		assert_eq!(4, test_struct.distance("kathrin", "kerstin"));
		assert_eq!(4, test_struct.distance("0000", "1111"));
		assert_eq!(3, test_struct.distance("2173896", "2233796"));
	}

	#[test]
	fn test_hamming_distance_op_distance() {
		let test_struct = super::HammingDistance {};

		let mut test_vec: Vec<StringDiffOp> = Vec::new();
		test_vec.push(StringDiffOp::new_substitute('r', 't', 2));
		test_vec.push(StringDiffOp::new_substitute('o', 'h', 3));
		test_vec.push(StringDiffOp::new_substitute('l', 'r', 4));

		let mut test_vec_2: Vec<StringDiffOp> = Vec::new();
		test_vec_2.push(StringDiffOp::new_substitute('a', 'e', 1));
		test_vec_2.push(StringDiffOp::new_substitute('o', 's', 3));
		test_vec_2.push(StringDiffOp::new_substitute('l', 't', 4));

		let mut test_vec_3: Vec<StringDiffOp> = Vec::new();
		test_vec_3.push(StringDiffOp::new_substitute('a', 'e', 1));
		test_vec_3.push(StringDiffOp::new_substitute('t', 'r', 2));
		test_vec_3.push(StringDiffOp::new_substitute('h', 's', 3));
		test_vec_3.push(StringDiffOp::new_substitute('r', 't', 4));

		let mut test_vec_4: Vec<StringDiffOp> = Vec::new();
		test_vec_4.push(StringDiffOp::new_substitute('0', '1', 0));
		test_vec_4.push(StringDiffOp::new_substitute('0', '1', 1));
		test_vec_4.push(StringDiffOp::new_substitute('0', '1', 2));
		test_vec_4.push(StringDiffOp::new_substitute('0', '1', 3));

		let mut test_vec_5: Vec<StringDiffOp> = Vec::new();
		test_vec_5.push(StringDiffOp::new_substitute('1', '2', 1));
		test_vec_5.push(StringDiffOp::new_substitute('7', '3', 2));
		test_vec_5.push(StringDiffOp::new_substitute('8', '7', 4));

		assert_eq!(
			vec_compare(&test_vec, &test_struct.diff("karolin", "kathrin")),
			true
		);
		assert_eq!(
			vec_compare(&test_vec_2, &test_struct.diff("karolin", "kerstin")),
			true
		);
		assert_eq!(
			vec_compare(&test_vec_3, &test_struct.diff("kathrin", "kerstin")),
			true
		);
		assert_eq!(
			vec_compare(&test_vec_4, &test_struct.diff("0000", "1111")),
			true
		);
		assert_eq!(
			vec_compare(&test_vec_5, &test_struct.diff("2173896", "2233796")),
			true
		);
	}

	#[test]
	fn test_apply_diffs() {
		let mut test_vec: Vec<StringDiffOp> = Vec::new();
		test_vec.push(StringDiffOp::new_insert('g', 0));
		test_vec.push(StringDiffOp::new_substitute('e', 'i', 4));
		test_vec.push(StringDiffOp::new_substitute('k', 's', 0));

		let mut test_vec_2: Vec<StringDiffOp> = Vec::new();
		test_vec_2.push(StringDiffOp::new_substitute('r', 'n', 4));
		test_vec_2.push(StringDiffOp::new_delete('t', 2));
		test_vec_2.push(StringDiffOp::new_delete('a', 1));

		let mut test_vec_3: Vec<StringDiffOp> = Vec::new();
		test_vec_3.push(StringDiffOp::new_insert('S', 5));
		test_vec_3.push(StringDiffOp::new_delete('E', 1));
		test_vec_3.push(StringDiffOp::new_delete('R', 0));

		assert_eq!(
			String::from("sitting"),
			super::apply_diff("kitten", test_vec)
		);
		assert_eq!(
			String::from("Sunday"),
			super::apply_diff("Saturday", test_vec_2)
		);
		assert_eq!(String::from("SETS"), super::apply_diff("RESET", test_vec_3));
	}
}

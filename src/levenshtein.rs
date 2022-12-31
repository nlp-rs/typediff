use crate::{StringDiffAlgorithm, StringDiffOp};
use core::panic;
use std::cmp;

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
	pub(crate) fn get_operation_matrix(s1: &str, s2: &str) -> Vec<Vec<char>> {
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
		dir_vector
	}
}
impl StringDiffAlgorithm for LevenshteinDistance {
	fn diff<'a>(&self, s1: &'a str, s2: &'a str) -> Vec<StringDiffOp> {
		let dir_matrix = LevenshteinDistance::get_operation_matrix(s1, s2);
		LevenshteinDistance::get_operations(&dir_matrix, s2, s1)
	}
	fn distance<'a>(&self, s1: &'a str, s2: &'a str) -> usize {
		let dir_matrix = LevenshteinDistance::get_operation_matrix(s1, s2);
		LevenshteinDistance::get_operations(&dir_matrix, s2, s1).len()
	}
}

#[cfg(test)]
mod tests {

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
}

use crate::diff::Diff;
use crate::{get_operation_matrix, StringDiffOp};
use core::panic;

pub(crate) fn reverse_vec_and_indexes(my_vec: &mut Vec<StringDiffOp>, mut top_string_len: usize) {
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
				let insertion_op =
					StringDiffOp::new_insert(left_string.chars().nth(left_str_len - 1).unwrap(), 0);

				left_str_len -= 1;
				diff_ops.push(insertion_op);
				prev_char = '^';
			}
			//substitution
			'\\' => {
				if prev_char == '^' {
					reverse_vec_and_indexes(&mut diff_ops, top_str_len);
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
					reverse_vec_and_indexes(&mut diff_ops, top_str_len)
				}

				let deletion_op = StringDiffOp::new_delete(top_str_len - 1);

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

/// At a given (x,y) we must choose the minimum value between a cells
/// Top, Left, and Diagonal value. Depending on which cell is chosen between
/// the three it will tell us if its a deletion, insertion or substitution operation.
/// if we chooze x(The value above the cell) as the min value its a insertion operation (symbolized by '^')
/// if we choose y(The value left of the cell) as the min value its a deletion operation(symbolized by '<')
/// if we choose z(The value diagnal of the cell) as the min value its a substitution operation( sybmolized by '\' )
/// we should always return either x,y,z if somehow we dont we panic with the unrechable macro.
pub(crate) fn min_dist_with_dir(x: isize, y: isize, z: isize) -> (isize, char) {
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

pub(crate) fn my_init_vec(my_vec: &mut Vec<Vec<isize>>, top_str_len: usize, left_str_len: usize) {
	for i in 0..top_str_len {
		my_vec[0][i] = i as isize;
	}
	for j in 0..left_str_len {
		my_vec[j][0] = j as isize;
	}
}

pub fn levenshtein<'a>(s1: &'a str, s2: &'a str) -> Diff {
	let dir_matrix = get_operation_matrix(s1, s2, min_dist_with_dir, my_init_vec, 0, 1, 1);
	let temp = get_operations(&dir_matrix, s2, s1).clone();
	let val: usize = if s1.len() >= s2.len() {
		s1.len()
	} else {
		s2.len()
	};

	Diff::new(temp, val)
}

#[cfg(test)]
mod tests {

	#[test]
	fn test_levenshtein_distance_op_distance() {
		use crate::diff::Diff;
		use crate::levenshtein::levenshtein;

		let test_diff = Diff {
			ops: Box::new([
				super::StringDiffOp::new_insert('g', 6),
				super::StringDiffOp::new_substitute('e', 'i', 4),
				super::StringDiffOp::new_substitute('k', 's', 0),
			]),
			total_len: 7,
		};

		let test_diff_2 = Diff {
			ops: Box::new([
				super::StringDiffOp::new_substitute('r', 'n', 4),
				super::StringDiffOp::new_delete(2),
				super::StringDiffOp::new_delete(1),
			]),
			total_len: 8,
		};

		let test_diff_3 = Diff {
			ops: Box::new([
				super::StringDiffOp::new_insert('S', 5),
				super::StringDiffOp::new_delete(1),
				super::StringDiffOp::new_delete(0),
			]),
			total_len: 5,
		};

		let test_diff_4 = Diff {
			ops: Box::new([
				super::StringDiffOp::new_insert('E', 5),
				super::StringDiffOp::new_insert('R', 6),
			]),
			total_len: 7,
		};

		assert_eq!(test_diff, levenshtein("kitten", "sitting"));
		assert_eq!(test_diff_2, levenshtein("Saturday", "Sunday"));
		assert_eq!(test_diff_3, levenshtein("RESET", "SETS"));
		assert_eq!(test_diff_4, levenshtein("RESET", "RESETER"));
	}
}

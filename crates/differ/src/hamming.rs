use crate::{Diff, StringDiffOp};
use std::iter::zip;

pub fn hamming<'a>(s1: &'a str, s2: &'a str) -> Diff {
	if s1.len() != s2.len() {
		panic!("Strings must be same length");
	}

	let mut opp_vec: Vec<StringDiffOp> = Vec::new();
	let iter = zip(s1.chars(), s2.chars());

	for (i, (char1, char2)) in iter.enumerate() {
		if char1 != char2 {
			opp_vec.push(StringDiffOp::new_substitute(char1, char2, i));
		}
	}
	Diff::new(opp_vec, s1.len())
}

#[cfg(test)]
mod tests {
	use crate::StringDiffOp;

	#[test]
	fn test_hamming_distance_op_distance() {
		use crate::hamming::hamming;
		use crate::Diff;

		let test_diff = Diff {
			ops: Box::new([
				StringDiffOp::new_substitute('r', 't', 2),
				StringDiffOp::new_substitute('o', 'h', 3),
				StringDiffOp::new_substitute('l', 'r', 4),
			]),
			total_len: 7,
		};

		let test_diff_2 = Diff {
			ops: Box::new([
				StringDiffOp::new_substitute('a', 'e', 1),
				StringDiffOp::new_substitute('o', 's', 3),
				StringDiffOp::new_substitute('l', 't', 4),
			]),
			total_len: 7,
		};

		let test_diff_3 = Diff {
			ops: Box::new([
				StringDiffOp::new_substitute('a', 'e', 1),
				StringDiffOp::new_substitute('t', 'r', 2),
				StringDiffOp::new_substitute('h', 's', 3),
				StringDiffOp::new_substitute('r', 't', 4),
			]),
			total_len: 7,
		};

		let test_diff_4 = Diff {
			ops: Box::new([
				StringDiffOp::new_substitute('0', '1', 0),
				StringDiffOp::new_substitute('0', '1', 1),
				StringDiffOp::new_substitute('0', '1', 2),
				StringDiffOp::new_substitute('0', '1', 3),
			]),
			total_len: 4,
		};

		let test_diff_5 = Diff {
			ops: Box::new([
				StringDiffOp::new_substitute('1', '2', 1),
				StringDiffOp::new_substitute('7', '3', 2),
				StringDiffOp::new_substitute('8', '7', 4),
			]),
			total_len: 7,
		};

		assert_eq!(test_diff, hamming("karolin", "kathrin"));
		assert_eq!(test_diff_2, hamming("karolin", "kerstin"));
		assert_eq!(test_diff_3, hamming("kathrin", "kerstin"));
		assert_eq!(test_diff_4, hamming("0000", "1111"));
		assert_eq!(test_diff_5, hamming("2173896", "2233796"));
	}
}

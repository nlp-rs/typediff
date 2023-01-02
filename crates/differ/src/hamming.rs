use crate::{StringDiffAlgorithm, StringDiffOp};
use std::iter::zip;

pub struct HammingDistance {}
impl StringDiffAlgorithm for HammingDistance {
	fn diff<'a>(&self, s1: &'a str, s2: &'a str) -> Vec<StringDiffOp> {
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
		opp_vec
	}

	fn distance<'a>(&self, s1: &'a str, s2: &'a str) -> usize {
		self.diff(s1, s2).len()
	}
}

#[cfg(test)]
mod tests {
	use crate::{StringDiffAlgorithm, StringDiffOp};

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

		assert_eq!(&test_vec, &test_struct.diff("karolin", "kathrin"));
		assert_eq!(&test_vec_2, &test_struct.diff("karolin", "kerstin"));
		assert_eq!(&test_vec_3, &test_struct.diff("kathrin", "kerstin"));
		assert_eq!(&test_vec_4, &test_struct.diff("0000", "1111"));
		assert_eq!(&test_vec_5, &test_struct.diff("2173896", "2233796"));
	}
}

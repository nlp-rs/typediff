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

	#[test]
	fn test_hamming_similarity() {
		use crate::hamming::hamming;
		use crate::DiffScoreConfig;

		let sim = hamming("karolin", "kathrin");
		let config = DiffScoreConfig::default();
		let similarity = ((7.0) - 3.0) / (7.0);

		let sim_v2 = hamming("karolin", "kerstin");
		let mut config_v2 = DiffScoreConfig::default();
		config_v2.sub_cost = 0.5;
		let similarity_v2 = ((7.0) - 1.5) / (7.0);

		let sim_v3 = hamming("kathrin", "kerstin");
		let mut config_v3 = DiffScoreConfig::default();
		config_v3.sub_cost = 1.5;
		let similarity_v3 = ((7.0) - 6.0) / (7.0);

		assert_eq!(similarity, sim.similarity(&config));
		assert_eq!(similarity_v2, sim_v2.similarity(&config_v2));
		assert_eq!(similarity_v3, sim_v3.similarity(&config_v3));
	}

	#[test]
	fn test_hamming_difference() {
		use crate::hamming::hamming;
		use crate::DiffScoreConfig;

		let diff = hamming("karolin", "kathrin");
		let config = DiffScoreConfig::default();
		let difference = ((7.0) - 3.0) / (7.0);

		let diff_v2 = hamming("karolin", "kerstin");
		let mut config_v2 = DiffScoreConfig::default();
		config_v2.sub_cost = 0.5;
		let difference_v2 = ((7.0) - 1.5) / (7.0);

		let diff_v3 = hamming("kathrin", "kerstin");
		let mut config_v3 = DiffScoreConfig::default();
		config_v3.sub_cost = 1.5;
		let difference_v3 = ((7.0) - 6.0) / (7.0);

		assert_eq!(difference, diff.similarity(&config));
		assert_eq!(difference_v2, diff_v2.similarity(&config_v2));
		assert_eq!(difference_v3, diff_v3.similarity(&config_v3));
	}
}

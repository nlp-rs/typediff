use crate::StringAlignAlgorithm;
pub struct NeedlemanWunsch {}

impl NeedlemanWunsch {
	pub(crate) fn debug_vec<T: std::fmt::Debug>(my_vector: &Vec<Vec<T>>) {
		for i in my_vector.iter() {
			println!("{:?}", i);
		}
	}
	pub(crate) fn max_dist_with_dir(x: isize, y: isize, z: isize) -> (isize, char) {
		if x >= y && x >= z {
			return (x, '^');
		}
		if y >= x && y >= z {
			return (y, '<');
		}
		if z >= x && z >= y {
			return (z, '\\');
		}
		unreachable!()
	}
	pub(crate) fn align_strings<'a>(
		s1: &'a str,
		s2: &'a str,
		dir_vector: &Vec<Vec<char>>,
	) -> (String, String) {
		let mut top_str_len = s1.len();
		let mut left_str_len = s2.len();

		let mut top_string: String = String::new();
		let mut left_string: String = String::new();

		loop {
			if top_str_len == 0 && left_str_len == 0 {
				break;
			}
			match dir_vector[left_str_len][top_str_len] {
				//Vertical arrows will align a gap ("-") to the letter of the row
				'^' => {
					top_string.insert(0, '-');
					left_string.insert(0, s2.chars().nth(left_str_len - 1).unwrap());
					left_str_len -= 1;
				}
				//horizontal arrows will align a gap to the letter of the column
				'<' => {
					top_string.insert(0, s1.chars().nth(top_str_len - 1).unwrap());
					left_string.insert(0, '-');
					top_str_len -= 1;
				}

				'\\' => {
					top_string.insert(0, s1.chars().nth(top_str_len - 1).unwrap());
					left_string.insert(0, s2.chars().nth(left_str_len - 1).unwrap());
					top_str_len -= 1;
					left_str_len -= 1;
				}
				_ => {
					panic!("UNRECOGNIZED SYMBOL OPERATION !")
				}
			}
		}

		(top_string.to_owned(), left_string.to_owned())
	}
}

impl StringAlignAlgorithm for NeedlemanWunsch {
	fn align<'a>(&self, s1: &'a str, s2: &'a str) -> (String, String) {
		let top_str_len: usize = s1.len();
		let left_str_len: usize = s2.len();

		let mut align_vector: Vec<Vec<isize>> =
			vec![vec![0isize; top_str_len + 1]; left_str_len + 1];
		let mut dir_vector: Vec<Vec<char>> = vec![vec![' '; top_str_len + 1]; left_str_len + 1];

		for i in 0..top_str_len + 1 {
			align_vector[0][i] = -(i as isize);
		}
		for i in 0..left_str_len + 1 {
			align_vector[i][0] = -(i as isize);
		}

		dir_vector[0][0] = '\\';
		for j in 1..left_str_len + 1 {
			dir_vector[j][0] = '^';
		}
		for i in 1..top_str_len + 1 {
			dir_vector[0][i] = '<';
		}
		//Match: +1
		//Mismatch or Indel: −1
		for i in 1..left_str_len + 1 {
			for j in 1..top_str_len + 1 {
				let mut diagnal_cost = 0;
				if s1.chars().nth(j - 1).unwrap() == s2.chars().nth(i - 1).unwrap() {
					diagnal_cost = 1;
				} else {
					diagnal_cost = -1
				}

				(align_vector[i][j], dir_vector[i][j]) = NeedlemanWunsch::max_dist_with_dir(
					align_vector[i - 1][j] + (-1),
					align_vector[i][j - 1] + (-1),
					align_vector[i - 1][j - 1] + (diagnal_cost),
				)
			}
		}
		NeedlemanWunsch::debug_vec(&align_vector);
		NeedlemanWunsch::debug_vec(&dir_vector);
		let (test, test2) = NeedlemanWunsch::align_strings(s1, s2, &dir_vector);

		return (test, test2);

		//("str", "str")
	}
}

#[cfg(test)]
mod tests {

	use crate::StringAlignAlgorithm;
	#[test]
	fn test_needlemanwunsch_align() {
		let test_struct = super::NeedlemanWunsch {};

		assert_eq!(
			test_struct.align("GCATGCG", "GATTACA"),
			(String::from("GCATG-CG"), String::from("G-ATTACA"))
		);

		assert_eq!(
			test_struct.align("CAGTG", "ATCTC"),
			(String::from("CAG-TG"), String::from("-ATCTC"))
		);
	}
}

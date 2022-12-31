use crate::{StringDiffOp, StringDiffOpKind};

pub(crate) fn remove(start: usize, stop: usize, s: &str) -> String {
	assert!(stop > start);
	let mut result = String::new();
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

#[cfg(test)]
mod tests {
	use crate::StringDiffOp;

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

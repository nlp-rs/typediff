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
			StringDiffOpKind::Delete => {
				new_string = remove(i.index, i.index + 1, &new_string);
			}
			StringDiffOpKind::Insert(_x) => new_string.insert(i.index, _x),
			StringDiffOpKind::Substitute(_x, _y) => {
				new_string.replace_range((i.index)..(i.index + 1), &_y.to_string())
			}
			StringDiffOpKind::Transpose => {
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
		let test_vec: Vec<StringDiffOp> = vec![
			StringDiffOp::new_insert('g', 6),
			StringDiffOp::new_substitute('e', 'i', 4),
			StringDiffOp::new_substitute('k', 's', 0),
		];

		let test_vec_2: Vec<StringDiffOp> = vec![
			StringDiffOp::new_substitute('r', 'n', 4),
			StringDiffOp::new_delete(2),
			StringDiffOp::new_delete(1),
		];

		let test_vec_3: Vec<StringDiffOp> = vec![
			StringDiffOp::new_insert('S', 5),
			StringDiffOp::new_delete(1),
			StringDiffOp::new_delete(0),
		];

		let test_vec_4 = vec![
			StringDiffOp::new_insert('e', 1),
			StringDiffOp::new_insert('o', 3),
		];

		let test_vec_5 = vec![
			StringDiffOp::new_insert('r', 4),
			StringDiffOp::new_insert('s', 0),
		];

		assert_eq!(
			String::from("sitting"),
			super::apply_diff("kitten", test_vec)
		);
		assert_eq!(
			String::from("Sunday"),
			super::apply_diff("Saturday", test_vec_2)
		);
		assert_eq!(String::from("SETS"), super::apply_diff("RESET", test_vec_3));
		assert_eq!(String::from("heeoy"), super::apply_diff("hey", test_vec_4));
		assert_eq!(
			String::from("skater"),
			super::apply_diff("kate", test_vec_5)
		);
	}
}

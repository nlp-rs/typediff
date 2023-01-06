pub struct DiffScoreConfig {
	pub sub_cost: f32,
	pub lowercase_sub_cost: f32,
	pub indel_cost: f32,
	pub transpose_cost: f32,
	// future properties here as needed
}

impl Default for DiffScoreConfig {
	fn default() -> Self {
		Self {
			sub_cost: 1.0,
			lowercase_sub_cost: 1.0,
			indel_cost: 1.0,
			transpose_cost: 1.0,
		}
	}
}

#[cfg(test)]
mod tests {

	#[test]
	fn test_default() {
		let test_struct = super::DiffScoreConfig::default();
		assert_eq!(test_struct.sub_cost, 1.0);
		assert_eq!(test_struct.lowercase_sub_cost, 1.0);
		assert_eq!(test_struct.indel_cost, 1.0);
		assert_eq!(test_struct.transpose_cost, 1.0);

		let mut test_struct = super::DiffScoreConfig::default();
		test_struct.sub_cost = 2.0;
		test_struct.lowercase_sub_cost = 2.0;
		test_struct.indel_cost = 2.0;
		test_struct.transpose_cost = 2.0;
		assert_eq!(test_struct.sub_cost, 2.0);
		assert_eq!(test_struct.lowercase_sub_cost, 2.0);
		assert_eq!(test_struct.indel_cost, 2.0);
		assert_eq!(test_struct.transpose_cost, 2.0);
	}
}

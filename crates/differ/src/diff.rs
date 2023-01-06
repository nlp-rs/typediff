use crate::StringDiffOp;

#[derive(Debug, PartialEq)]
pub struct Diff {
	pub ops: Box<[StringDiffOp]>,
	pub total_len: usize,
}

impl Diff {
	pub fn new(diffs: Vec<StringDiffOp>, total_len: usize) -> Self {
		Self {
			ops: diffs.into_boxed_slice(),
			total_len: total_len,
		}
	}

	pub fn distance(&self) -> usize {
		self.ops.len()
	}

	pub fn operations(&self) {
		for i in self.ops.iter() {
			println!("{:?}", i);
		}
	}
}

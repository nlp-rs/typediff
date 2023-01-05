mod needlemanwunsch;
pub use crate::needlemanwunsch::NeedlemanWunsch;

pub trait StringAlignAlgorithm {
	fn align<'a>(&self, s1: &'a str, s2: &'a str) -> (String, String);
}

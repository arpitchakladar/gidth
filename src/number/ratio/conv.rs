use crate::number::{
	Int,
	Ratio,
	One,
};

impl<T: Int> From<T> for Ratio<T> {
	fn from(n: T) -> Self {
		Self {
			num: n,
			den: One::one(),
		}
	}
}

use crate::number::{
	BigInt,
	Abs,
};

impl Abs for BigInt {
	#[inline]
	fn abs(mut self) -> BigInt {
		self.positive = true;

		self
	}
}

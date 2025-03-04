use crate::number::BigInt;
use crate::utils::Abs;

impl Abs for BigInt {
	#[inline]
	fn abs(mut self) -> BigInt {
		self.positive = true;

		self
	}
}

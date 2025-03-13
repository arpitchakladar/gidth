pub use inherent::inherent;

use crate::number::{
	BigInt,
	Abs,
};

#[inherent]
impl Abs for BigInt {
	#[inline]
	pub fn abs(mut self) -> BigInt {
		self.positive = true;

		self
	}
}

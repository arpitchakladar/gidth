use inherent::inherent;

use crate::number::{
	BigDecimal,
	Abs,
};

#[inherent]
impl Abs for BigDecimal {
	#[inline]
	pub fn abs(mut self) -> BigDecimal {
		self.positive = true;

		self
	}
}

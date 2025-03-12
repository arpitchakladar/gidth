use crate::number::{
	BigDecimal,
	Abs,
};

impl Abs for BigDecimal {
	#[inline]
	fn abs(mut self) -> BigDecimal {
		self.positive = true;

		self
	}
}

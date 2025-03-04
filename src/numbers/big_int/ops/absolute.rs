use crate::numbers::BigInt;

impl BigInt {
	pub fn abs(self) -> BigInt {
		self.positive = true;

		self
	}
}

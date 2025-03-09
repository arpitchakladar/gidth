use crate::number::BigInt;

impl std::ops::Neg for BigInt {
	type Output = BigInt;

	fn neg(mut self) -> Self::Output {
		self.positive = !self.positive;

		self
	}
}

impl std::ops::Neg for &BigInt {
	type Output = BigInt;

	fn neg(self) -> Self::Output {
		-self.clone()
	}
}

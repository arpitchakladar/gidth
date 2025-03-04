use crate::numbers::BigInt;

impl std::ops::Neg for BigInt {
	type Output = BigInt;

	fn neg(self) -> Self::Output {
		let mut result = self;
		result.positive = !result.positive;

		result
	}
}

impl std::ops::Neg for &BigInt {
	type Output = BigInt;

	fn neg(self) -> Self::Output {
		-self.clone()
	}
}

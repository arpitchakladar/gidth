use crate::number::BigDecimal;

impl std::ops::Neg for BigDecimal {
	type Output = BigDecimal;

	fn neg(mut self) -> Self::Output {
		self.positive = !self.positive;

		self
	}
}

impl std::ops::Neg for &BigDecimal {
	type Output = BigDecimal;

	fn neg(self) -> Self::Output {
		-self.clone()
	}
}

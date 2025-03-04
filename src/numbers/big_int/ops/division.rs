use crate::numbers::BigInt;

impl std::ops::Div for &BigInt {
	type Output = BigInt;

	fn div(self, other: Self) -> Self::Output {
		BigInt::unsigned_divmod(self, other).0
	}
}

impl std::ops::Div for BigInt {
	type Output = BigInt;

	fn div(self, other: Self) -> Self::Output {
		&self / &other
	}
}

impl std::ops::Div<&BigInt> for BigInt {
	type Output = BigInt;

	fn div(self, other: &Self) -> Self::Output {
		&self / other
	}
}

impl std::ops::Div<BigInt> for &BigInt {
	type Output = BigInt;

	fn div(self, other: BigInt) -> Self::Output {
		self / &other
	}
}

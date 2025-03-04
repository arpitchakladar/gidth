use crate::number::BigInt;

impl std::ops::Rem for &BigInt {
	type Output = BigInt;

	fn rem(self, other: Self) -> Self::Output {
		BigInt::unsigned_divmod(self, other).1
	}
}

impl std::ops::Rem for BigInt {
	type Output = BigInt;

	fn rem(self, other: Self) -> Self::Output {
		&self % &other
	}
}

impl std::ops::Rem<&BigInt> for BigInt {
	type Output = BigInt;

	fn rem(self, other: &Self) -> Self::Output {
		&self % other
	}
}

impl std::ops::Rem<BigInt> for &BigInt {
	type Output = BigInt;

	fn rem(self, other: BigInt) -> Self::Output {
		self % &other
	}
}

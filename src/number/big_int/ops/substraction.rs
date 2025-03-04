use crate::number::BigInt;

impl std::ops::Sub for &BigInt {
	type Output = BigInt;

	fn sub(self, other: Self) -> Self::Output {
		match (self.positive, other.positive) {
			(true, true) => BigInt::unsigned_sub(self, other),
			(true, false) => BigInt::unsigned_add(self, other),
			(false, true) => {
				let mut result = BigInt::unsigned_add(self, other);
				result.positive = false;
				result
			},
			(false, false) => BigInt::unsigned_sub(other, self),
		}
	}
}

impl std::ops::Sub for BigInt {
	type Output = BigInt;

	fn sub(self, other: Self) -> Self::Output {
		&self - &other
	}
}

impl std::ops::Sub<&BigInt> for BigInt {
	type Output = BigInt;

	fn sub(self, other: &BigInt) -> Self::Output {
		&self - other
	}
}

impl std::ops::Sub<BigInt> for &BigInt {
	type Output = BigInt;

	fn sub(self, other: BigInt) -> Self::Output {
		self - &other
	}
}

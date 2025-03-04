use std::ops::Sub;
use crate::numbers::{BigInt, unsigned_big_int_add, unsigned_big_int_sub};

impl Sub for &BigInt {
	type Output = BigInt;

	fn sub(self, other: Self) -> Self::Output {
		match (self.positive, other.positive) {
			(true, true) => unsigned_big_int_sub(self, other),
			(true, false) => unsigned_big_int_add(self, other),
			(false, true) => {
				let mut result = unsigned_big_int_add(self, other);
				result.positive = false;
				result
			},
			(false, false) => unsigned_big_int_sub(other, self),
		}
	}
}

impl Sub for BigInt {
	type Output = BigInt;

	fn sub(self, other: Self) -> Self::Output {
		&self - &other
	}
}

impl Sub<&BigInt> for BigInt {
	type Output = BigInt;

	fn sub(self, other: &BigInt) -> Self::Output {
		&self - other
	}
}

impl Sub<BigInt> for &BigInt {
	type Output = BigInt;

	fn sub(self, other: BigInt) -> Self::Output {
		self - &other
	}
}

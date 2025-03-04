use std::ops::Mul;

use crate::numbers::{
	BigInt,
	unsigned_big_int_mul
};

impl Mul for &BigInt {
	type Output = BigInt;

	fn mul(self, other: Self) -> Self::Output {
		let mut result = unsigned_big_int_mul(self, other);
		result.positive = self.positive == other.positive;

		result
	}
}

impl Mul for BigInt {
	type Output = BigInt;

	fn mul(self, other: Self) -> Self::Output {
		&self * &other
	}
}

impl Mul<&BigInt> for BigInt {
	type Output = BigInt;

	fn mul(self, other: &Self) -> Self::Output {
		&self * other
	}
}

impl Mul<BigInt> for &BigInt {
	type Output = BigInt;

	fn mul(self, other: BigInt) -> Self::Output {
		self * &other
	}
}

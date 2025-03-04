use crate::numbers::{
	BigInt,
	unsigned_big_int_add,
	unsigned_big_int_sub,
};
use std::ops::Add;

impl Add for &BigInt {
	type Output = BigInt;

	fn add(self, other: Self) -> Self::Output {
		match (self.positive, other.positive) {
			(true, true) => unsigned_big_int_add(self, other),
			(true, false) => unsigned_big_int_sub(self, other),
			(false, true) => unsigned_big_int_sub(other, self),
			(false, false) => {
				let mut result = unsigned_big_int_add(self, other);
				result.positive = false;
				result
			},
		}
	}
}

impl Add for BigInt {
	type Output = BigInt;

	fn add(self, other: Self) -> Self::Output {
		&self + &other
	}
}

impl Add<&BigInt> for BigInt {
	type Output = BigInt;

	fn add(self, other: &BigInt) -> Self::Output {
		&self + other
	}
}

impl Add<BigInt> for &BigInt {
	type Output = BigInt;

	fn add(self, other: BigInt) -> Self::Output {
		self + &other
	}
}

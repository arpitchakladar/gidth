use std::ops::Mul;

use crate::numbers::{
	Integer,
	unsigned_integer_mul
};

impl Mul for &Integer {
	type Output = Integer;

	fn mul(self, other: Self) -> Self::Output {
		let mut result = unsigned_integer_mul(self, other);
		result.positive = (self.positive && other.positive) || !(self.positive || other.positive);

		result
	}
}

impl Mul for Integer {
	type Output = Integer;

	fn mul(self, other: Self) -> Self::Output {
		&self * &other
	}
}

impl Mul<&Integer> for Integer {
	type Output = Integer;

	fn mul(self, other: &Self) -> Self::Output {
		&self * other
	}
}

impl Mul<Integer> for &Integer {
	type Output = Integer;

	fn mul(self, other: Integer) -> Self::Output {
		self * &other
	}
}

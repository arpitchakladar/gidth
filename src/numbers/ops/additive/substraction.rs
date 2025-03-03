use std::ops::Sub;
use crate::numbers::{Integer, unsigned_integer_add, unsigned_integer_sub};

impl Sub for &Integer {
	type Output = Integer;

	fn sub(self, other: Self) -> Self::Output {
		match (self.positive, other.positive) {
			(true, true) => unsigned_integer_sub(self, other),
			(true, false) => unsigned_integer_add(self, other),
			(false, true) => {
				let mut result = unsigned_integer_add(self, other);
				result.positive = false;
				result
			}
			(false, false) => unsigned_integer_sub(other, self),
		}
	}
}

impl Sub for Integer {
	type Output = Integer;

	fn sub(self, other: Self) -> Self::Output {
		&self - &other
	}
}

impl Sub<&Integer> for Integer {
	type Output = Integer;

	fn sub(self, other: &Integer) -> Self::Output {
		&self - other
	}
}

impl Sub<Integer> for &Integer {
	type Output = Integer;

	fn sub(self, other: Integer) -> Self::Output {
		self - &other
	}
}

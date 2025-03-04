use crate::numbers::{
	Integer,
	unsigned_integer_add,
	unsigned_integer_sub,
};
use std::ops::Add;

impl Add for &Integer {
	type Output = Integer;

	fn add(self, other: Self) -> Self::Output {
		match (self.positive, other.positive) {
			(true, true) => unsigned_integer_add(self, other),
			(true, false) => unsigned_integer_sub(self, other),
			(false, true) => unsigned_integer_sub(other, self),
			(false, false) => {
				let mut result = unsigned_integer_add(self, other);
				result.positive = false;
				result
			},
		}
	}
}

impl Add for Integer {
	type Output = Integer;

	fn add(self, other: Self) -> Self::Output {
		&self + &other
	}
}

impl Add<&Integer> for Integer {
	type Output = Integer;

	fn add(self, other: &Integer) -> Self::Output {
		&self + other
	}
}

impl Add<Integer> for &Integer {
	type Output = Integer;

	fn add(self, other: Integer) -> Self::Output {
		self + &other
	}
}

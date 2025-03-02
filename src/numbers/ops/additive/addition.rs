use crate::numbers::Integer;

use crate::numbers::{
	unsigned_integer_add,
	unsigned_integer_sub,
};

impl std::ops::Add for &Integer {
	type Output = Integer;

	fn add(self, other: Self) -> Self::Output {
		if self.positive && other.positive {
			unsigned_integer_add(self, other)
		} else if self.positive && !other.positive {
			unsigned_integer_sub(self, other)
		} else if !self.positive && other.positive {
			unsigned_integer_sub(other, self)
		} else {
			let mut result = unsigned_integer_add(self, other);
			result.positive = false;
			result
		}
	}
}

impl std::ops::Add for Integer {
	type Output = Integer;


	fn add(self, other: Self) -> Self::Output {
		&self + &other
	}
}

impl std::ops::Add<&Integer> for Integer {
	type Output = Integer;


	fn add(self, other: &Self) -> Self::Output {
		&self + other
	}
}

impl std::ops::Add<Integer> for &Integer {
	type Output = Integer;


	fn add(self, other: Integer) -> Self::Output {
		self + &other
	}
}

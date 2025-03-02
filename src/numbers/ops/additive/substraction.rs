use crate::numbers::Integer;

use crate::numbers::{
	unsigned_integer_add,
	unsigned_integer_sub,
};

impl std::ops::Sub for &Integer {
	type Output = Integer;

	fn sub(self, other: Self) -> Self::Output {
		if self.positive && other.positive {
			unsigned_integer_sub(self, other)
		} else if self.positive && !other.positive {
			unsigned_integer_add(self, other)
		} else if !self.positive && other.positive {
			let mut result = unsigned_integer_add(self, other);
			result.positive = false;
			result
		} else {
			unsigned_integer_sub(other, self)
		}
	}
}

impl std::ops::Sub for Integer {
	type Output = Integer;


	fn sub(self, other: Self) -> Self::Output {
		&self - &other
	}
}

impl std::ops::Sub<&Integer> for Integer {
	type Output = Integer;


	fn sub(self, other: &Self) -> Self::Output {
		&self - other
	}
}

impl std::ops::Sub<Integer> for &Integer {
	type Output = Integer;


	fn sub(self, other: Integer) -> Self::Output {
		self - &other
	}
}

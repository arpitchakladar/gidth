use crate::numbers::Integer;

use crate::numbers::unsigned_integer_mul;

impl std::ops::Mul for &Integer {
	type Output = Integer;

	fn mul(self, other: Self) -> Self::Output {
		let mut result = unsigned_integer_mul(self, other);
		result.positive = (self.positive && other.positive) || !(self.positive || other.positive);

		result
	}
}

impl std::ops::Mul for Integer {
	type Output = Integer;

	fn mul(self, other: Self) -> Self::Output {
		&self * &other
	}
}

impl std::ops::Mul<&Integer> for Integer {
	type Output = Integer;

	fn mul(self, other: &Self) -> Self::Output {
		&self * other
	}
}

impl std::ops::Mul<Integer> for &Integer {
	type Output = Integer;

	fn mul(self, other: Integer) -> Self::Output {
		self * &other
	}
}

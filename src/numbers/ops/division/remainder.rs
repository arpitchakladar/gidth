use std::ops::Rem;

use crate::numbers::{
	Integer,
	unsigned_integer_divmod,
};

impl Rem for &Integer {
	type Output = Integer;

	fn rem(self, other: Self) -> Self::Output {
		unsigned_integer_divmod(self, other).1
	}
}

impl Rem for Integer {
	type Output = Integer;

	fn rem(self, other: Self) -> Self::Output {
		&self % &other
	}
}

impl Rem<&Integer> for Integer {
	type Output = Integer;

	fn rem(self, other: &Self) -> Self::Output {
		&self % other
	}
}

impl Rem<Integer> for &Integer {
	type Output = Integer;

	fn rem(self, other: Integer) -> Self::Output {
		self % &other
	}
}

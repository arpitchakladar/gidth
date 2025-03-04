use std::ops::Div;

use crate::numbers::{
	Integer,
	unsigned_integer_div,
};

impl Div for &Integer {
	type Output = Integer;

	fn div(self, other: Self) -> Self::Output {
		unsigned_integer_div(self, other)
	}
}

impl Div for Integer {
	type Output = Integer;

	fn div(self, other: Self) -> Self::Output {
		&self / &other
	}
}

impl Div<&Integer> for Integer {
	type Output = Integer;

	fn div(self, other: &Self) -> Self::Output {
		&self / other
	}
}

impl Div<Integer> for &Integer {
	type Output = Integer;

	fn div(self, other: Integer) -> Self::Output {
		self / &other
	}
}

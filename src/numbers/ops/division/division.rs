use std::ops::Div;

use crate::numbers::{
	BigInt,
	unsigned_big_int_divmod,
};

impl Div for &BigInt {
	type Output = BigInt;

	fn div(self, other: Self) -> Self::Output {
		unsigned_big_int_divmod(self, other).0
	}
}

impl Div for BigInt {
	type Output = BigInt;

	fn div(self, other: Self) -> Self::Output {
		&self / &other
	}
}

impl Div<&BigInt> for BigInt {
	type Output = BigInt;

	fn div(self, other: &Self) -> Self::Output {
		&self / other
	}
}

impl Div<BigInt> for &BigInt {
	type Output = BigInt;

	fn div(self, other: BigInt) -> Self::Output {
		self / &other
	}
}

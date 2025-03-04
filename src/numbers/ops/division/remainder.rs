use std::ops::Rem;

use crate::numbers::{
	BigInt,
	unsigned_big_int_divmod,
};

impl Rem for &BigInt {
	type Output = BigInt;

	fn rem(self, other: Self) -> Self::Output {
		unsigned_big_int_divmod(self, other).1
	}
}

impl Rem for BigInt {
	type Output = BigInt;

	fn rem(self, other: Self) -> Self::Output {
		&self % &other
	}
}

impl Rem<&BigInt> for BigInt {
	type Output = BigInt;

	fn rem(self, other: &Self) -> Self::Output {
		&self % other
	}
}

impl Rem<BigInt> for &BigInt {
	type Output = BigInt;

	fn rem(self, other: BigInt) -> Self::Output {
		self % &other
	}
}

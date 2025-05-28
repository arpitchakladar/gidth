use std::ops::{
	Add,
	AddAssign,
};
use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl Add<&BigInt> for BigInt {
	type Output = BigInt;

	fn add(self, rhs: &BigInt) -> Self::Output {
		let mut lhs = self;

		match (lhs.positive, rhs.positive) {
			(true, true) => BigInt::u_add_assign(&mut lhs, rhs),
			(true, false) => {return 0.into();},
			(false, true) => {return 0.into();},
			(false, false) => {
				BigInt::u_add_assign(&mut lhs, rhs);
				lhs.positive = false;
			},
		}

		lhs
	}
}

impl AddAssign<&BigInt> for BigInt {
	fn add_assign(&mut self, rhs: &BigInt) {
		*self = self.clone() + rhs;
	}
}

impl AddAssign<BigInt> for BigInt {
	fn add_assign(&mut self, rhs: BigInt) {
		*self = self.clone() + &rhs;
	}
}

// impl_big_int_binop_variants!(Add, add, +);

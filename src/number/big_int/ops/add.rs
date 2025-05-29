use std::ops::{
	Add,
	AddAssign,
};
use crate::number::BigInt;
// use crate::impl_big_in_placet_binop_variants;

impl Add<&BigInt> for BigInt {
	type Output = BigInt;

	fn add(self, rhs: &BigInt) -> Self::Output {
		let mut lhs = self;
		lhs += rhs;
		lhs
	}
}

impl Add<BigInt> for BigInt {
	type Output = BigInt;

	fn add(self, rhs: BigInt) -> Self::Output {
		self + &rhs
	}
}

impl Add<BigInt> for &BigInt {
	type Output = BigInt;

	fn add(self, rhs: BigInt) -> Self::Output {
		rhs + self
	}
}

impl Add<&BigInt> for &BigInt {
	type Output = BigInt;

	fn add(self, rhs: &BigInt) -> Self::Output {
		let mut res = BigInt::with_capacity(
			self.limbs.len().max(rhs.limbs.len()) + 1
		);
		match (self.positive, rhs.positive) {
			(true, true) => BigInt::u_add_in_place(self, rhs, &mut res),
			(true, false) => BigInt::u_sub_in_place(self, rhs, &mut res),
			(false, true) => BigInt::u_sub_in_place(rhs, self, &mut res),
			(false, false) => {
				BigInt::u_add_in_place(self, rhs, &mut res);
				res.positive = false;
			},
		}

		res
	}
}

impl AddAssign<&BigInt> for BigInt {
	fn add_assign(&mut self, rhs: &BigInt) {
		match (self.positive, rhs.positive) {
			(true, true) => BigInt::u_add_assign(self, rhs),
			(true, false) => BigInt::u_sub_assign(self, rhs),
			(false, true) => {
				BigInt::u_sub_assign(self, rhs);
				self.positive = !self.positive;
			},
			(false, false) => {
				BigInt::u_add_assign(self, rhs);
				self.positive = false;
			},
		}
	}
}

impl AddAssign<BigInt> for BigInt {
	fn add_assign(&mut self, rhs: BigInt) {
		*self += &rhs;
	}
}

// impl_big_in_placet_binop_variants!(Add, add, +);

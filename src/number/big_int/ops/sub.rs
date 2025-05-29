use std::ops::{
	Sub,
	SubAssign,
};
use crate::number::BigInt;

impl Sub<&BigInt> for BigInt {
	type Output = BigInt;

	fn sub(self, rhs: &BigInt) -> Self::Output {
		let mut lhs = self;
		lhs -= rhs;

		lhs
	}
}

impl Sub<BigInt> for BigInt {
	type Output = BigInt;

	fn sub(self, rhs: BigInt) -> Self::Output {
		self - &rhs
	}
}

impl Sub<BigInt> for &BigInt {
	type Output = BigInt;

	fn sub(self, rhs: BigInt) -> Self::Output {
		let mut rhs = rhs;
		rhs -= self;
		rhs.positive = !rhs.positive;
		rhs
	}
}

impl Sub<&BigInt> for &BigInt {
	type Output = BigInt;

	fn sub(self, rhs: &BigInt) -> Self::Output {
		let mut res = BigInt::with_capacity(
			self.limbs.len().max(rhs.limbs.len())
		);
		match (self.positive, rhs.positive) {
			(true, true) => BigInt::u_sub_in_place(self, rhs, &mut res),
			(true, false) => BigInt::u_add_in_place(self, rhs, &mut res),
			(false, true) => {
				BigInt::u_add_in_place(self, rhs, &mut res);
				res.positive = false;
			},
			(false, false) => BigInt::u_sub_in_place(rhs, self, &mut res),
		}

		res
	}
}

impl SubAssign<&BigInt> for BigInt {
	fn sub_assign(&mut self, rhs: &BigInt) {
		match (self.positive, rhs.positive) {
			(true, true) => BigInt::u_sub_assign(self, rhs),
			(true, false) => BigInt::u_add_assign(self, rhs),
			(false, true) => BigInt::u_add_assign(self, rhs),
			(false, false) => {
				BigInt::u_sub_assign(self, rhs);
				self.positive = !self.positive;
			},
		}
	}
}

impl SubAssign<BigInt> for BigInt {
	fn sub_assign(&mut self, rhs: BigInt) {
		*self -= &rhs;
	}
}

use std::ops::{
	Sub,
	SubAssign,
};
use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl Sub for &BigInt {
	type Output = BigInt;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut result = BigInt::with_capacity(
			std::cmp::max(
				self.limbs.len(),
				rhs.limbs.len(),
			) + 1,
		);
		match (self.positive, rhs.positive) {
			(true, true) => BigInt::u_sub_in(self, rhs, &mut result),
			(true, false) => {return 0.into();},
			(false, true) => {return 0.into();},
			(false, false) => BigInt::u_sub_in(rhs, self, &mut result),
		}

		result
	}
}

impl SubAssign<&BigInt> for BigInt {
	fn sub_assign(&mut self, rhs: &BigInt) {
		*self = &*self - rhs;
	}
}

impl SubAssign<BigInt> for BigInt {
	fn sub_assign(&mut self, rhs: BigInt) {
		*self = &*self - rhs;
	}
}

impl_big_int_binop_variants!(Sub, sub, -);

use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl std::ops::Add for &BigInt {
	type Output = BigInt;

	fn add(self, rhs: Self) -> Self::Output {
		let mut result = BigInt::with_capacity(
			std::cmp::max(
				self.limbs.len(),
				rhs.limbs.len(),
			) + 1,
		);

		match (self.positive, rhs.positive) {
			(true, true) => BigInt::u_add_in(self, rhs, &mut result),
			(true, false) => BigInt::u_sub_in(self, rhs, &mut result),
			(false, true) => BigInt::u_sub_in(rhs, self, &mut result),
			(false, false) => {
				BigInt::u_add_in(self, rhs, &mut result);
				result.positive = false;
			},
		}

		result
	}
}

impl_big_int_binop_variants!(Add, add, +);

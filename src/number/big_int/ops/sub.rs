use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl std::ops::Sub for &BigInt {
	type Output = BigInt;

	fn sub(self, other: Self) -> Self::Output {
		match (self.positive, other.positive) {
			(true, true) => BigInt::u_sub(self, other),
			(true, false) => BigInt::u_add(self, other),
			(false, true) => {
				let mut result = BigInt::u_add(self, other);
				result.positive = false;
				result
			},
			(false, false) => BigInt::u_sub(other, self),
		}
	}
}

impl_big_int_binop_variants!(Sub, sub, -);

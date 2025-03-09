use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl std::ops::Add for &BigInt {
	type Output = BigInt;

	fn add(self, other: Self) -> Self::Output {
		match (self.positive, other.positive) {
			(true, true) => BigInt::u_add(self, other),
			(true, false) => BigInt::u_sub(self, other),
			(false, true) => BigInt::u_sub(other, self),
			(false, false) => {
				let mut result = BigInt::u_add(self, other);
				result.positive = false;
				result
			},
		}
	}
}

impl_big_int_binop_variants!(Add, add, +);

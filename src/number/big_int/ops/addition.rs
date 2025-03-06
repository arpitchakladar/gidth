use crate::number::BigInt;
use crate::impl_binop_variants;

impl std::ops::Add for &BigInt {
	type Output = BigInt;

	fn add(self, other: Self) -> Self::Output {
		match (self.positive, other.positive) {
			(true, true) => BigInt::unsigned_add(self, other),
			(true, false) => BigInt::unsigned_sub(self, other),
			(false, true) => BigInt::unsigned_sub(other, self),
			(false, false) => {
				let mut result = BigInt::unsigned_add(self, other);
				result.positive = false;
				result
			},
		}
	}
}

impl_binop_variants!(Add, add, +);

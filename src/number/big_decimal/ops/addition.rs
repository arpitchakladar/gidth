use crate::number::BigDecimal;
use crate::impl_big_decimal_binop_variants;

impl std::ops::Add for &BigDecimal {
	type Output = BigDecimal;

	fn add(self, other: Self) -> Self::Output {
		let mut result = BigDecimal::with_capacity((self.order() - other.order()).abs() as usize);
		match (self.positive, other.positive) {
			(true, true) => BigDecimal::unsigned_add(self, other, &mut result),
			(true, false) => BigDecimal::unsigned_sub(self, other, &mut result),
			(false, true) => BigDecimal::unsigned_sub(other, self, &mut result),
			(false, false) => {
				BigDecimal::unsigned_add(self, other, &mut result);
				result.positive = false;
			},
		}

		result
	}
}

impl_big_decimal_binop_variants!(Add, add, +);

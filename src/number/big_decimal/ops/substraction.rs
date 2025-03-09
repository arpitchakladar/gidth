use crate::number::BigDecimal;
use crate::impl_big_decimal_binop_variants;

impl std::ops::Sub for &BigDecimal {
	type Output = BigDecimal;

	fn sub(self, other: Self) -> Self::Output {
		let mut result = BigDecimal::with_capacity((self.order() - other.order()).abs() as usize);
		match (self.positive, other.positive) {
			(true, true) => BigDecimal::unsigned_sub(self, other, &mut result),
			(true, false) => BigDecimal::unsigned_add(self, other, &mut result),
			(false, true) => {
				BigDecimal::unsigned_add(self, other, &mut result);
				result.positive = false;
			},
			(false, false) => BigDecimal::unsigned_sub(other, self, &mut result),
		}

		result
	}
}

impl_big_decimal_binop_variants!(Sub, sub, -);

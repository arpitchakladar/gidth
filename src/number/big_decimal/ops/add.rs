use crate::number::BigDecimal;
use crate::impl_big_decimal_binop_variants;

impl std::ops::Add for &BigDecimal {
	type Output = BigDecimal;

	fn add(self, other: Self) -> Self::Output {
		let mut result = BigDecimal::with_capacity(
			(self.order() - other.order()).abs() as usize
		);
		match (self.positive, other.positive) {
			(true, true) => BigDecimal::u_add_in(self, other, &mut result),
			(true, false) => BigDecimal::u_sub_in(self, other, &mut result),
			(false, true) => BigDecimal::u_sub_in(other, self, &mut result),
			(false, false) => {
				BigDecimal::u_add_in(self, other, &mut result);
				result.positive = false;
			},
		}

		result
	}
}

impl_big_decimal_binop_variants!(Add, add, +);

use crate::number::BigDecimal;
use crate::impl_big_decimal_binop_variants;

impl std::ops::Div for &BigDecimal {
	type Output = BigDecimal;

	fn div(self, rhs: Self) -> Self::Output {
		let precision = rhs.decimal_pos + self.decimal_pos + 1;
		let mut quotient = BigDecimal::with_capacity(
			self.limbs.len()
				.saturating_sub(rhs.limbs.len()) + 1 + precision,
		);

		let mut remainder = BigDecimal::with_capacity(
			self.limbs.len() + precision,
		);
		remainder.limbs.resize(precision, 0u32);
		remainder.limbs.extend(&self.limbs);
		// This is used as the precision for the result
		remainder.decimal_pos = precision + self.decimal_pos - rhs.decimal_pos;
		BigDecimal::u_div_in(&mut remainder, &rhs, &mut quotient);

		quotient.positive = self.positive == rhs.positive;

		quotient
	}
}

impl_big_decimal_binop_variants!(Div, div, /);

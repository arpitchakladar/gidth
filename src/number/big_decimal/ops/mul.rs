use crate::number::BigDecimal;
use crate::{
	impl_big_decimal_binop_variants,
	impl_big_decimal_binop_assign_variants,
};

impl BigDecimal {
	pub(crate) fn u_mul_in(
		&self,
		rhs: &BigDecimal,
		result: &mut BigDecimal,
	) {
		result.limbs.resize(
			self.limbs.len() + rhs.limbs.len(),
			0,
		);

		result.decimal_pos = self.decimal_pos + rhs.decimal_pos;

		rhs.limbs
			.iter()
			.copied()
			.enumerate()
			.for_each(|(rhs_index, rhs_limb)| {
				let carry = self.limbs
					.iter()
					.copied()
					.enumerate()
					.fold(
						0u64,
						|carry, (lhs_index, lhs_limb)| {
							let result_index = rhs_index + lhs_index;
							let product = lhs_limb as u64 * rhs_limb as u64;
							let current_limb = result.limbs[result_index] as u64;
							let sum = product + carry + current_limb;
							result.limbs[result_index] = sum as u32;

							sum >> 32
						},
					);

				let starting_pos = self.limbs.len() + rhs_index;
				result.limbs[starting_pos..]
					.iter_mut()
					.fold(
						carry,
						|carry, limb| {
							let sum = carry + *limb as u64;
							*limb = sum as u32;

							sum >> 32
						},
					);
			});
	}
}

impl std::ops::Mul for &BigDecimal {
	type Output = BigDecimal;

	fn mul(self, other: Self) -> Self::Output {
		let mut result = BigDecimal::with_capacity(
			self.limbs.len() + other.limbs.len(),
		);
		BigDecimal::u_mul_in(
			self,
			other,
			&mut result,
		);
		result.positive = self.positive == other.positive;

		result
	}
}

impl_big_decimal_binop_variants!(Mul, mul, *);
impl_big_decimal_binop_assign_variants!(MulAssign, mul_assign, *);

use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl BigInt {
	/// Multiplies two unsigned `BigInt` numbers and stores the result in `result`.
	pub(crate) fn u_mul_in(&self, rhs: &BigInt, result: &mut BigInt) {
		// Ensure result has enough space to store the maximum possible digits.
		result.digits.resize(
			self.digits.len() + rhs.digits.len(),
			0,
		);

		rhs.digits
			.iter()
			.copied()
			.enumerate()
			.for_each(|(rhs_index, rhs_digit)| {
				let carry = self.digits
					.iter()
					.copied()
					.enumerate()
					.fold(
						0u64,
						|carry, (lhs_index, lhs_digit)| {
							let result_index = rhs_index + lhs_index;
							let product = lhs_digit as u64 * rhs_digit as u64;
							let sum = product + carry + result.digits[result_index] as u64;
							result.digits[result_index] = sum as u32;
							sum >> 32
						},
					);

				let starting_pos = self.digits.len() + rhs_index;
				result.digits[starting_pos..]
					.iter_mut()
					.fold(
						carry,
						|carry, digit| {
							let sum = carry + *digit as u64;
							*digit = sum as u32;
							sum >> 32
						},
					);
			});
	}
}

impl std::ops::Mul for &BigInt {
	type Output = BigInt;

	fn mul(self, other: Self) -> Self::Output {
		let mut result = BigInt::with_capacity(
			self.digits.len() + other.digits.len(),
		);
		BigInt::u_mul_in(
			self,
			other,
			&mut result,
		);
		result.positive = self.positive == other.positive;

		result
	}
}

impl_big_int_binop_variants!(Mul, mul, *);

use crate::number::BigInt;
use crate::impl_binop_variants;

impl BigInt {
	/// Multiplies two unsigned `BigInt` numbers and stores the result in `result`.
	pub(crate) fn unsigned_inplace_mul(&self, rhs: &BigInt, result: &mut BigInt) {
		// Ensure result has enough space to store the maximum possible digits.
		result.digits.resize(self.digits.len() + rhs.digits.len(), 0);

		for (rhs_index, &rhs_digit) in rhs.digits.iter().enumerate() {
			let mut carry = 0u64;

			// Multiply the current rhs digit with each digit of self.
			for lhs_index in 0..self.digits.len() {
				let result_index = rhs_index + lhs_index;
				let product = self.digits[lhs_index] as u64 * rhs_digit as u64;
				let sum = product + carry + result.digits[result_index] as u64;

				// Store lower 32 bits.
				result.digits[result_index] = sum as u32;
				carry = sum >> 32; // Carry upper 32 bits.
			}

			// Add remaining carry to higher result digits.
			for result_index in (self.digits.len() + rhs_index)..result.digits.len() {
				if carry == 0 {
					break; // No remaining carry, stop propagation.
				}

				let sum = carry + result.digits[result_index] as u64;
				result.digits[result_index] = sum as u32;
				carry = sum >> 32;
			}
		}
	}
}

impl std::ops::Mul for &BigInt {
	type Output = BigInt;

	fn mul(self, other: Self) -> Self::Output {
		let mut result = BigInt::with_capacity(
			self.digits.len() + other.digits.len(),
		);
		BigInt::unsigned_inplace_mul(
			self,
			other,
			&mut result,
		);
		result.positive = self.positive == other.positive;

		result
	}
}

impl_binop_variants!(Mul, mul, *);

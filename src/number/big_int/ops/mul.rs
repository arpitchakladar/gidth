use std::ops::{
	Mul,
	MulAssign,
};
use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl BigInt {
	/// Multiplies two unsigned `BigInt` numbers and stores the result in `result`.
	pub(crate) fn u_mul_in(&self, rhs: &BigInt, result: &mut BigInt) {
		// Ensure result has enough space to store the maximum possible limbs.
		result.limbs.resize(
			self.limbs.len() + rhs.limbs.len(),
			0,
		);

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

impl Mul for &BigInt {
	type Output = BigInt;

	fn mul(self, other: Self) -> Self::Output {
		let mut result = BigInt::with_capacity(
			self.limbs.len() + other.limbs.len(),
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

impl MulAssign<&BigInt> for BigInt {
	fn mul_assign(&mut self, rhs: &BigInt) {
		*self = &*self * rhs;
	}
}

impl_big_int_binop_variants!(Mul, mul, *);

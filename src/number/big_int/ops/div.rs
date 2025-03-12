use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl std::ops::Div for &BigInt {
	type Output = BigInt;

	fn div(self, rhs: Self) -> Self::Output {
		let mut quotient = BigInt::with_capacity(
			self.limbs.len()
				.saturating_sub(rhs.limbs.len()) + 1,
		);
		let mut remainder = self.clone();
		BigInt::u_div_in(&mut remainder, &rhs, &mut quotient);


		quotient
	}
}

impl_big_int_binop_variants!(Div, div, /);

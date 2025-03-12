use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl std::ops::Rem for &BigInt {
	type Output = BigInt;

	fn rem(self, rhs: Self) -> Self::Output {
		let mut remainder = BigInt::with_capacity(
			rhs.limbs.len(),
		);
		BigInt::u_rem_in(self, rhs, &mut remainder);

		remainder
	}
}

impl_big_int_binop_variants!(Rem, rem, %);

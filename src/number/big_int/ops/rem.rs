use crate::number::BigInt;
use crate::{
	impl_big_int_binop_variants,
	impl_big_int_binop_assign_variants,
};

impl std::ops::Rem for &BigInt {
	type Output = BigInt;

	fn rem(self, rhs: Self) -> Self::Output {
		let mut remainder = self.clone();
		BigInt::u_rem_in(&mut remainder, &rhs);

		remainder
	}
}

impl_big_int_binop_variants!(Rem, rem, %);
impl_big_int_binop_assign_variants!(RemAssign, rem_assign, %);

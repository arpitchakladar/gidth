use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl std::ops::Rem for &BigInt {
	type Output = BigInt;

	fn rem(self, other: Self) -> Self::Output {
		BigInt::u_divmod(self, other).1
	}
}

impl_big_int_binop_variants!(Rem, rem, %);

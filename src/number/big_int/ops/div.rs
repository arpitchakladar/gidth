use crate::number::BigInt;
use crate::impl_big_int_binop_variants;

impl std::ops::Div for &BigInt {
	type Output = BigInt;

	fn div(self, other: Self) -> Self::Output {
		BigInt::u_divmod(self, other).0
	}
}

impl_big_int_binop_variants!(Div, div, /);

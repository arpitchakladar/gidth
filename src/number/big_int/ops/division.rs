use crate::number::BigInt;
use crate::impl_binop_variants;

impl std::ops::Div for &BigInt {
	type Output = BigInt;

	fn div(self, other: Self) -> Self::Output {
		BigInt::unsigned_divmod(self, other).0
	}
}

impl_binop_variants!(Div, div, /);

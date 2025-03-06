#[macro_export]
macro_rules! impl_binop_variants {
	($trait:ident, $method:ident, $op:tt) => {
		impl std::ops::$trait for BigInt {
			type Output = BigInt;

			#[inline]
			fn $method(self, other: Self) -> Self::Output {
				&self $op &other
			}
		}

		impl std::ops::$trait<&BigInt> for BigInt {
			type Output = BigInt;

			#[inline]
			fn $method(self, other: &Self) -> Self::Output {
				&self $op other
			}
		}

		impl std::ops::$trait<BigInt> for &BigInt {
			type Output = BigInt;

			#[inline]
			fn $method(self, other: BigInt) -> Self::Output {
				self $op &other
			}
		}
	};
}

#[macro_export]
macro_rules! impl_binop_variants {
	($num_type:ident, $trait:ident, $method:ident, $op:tt) => {
		impl $trait for $num_type {
			type Output = $num_type;

			#[inline]
			fn $method(self, other: Self) -> Self::Output {
				&self $op &other
			}
		}

		impl $trait<&$num_type> for $num_type {
			type Output = $num_type;

			#[inline]
			fn $method(self, other: &Self) -> Self::Output {
				&self $op other
			}
		}

		impl $trait<$num_type> for &$num_type {
			type Output = $num_type;

			#[inline]
			fn $method(self, other: $num_type) -> Self::Output {
				self $op &other
			}
		}
	};
}

#[macro_export]
macro_rules! impl_binop_assign_variants {
	($num_type:ident, $trait:ident, $method:ident, $op:tt) => {
		impl $trait<&$num_type> for $num_type {
			#[inline]
			fn $method(&mut self, other: &Self) {
				*self = &*self $op other
			}
		}

		impl $trait<$num_type> for $num_type {
			#[inline]
			fn $method(&mut self, other: Self) {
				*self = &*self $op &other
			}
		}
	};
}

#[macro_export]
macro_rules! impl_ratio_binop_variants {
	($trait:ident, $method:ident, $op:tt) => {
		impl<T: Int + Clone> std::ops::$trait for Ratio<T> {
			type Output = Ratio<T>;

			#[inline]
			fn $method(self, other: Ratio<T>) -> Self::Output {
				&self $op &other
			}
		}

		impl<T: Int + Clone> std::ops::$trait<&Ratio<T>> for Ratio<T> {
			type Output = Ratio<T>;

			#[inline]
			fn $method(self, other: &Ratio<T>) -> Self::Output {
				&self $op other
			}
		}

		impl<T: Int + Clone> std::ops::$trait<Ratio<T>> for &Ratio<T> {
			type Output = Ratio<T>;

			#[inline]
			fn $method(self, other: Ratio<T>) -> Self::Output {
				self $op &other
			}
		}
	};
}

#[macro_export]
macro_rules! impl_ratio_binop_assign_variants {
	($trait:ident, $method:ident, $op:tt) => {
		impl<T: Int + Clone> std::ops::$trait for Ratio<T> {
			#[inline]
			fn $method(&mut self, other: Ratio<T>) {
				*self = &*self $op &other;
			}
		}

		impl<T: Int + Clone> std::ops::$trait<&Ratio<T>> for Ratio<T> {
			#[inline]
			fn $method(&mut self, other: &Ratio<T>) {
				*self = &*self $op other;
			}
		}
	};
}

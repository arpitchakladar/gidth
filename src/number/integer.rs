use crate::number::{
	Abs,
	Square,
};

pub trait DivMod<T>: Sized
where
	Self: From<T>,
{
	fn divmod(self, rhs: T) -> (Self, T);
}

pub trait Integer:
	std::ops::Add<Output = Self> +
	std::ops::Sub<Output = Self> +
	std::ops::Mul<Output = Self> +
	std::ops::Div<Output = Self> +
	std::ops::Rem<Output = Self> +
	DivMod<Self> +
	Abs +
	Square +
	Sized
{}

macro_rules! impl_abs_for_signed {
	($($t:ty),*) => {
		$(
		impl Abs for $t {
			#[inline(always)]
			fn abs(self) -> Self {
				if self >= 0 {
					self
				} else {
					-self
				}
			}
		}
		)*
	};
}

macro_rules! impl_abs_for_unsigned {
	($($t:ty),*) => {
		$(
		impl Abs for $t {
			#[inline(always)]
			fn abs(self) -> Self {
				self
			}
		}
		)*
	};
}

macro_rules! impl_integer {
	($($t:ty),*) => {
		$(
		impl Square for $t {
			fn sq(self) -> Self {
				self * self
			}
		}

		impl DivMod<$t> for $t {
			fn divmod(self, rhs: Self) -> (Self, Self) {
				(self / rhs, self % rhs)
			}
		}

		impl Integer for $t {}
		)*
	};
}

impl_abs_for_signed!(i8, i16, i32, i64, i128, isize);
impl_abs_for_unsigned!(u8, u16, u32, u64, u128, usize);
impl_integer!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

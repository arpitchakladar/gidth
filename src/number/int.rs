use crate::number::{
	Abs,
	Square,
	Zero,
	One,
	Real,
};
use gidth_macros::{
	siphon_traits,
	satisfy,
};

pub trait DivMod<T>: Sized
where
	Self: From<T>,
{
	fn divmod(self, rhs: T) -> (Self, T);
}

#[siphon_traits]
pub trait Int:
	Real +
	std::ops::Add<Output = Self> +
	std::ops::Sub<Output = Self> +
	std::ops::Mul<Output = Self> +
	std::ops::Div<Output = Self> +
	std::ops::Rem<Output = Self> +
	for<'a> std::ops::Add<&'a Self, Output = Self> +
	for<'a> std::ops::Sub<&'a Self, Output = Self> +
	for<'a> std::ops::Mul<&'a Self, Output = Self> +
	for<'a> std::ops::Div<&'a Self, Output = Self> +
	for<'a> std::ops::Rem<&'a Self, Output = Self> +
	std::cmp::PartialEq +
	DivMod<Self> +
	Zero +
	One +
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

			impl Zero for $t {
				#[inline(always)]
				fn zero() -> Self {
					0
				}

				#[inline(always)]
				fn is_zero(&self) -> bool {
					*self == 0
				}
			}

			impl One for $t {
				#[inline(always)]
				fn one() -> Self {
					1
				}

				#[inline(always)]
				fn is_one(&self) -> bool {
					*self == 1
				}
			}

			// Make all primitive integers satisfy Int and Real
			satisfy!($t; Real, Int);
		)*
	};
}

impl_abs_for_signed!(i8, i16, i32, i64, i128, isize);
impl_abs_for_unsigned!(u8, u16, u32, u64, u128, usize);
impl_integer!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

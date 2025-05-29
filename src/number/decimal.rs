use std::ops::{
	Add,
	Sub,
	Mul,
	Div,
	AddAssign,
	SubAssign,
	MulAssign,
	DivAssign,
};
use std::cmp::{
	PartialEq,
	PartialOrd,
};
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

#[siphon_traits]
pub trait Decimal:
	Real +
	Clone +
	Add<Output = Self> +
	Sub<Output = Self> +
	Mul<Output = Self> +
	Div<Output = Self> +
	for<'a> Add<&'a Self, Output = Self> +
	for<'a> Sub<&'a Self, Output = Self> +
	for<'a> Div<&'a Self, Output = Self> +
	for<'a> Mul<&'a Self, Output = Self> +
	AddAssign<Self> +
	SubAssign<Self> +
	MulAssign<Self> +
	DivAssign<Self> +
	for<'a> AddAssign<&'a Self> +
	for<'a> SubAssign<&'a Self> +
	for<'a> DivAssign<&'a Self> +
	for<'a> MulAssign<&'a Self> +
	PartialEq +
	PartialOrd +
	Zero +
	One +
	Abs +
	Square +
	Sized
{}

macro_rules! impl_decimal {
	($($t:ty),*) => {
		$(
			impl Abs for $t {
				#[inline(always)]
				fn abs(self) -> Self {
					if self >= 0.0 {
						self
					} else {
						-self
					}
				}
			}

			impl Square for $t {
				fn sq(self) -> Self {
					self * self
				}
			}

			impl Zero for $t {
				#[inline(always)]
				fn zero() -> Self {
					0.0
				}

				#[inline(always)]
				fn is_zero(&self) -> bool {
					*self == 0.0
				}
			}

			impl One for $t {
				#[inline(always)]
				fn one() -> Self {
					1.0
				}

				#[inline(always)]
				fn is_one(&self) -> bool {
					*self == 1.0
				}
			}

			// Make all primitive floats satisfyReal
			satisfy!($t; Real, Decimal);
		)*
	};
}

impl_decimal!(f32, f64);

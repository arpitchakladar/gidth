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
	Clone +
	Real +
	std::ops::Add<Output = Self> +
	std::ops::Sub<Output = Self> +
	std::ops::Mul<Output = Self> +
	std::ops::Div<Output = Self> +
	for<'a> std::ops::Add<&'a Self, Output = Self> +
	for<'a> std::ops::Sub<&'a Self, Output = Self> +
	for<'a> std::ops::Mul<&'a Self, Output = Self> +
	for<'a> std::ops::Div<&'a Self, Output = Self> +
	std::cmp::PartialEq +
	std::cmp::PartialOrd +
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

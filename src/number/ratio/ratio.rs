use inherent::inherent;

// use gidth_macros::satisfies;

use crate::number::{
	Int,
	Zero,
	One,
};

#[derive(Clone, Debug)]
// #[satisfies(Real)]
pub struct Ratio<T: Int> {
	pub(crate) num: T,
	pub(crate) den: T,
}

impl<T: Int> Ratio<T> {
	pub fn new(num: T, den: T) -> Self {
		Self {
			num,
			den,
		}
	}
}

#[inherent]
impl<T: Int> Zero for Ratio<T> {
	pub fn zero() -> Self {
		Self {
			num: Zero::zero(),
			den: One::one(),
		}
	}

	pub fn is_zero(&self) -> bool {
		Zero::is_zero(&self.num)
	}
}

#[inherent]
impl<T: Int> One for Ratio<T> {
	pub fn one() -> Self {
		Self {
			num: One::one(),
			den: One::one(),
		}
	}

	pub fn is_one(&self) -> bool {
		self.num == self.den
	}
}

pub use inherent::inherent;

use crate::number::{
	Int,
	Ratio,
	Abs,
};

#[inherent]
impl<T: Int> Abs for Ratio<T> {
	#[inline]
	pub fn abs(mut self) -> Self {
		self.num = Abs::abs(self.num);
		self.den = Abs::abs(self.den);

		self
	}
}

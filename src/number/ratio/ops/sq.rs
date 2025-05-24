use inherent::inherent;

use crate::number::{
	Int,
	Ratio,
	Square,
};

#[inherent]
impl<T: Int> Square for Ratio<T> {
	pub fn sq(mut self) -> Self {
		self.num = Square::sq(self.num);
		self.den = Square::sq(self.den);

		self
	}
}

use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + std::ops::Neg<Output = T>> std::ops::Neg for Ratio<T> {
	type Output = Ratio<T>;

	fn neg(mut self) -> Self::Output {
		self.num = -self.num;
		self
	}
}

impl<T: Int + Clone + std::ops::Neg<Output = T>> std::ops::Neg for &Ratio<T> {
	type Output = Ratio<T>;

	fn neg(self) -> Self::Output {
		-self.clone()
	}
}

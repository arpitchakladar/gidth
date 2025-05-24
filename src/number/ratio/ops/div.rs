use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + Clone> std::ops::Div<&Ratio<T>> for Ratio<T> {
	type Output = Ratio<T>;

	fn div(mut self, rhs: &Ratio<T>) -> Self::Output {
		self.num = self.num.clone() * &rhs.den;
		self.den = self.den.clone() * &rhs.num;

		self
	}
}

impl<T: Int + Clone> std::ops::Div<Ratio<T>> for &Ratio<T> {
	type Output = Ratio<T>;

	fn div(self, mut rhs: Ratio<T>) -> Self::Output {
		rhs.num = self.num.clone() * &rhs.den;
		rhs.den = self.den.clone() * &rhs.num;

		rhs
	}
}

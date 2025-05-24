use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + Clone> std::ops::Mul<&Ratio<T>> for Ratio<T> {
	type Output = Ratio<T>;

	fn mul(mut self, rhs: &Ratio<T>) -> Self::Output {
		self.num = self.num.clone() * &rhs.num;
		self.den = self.den.clone() * &rhs.den;

		self
	}
}

impl<T: Int + Clone> std::ops::Mul<Ratio<T>> for &Ratio<T> {
	type Output = Ratio<T>;

	fn mul(self, mut rhs: Ratio<T>) -> Self::Output {
		rhs.num = self.num.clone() * &rhs.num;
		rhs.den = self.den.clone() * &rhs.den;

		rhs
	}
}

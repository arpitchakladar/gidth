use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + Clone> std::ops::Add<&Ratio<T>> for Ratio<T> {
	type Output = Ratio<T>;

	fn add(mut self, rhs: &Ratio<T>) -> Self::Output {
		if self.den == rhs.den {
			self.num = self.num.clone() + &rhs.num;
		} else {
			self.num = self.num.clone() * &rhs.den + self.den.clone() * &rhs.num;
			self.den = self.den.clone() * &rhs.den;
		}

		self
	}
}

impl<T: Int + Clone> std::ops::Add<Ratio<T>> for &Ratio<T> {
	type Output = Ratio<T>;

	fn add(self, mut rhs: Ratio<T>) -> Self::Output {
		if self.den == rhs.den {
			rhs.num = self.num.clone() + &rhs.num;
		} else {
			rhs.num = self.num.clone() * &rhs.den + self.den.clone() * &rhs.num;
			rhs.den = self.den.clone() * &rhs.den;
		}

		rhs
	}
}

use crate::impl_ratio_binop_variants;
use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + Clone + std::cmp::PartialEq> std::ops::Sub for &Ratio<T> {
	type Output = Ratio<T>;

	fn sub(self, rhs: Self) -> Self::Output {
		if self.den == rhs.den {
			Ratio {
				num: self.num.clone() - &rhs.num,
				den: self.den.clone(),
			}
		} else {
			Ratio {
				num: self.num.clone() * &rhs.den - self.den.clone() * &rhs.num,
				den: self.den.clone() * &rhs.den,
			}
		}
	}
}

impl_ratio_binop_variants!(Sub, sub, +);

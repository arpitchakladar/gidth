use crate::impl_ratio_binop_variants;
use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + Clone> std::ops::Mul for &Ratio<T> {
	type Output = Ratio<T>;

	fn mul(self, rhs: Self) -> Self::Output {
		Ratio {
			num: self.num.clone() * &rhs.num,
			den: self.den.clone() * &rhs.den,
		}
	}
}

impl_ratio_binop_variants!(Mul, mul, *);

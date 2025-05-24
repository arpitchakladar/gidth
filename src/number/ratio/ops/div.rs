use crate::impl_ratio_binop_variants;
use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + Clone> std::ops::Div for &Ratio<T> {
	type Output = Ratio<T>;

	fn div(self, rhs: Self) -> Self::Output {
		Ratio {
			num: self.num.clone() * &rhs.den,
			den: self.den.clone() * &rhs.num,
		}
	}
}


impl_ratio_binop_variants!(Div, div, +);

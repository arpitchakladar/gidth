use std::ops::{
	Div,
	DivAssign,
};
use crate::{
	impl_ratio_binop_variants,
	impl_ratio_binop_assign_variants,
};
use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + Clone> Div<&Ratio<T>> for &Ratio<T> {
	type Output = Ratio<T>;

	fn div(self, rhs: &Ratio<T>) -> Self::Output {
		Ratio {
			num: self.num.clone() * &rhs.den,
			den: self.den.clone() * &rhs.num,
		}
	}
}

impl_ratio_binop_variants!(Div, div, /);
impl_ratio_binop_assign_variants!(DivAssign, div_assign, /);

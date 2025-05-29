use std::ops::{
	Mul,
	MulAssign,
};
use crate::{
	impl_ratio_binop_variants,
	impl_ratio_binop_assign_variants,
};
use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + Clone> Mul<&Ratio<T>> for &Ratio<T> {
	type Output = Ratio<T>;

	fn mul(self, rhs: &Ratio<T>) -> Self::Output {
		Ratio {
			num: self.num.clone() * &rhs.num,
			den: self.den.clone() * &rhs.den,
		}
	}
}

impl_ratio_binop_variants!(Mul, mul, *);
impl_ratio_binop_assign_variants!(MulAssign, mul_assign, *);

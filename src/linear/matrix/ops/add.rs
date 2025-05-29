use std::ops::{
	Add,
	AddAssign,
};
use crate::{
	linear::Matrix,
	number::Real,
};

impl<T: Real, const R: usize, const C: usize> Add<&Matrix<T, R, C>> for Matrix<T, R, C>
where
	T: Real + Clone,
	for<'a> T: AddAssign<&'a T>
{
	type Output = Matrix<T, R, C>;

	fn add(self, rhs: &Matrix<T, R, C>) -> Self::Output {
		let mut lhs = self;
		for i in 0..R {
			for j in 0..C {
				lhs[i][j] += &rhs.data[i][j];
			}
		}
		lhs
	}
}

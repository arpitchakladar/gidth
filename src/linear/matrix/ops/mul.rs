use std::ops::Mul;
use crate::{
	linear::Matrix,
	number::Real,
};

impl<T, const R: usize, const C: usize, const K: usize>
	Mul<&Matrix<T, C, K>> for &Matrix<T, R, C>
where
	T: Real,
	for<'a> &'a T: Mul<&'a T, Output = T>,
{
	type Output = Matrix<T, R, K>;

	fn mul(self, rhs: &Matrix<T, C, K>) -> Self::Output {
		Matrix::from(
			std::array::from_fn(|i|
				std::array::from_fn(|j|
					(0..C)
						.map(|k| // Move left element out of self (consuming self), borrow right element
							&self.data[i][k] * &rhs.data[k][j]
						)
						.fold(Real::zero(), |acc, x| acc + x)
				)
			)
		)
	}
}

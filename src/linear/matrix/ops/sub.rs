use std::ops::Sub;
use crate::{
	linear::Matrix,
	number::Real,
};

impl<T: Real + Clone, const R: usize, const C: usize> Sub for &Matrix<T, R, C>
{
	type Output = Matrix<T, R, C>;

	fn sub(self, rhs: Self) -> Self::Output {
		Matrix::new(
			std::array::from_fn(|i|
				std::array::from_fn(|j|
					self.data[i][j].clone() - &rhs.data[i][j]
				)
			)
		)
	}
}

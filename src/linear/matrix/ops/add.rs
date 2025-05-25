use std::ops::Add;
use crate::{
	linear::Matrix,
	number::Real,
};

impl<T: Real + Clone, const R: usize, const C: usize> Add for &Matrix<T, R, C>
{
	type Output = Matrix<T, R, C>;

	fn add(self, rhs: Self) -> Self::Output {
		Matrix::from(
			std::array::from_fn(|i|
				std::array::from_fn(|j|
					self.data[i][j].clone() + &rhs.data[i][j]
				)
			)
		)
	}
}

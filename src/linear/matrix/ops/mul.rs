use std::ops::Mul;
use crate::{
	linear::Matrix,
	number::Real,
};

// remove + Clone
impl<T: Real + Clone, const R: usize, const C: usize, const K: usize>
	Mul<Matrix<T, C, K>> for Matrix<T, R, C>
{
	type Output = Matrix<T, R, K>;

	fn mul(self, rhs: Matrix<T, C, K>) -> Self::Output {
		// Use from_fn to build the result array
		let data = std::array::from_fn(|i| {
			std::array::from_fn(|j| {
				// sum over k: self[i][k] * rhs[k][j]
				(0..C)
					// make &rhs.data[k][j] so copy doesn't take place
					.map(|k| self.data[i][k].clone() * rhs.data[k][j].clone())
					.fold(Real::zero(), |acc, x| acc + x)
			})
		});

		Matrix::new(data)
	}
}

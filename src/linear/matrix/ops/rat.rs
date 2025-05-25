use crate::{
	linear::Matrix,
	number::{
		Int,
		Ratio,
	},
};

// Rationalizes the integer matrix
impl<T: Int, const R: usize, const C: usize> Matrix<T, R, C> {
	pub fn rat(self) -> Matrix<Ratio<T>, R, C> {
		Matrix::from(
			std::array::from_fn(
				|i| std::array::from_fn(
					|j| self.data[i][j].clone().into()
				),
			),
		)
	}
}

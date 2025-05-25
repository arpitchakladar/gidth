use crate::{
	linear::Matrix,
	number::{
		Int,
		Ratio,
	},
};

impl<T: Int + std::ops::Neg<Output = T>, const R: usize, const C: usize> Matrix<T, R, C> {
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

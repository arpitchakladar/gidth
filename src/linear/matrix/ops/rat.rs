use crate::{
	linear::Matrix,
	number::{
		Real,
		Int,
		Ratio,
	},
};

impl<T: Int + std::ops::Neg<Output = T>, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
	fn rat(self) -> Matrix<Ratio<T>, ROWS, COLS> {
		Matrix::from(
			std::array::from_fn(
				|i| std::array::from_fn(
					|j| self.data[i][j].clone().into()
				),
			),
		)
	}
}

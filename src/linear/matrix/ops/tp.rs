use crate::{
	number::Real,
	linear::Matrix,
};

impl<T: Real + Clone, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
	pub fn tp(&self) -> Matrix<T, COLS, ROWS> {
		Matrix::from(
			std::array::from_fn(
				|i|
					std::array::from_fn(
						|j| self.data[j][i].clone()
					)
			)
		)
	}
}

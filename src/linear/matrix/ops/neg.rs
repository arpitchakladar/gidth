use crate::{
	linear::Matrix,
	number::Real,
};

impl<T: Real + std::ops::Neg, const ROWS: usize, const COLS: usize> std::ops::Neg for Matrix<T, ROWS, COLS> {
	type Output = Matrix;

	fn neg(mut self) -> Self::Output {
		for row in self.data.iter_mut() {
			for x in row.iter_mut() {
				*x = -x;
			}
		}

		self
	}
}

impl<T: Real + std::ops::Neg, const ROWS: usize, const COLS: usize> std::ops::Neg for &Matrix<T, ROWS, COLS> {
	type Output = Matrix;

	fn neg(self) -> Self::Output {
		-self.clone()
	}
}

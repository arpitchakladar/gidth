use crate::{
	number::Real,
	linear::Matrix,
};

impl<T: Real, const R: usize, const C: usize> Matrix<T, R, C> {
	pub fn get(&self, row: usize, col: usize) -> Option<&T> {
		self.data.get(row)?.get(col)
	}

	pub fn set(&mut self, row: usize, col: usize, value: T) {
		if row < R && col < C {
			self.data[row][col] = value;
		}
	}

	pub fn rows(&self) -> usize {
		R
	}

	pub fn cols(&self) -> usize {
		C
	}
}

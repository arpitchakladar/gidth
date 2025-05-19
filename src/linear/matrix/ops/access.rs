use crate::{
	number::Real,
	linear::Matrix,
};

impl<T: Real, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
	pub fn get(&self, row: usize, col: usize) -> Option<&T> {
		self.data.get(row)?.get(col)
	}

	pub fn set(&mut self, row: usize, col: usize, value: T) {
		if row < ROWS && col < COLS {
			self.data[row][col] = value;
		}
	}

	pub fn rows(&self) -> usize {
		ROWS
	}

	pub fn cols(&self) -> usize {
		COLS
	}
}

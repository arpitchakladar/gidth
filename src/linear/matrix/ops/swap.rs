use crate::{
	linear::Matrix,
	number::Real,
};

// remove + Clone
// NOTE: Works for decimal types only
impl<T: Real + Clone, const R: usize, const C: usize> Matrix<T, R, C> {
	pub fn swap_cols(mut self, from: usize, to: usize) -> Self {
		for row in self.data.iter_mut() {
			row.swap(from, to);
		}

		self
	}

	pub fn swap_rows(mut self, from: usize, to: usize) -> Self {
		self.data.swap(from, to);

		self
	}
}

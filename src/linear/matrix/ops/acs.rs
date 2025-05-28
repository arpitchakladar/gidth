use crate::{
	number::Real,
	linear::Matrix,
};

impl<T: Real, const R: usize, const C: usize> Matrix<T, R, C> {
	pub fn rows(&self) -> usize {
		R
	}

	pub fn cols(&self) -> usize {
		C
	}
}

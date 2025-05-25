use std::ops::Index;
use crate::{
	linear::Matrix,
	number::Real,
};

impl<T: Real, const R: usize, const C: usize> Index<usize> for Matrix<T, R, C> {
	type Output = [T; C];

	fn index(&self, row: usize) -> &Self::Output {
		&self.data[row]
	}
}

impl<T: Real, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
	type Output = T;

	fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
		&self.data[row][column]
	}
}

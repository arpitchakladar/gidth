use std::ops::{
	Index,
	IndexMut,
};
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

impl<T: Real, const R: usize, const C: usize> IndexMut<usize> for Matrix<T, R, C> {
	fn index_mut(&mut self, row: usize) -> &mut Self::Output {
		&mut self.data[row]
	}
}

impl<T: Real, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
	fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
		&mut self.data[row][column]
	}
}

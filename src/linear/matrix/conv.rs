use crate::{
	number::Real,
	linear::Matrix,
};

impl<T: Real, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]> for Matrix<T, ROWS, COLS> {
	fn from(data: [[T; COLS]; ROWS]) -> Self {
		Self {
			data
		}
	}
}

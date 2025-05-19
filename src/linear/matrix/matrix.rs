use crate::number::Real;

#[derive(Clone)]
pub struct Matrix<T: Real, const ROWS: usize, const COLS: usize> {
	pub(crate) data: [[T; COLS]; ROWS],
}

impl<T: Real, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
	pub fn new(data: [[T; COLS]; ROWS]) -> Self {
		Self { data }
	}
}

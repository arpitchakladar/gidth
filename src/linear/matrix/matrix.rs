use crate::number::{
	Real,
	Zero,
	One,
};

#[derive(Clone)]
pub struct Matrix<T: Real, const ROWS: usize, const COLS: usize> {
	pub(crate) data: [[T; COLS]; ROWS],
}

impl<T: Real, const D: usize> Matrix<T, D, D> {
	pub fn id() -> Self {
		Self {
			data: std::array::from_fn(
				|i| std::array::from_fn(
					|j| if i == j {
						One::one()
					} else {
						Zero::zero()
					}
				),
			)
		}
	}
}

use crate::number::{
	Real,
	Zero,
	One,
};

#[derive(Clone)]
pub struct Matrix<T: Real, const R: usize, const C: usize> {
	pub(crate) data: [[T; C]; R],
}

impl<T: Real, const R: usize, const C: usize> Matrix<T, R, C> {
	pub fn zero() -> Self {
		Self {
			data: std::array::from_fn(
				|_| std::array::from_fn(
					|_| Zero::zero()
				),
			)
		}
	}
}

impl<T: Real, const D: usize> Matrix<T, D, D> {
	pub fn one() -> Self {
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

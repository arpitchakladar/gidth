use crate::{
	linear::Matrix,
	number::Real,
};

impl<T: Real + std::ops::Neg<Output = T>, const R: usize, const C: usize> std::ops::Neg for Matrix<T, R, C> {
	type Output = Matrix<T, R, C>;

	fn neg(mut self) -> Self::Output {
		for row in self.data.iter_mut() {
			for x in row.iter_mut() {
				*x = -x.clone();
			}
		}

		self
	}
}

impl<T: Real + std::ops::Neg<Output = T>, const R: usize, const C: usize> std::ops::Neg for &Matrix<T, R, C> {
	type Output = Matrix<T, R, C>;

	fn neg(self) -> Self::Output {
		-self.clone()
	}
}

use crate::{
	linear::Matrix,
	number::Decimal,
};

// remove + Clone
// NOTE: Works for decimal types only
impl<T, const D: usize> Matrix<T, D, D>
where
	T: Decimal + std::fmt::Display + std::ops::Neg<Output = T>,
	for<'a> &'a T: std::ops::Mul<&'a T, Output = T>,
	for<'a> &'a T: std::ops::Div<&'a T, Output = T>,
{
	pub fn inv(self) -> Option<Self> {
		let (u, l, p) = match self.lu() {
			Some(res) => res,
			None => return None,
		};

		// Solve LUx = Pb for each column of the oneentity matrix
		let mut inv = Matrix::zero();
		for col in 0..D {
			// Forward substitution: solve L y = Pb
			let mut y = Matrix::<T, D, 1>::zero();
			for i in 0..D {
				let mut sum = p[i][col].clone();
				for j in 0..i {
					sum -= &l[i][j] * &y[j][0];
				}
				y[i][0] = sum;
			}

			// Backward substitution: solve U x = y
			let mut x = Matrix::<T, D, 1>::zero();
			for i in (0..D).rev() {
				let mut sum = y[i][0].clone();
				for j in (i + 1)..D {
					sum -= &u[i][j] * &x[j][0];
				}
				x[i][0] = sum / &u[i][i];
			}

			// Set column in inverse matrix
			for i in 0..D {
				inv[i][col] = x[i][0].clone();
			}
		}

		Some(inv)
	}
}

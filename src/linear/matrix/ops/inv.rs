use crate::{
	linear::Matrix,
	number::{
		Abs,
		Decimal,
		Zero,
	}
};

// remove + Clone
// NOTE: Works for decimal types only
impl<T: Decimal + Clone + std::ops::Neg<Output = T>, const D: usize> Matrix<T, D, D> {
	pub fn inv(mut self) -> Option<Self> {
		let mut l: Matrix<T, D, D> = Matrix::one();
		let mut p: Matrix<T, D, D> = Matrix::one();

		for i in 0..D {
			let mut max_row = i;
			for r in (i + 1)..D {
				if Abs::abs(self[r][i].clone()) > Abs::abs(self[max_row][i].clone()) {
					max_row = r;
				}
			}

			if Zero::is_zero(&self[max_row][i]) {
				return None;
			}

			if max_row != i {
				self = self.swap_row(i, max_row);
				p.data.swap(i, max_row);
			}

			for j in (i + 1)..D {
				let x = self[j][i].clone() / &self[i][i];
				l[j][i] = x.clone();
				self[j][i] = Zero::zero();
				for k in (i + 1)..D {
					self[j][k] = self[j][k].clone() - self[i][k].clone() * &x;
				}
			}
		}

		// Solve LUx = Pb for each column of the oneentity matrix
		let mut inv = Matrix::zero();
		for col in 0..D {
			// Get column from permuted oneentity: Pb
			let mut b = Matrix::<T, D, 1>::zero();
			for i in 0..D {
				b[i][0] = p[i][col].clone();
			}

			// Forward substitution: solve L y = Pb
			let mut y = Matrix::<T, D, 1>::zero();
			for i in 0..D {
				let mut sum = b[i][0].clone();
				for j in 0..i {
					sum = sum - l[i][j].clone() * y[j][0].clone();
				}
				y[i][0] = sum;
			}

			// Backward substitution: solve U x = y
			let mut x = Matrix::<T, D, 1>::zero();
			for i in (0..D).rev() {
				let mut sum = y[i][0].clone();
				for j in (i + 1)..D {
					sum = sum - self[i][j].clone() * x[j][0].clone();
				}
				x[i][0] = sum / self[i][i].clone();
			}

			// Set column in inverse matrix
			for i in 0..D {
				inv[i][col] = x[i][0].clone();
			}
		}

		Some(inv)
	}
}

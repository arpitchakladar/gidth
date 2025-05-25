use crate::{
	linear::Matrix,
	number::{
		Abs,
		Real,
		Decimal,
		Ratio,
		Int,
		One,
		Zero,
	}
};

// remove + Clone
// NOTE: Works for decimal types only
impl<T: Decimal + Clone + std::ops::Neg<Output = T> + std::fmt::Display, const D: usize> Matrix<T, D, D> {
	pub fn inv(&self) -> Option<Self> {
		let mut u = self.clone();
		let mut l: Matrix<T, D, D> = Matrix::id();
		let mut p: Matrix<T, D, D> = Matrix::id();

		for i in 0..D {
			let mut max_row = i;
			for r in (i + 1)..D {
				if Abs::abs(u.data[r][i].clone()) > Abs::abs(u.data[max_row][i].clone()) {
					max_row = r;
				}
			}

			if Zero::is_zero(&u.data[max_row][i]) {
				return None;
			}

			if max_row != i {
				u.data.swap(i, max_row);
				p.data.swap(i, max_row);
			}

			for j in (i + 1)..D {
				let x = u.data[j][i].clone() / &u.data[i][i];
				l.data[j][i] = x.clone();
				u.data[j][i] = Zero::zero();
				for k in (i + 1)..D {
					u.data[j][k] = u.data[j][k].clone() - u.data[i][k].clone() * &x;
				}
			}
		}

		// Solve LUx = Pb for each column of the identity matrix
		let mut inv = Matrix::null();
		for col in 0..D {
			// Get column from permuted identity: Pb
			let mut b = Matrix::<T, D, 1>::null();
			for i in 0..D {
				b.data[i][0] = p.data[i][col].clone();
			}

			// Forward substitution: solve L y = Pb
			let mut y = Matrix::<T, D, 1>::null();
			for i in 0..D {
				let mut sum = b.data[i][0].clone();
				for j in 0..i {
					sum = sum - l.data[i][j].clone() * y.data[j][0].clone();
				}
				y.data[i][0] = sum;
			}

			// Backward substitution: solve U x = y
			let mut x = Matrix::<T, D, 1>::null();
			for i in (0..D).rev() {
				let mut sum = y.data[i][0].clone();
				for j in (i + 1)..D {
					sum = sum - u.data[i][j].clone() * x.data[j][0].clone();
				}
				x.data[i][0] = sum / u.data[i][i].clone();
			}

			// Set column in inverse matrix
			for i in 0..D {
				inv.data[i][col] = x.data[i][0].clone();
			}
		}

		Some(inv)
	}
}

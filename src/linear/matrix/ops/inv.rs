use crate::{
	ref_op_bounds,
	linear::Matrix,
	number::{
		Abs,
		Decimal,
		Zero,
	}
};

// remove + Clone
// NOTE: Works for decimal types only
impl<T, const D: usize> Matrix<T, D, D>
where
	T: Decimal + Clone + std::fmt::Display + std::ops::Neg<Output = T>,
	for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
	for<'a> &'a T: std::ops::Sub<&'a T, Output = T>,
	for<'a> &'a T: std::ops::Mul<&'a T, Output = T>,
	for<'a> &'a T: std::ops::Div<&'a T, Output = T>,
	for<'a> &'a T: std::ops::Add<T, Output = T>,
	for<'a> &'a T: std::ops::Sub<T, Output = T>,
	for<'a> &'a T: std::ops::Mul<T, Output = T>,
	for<'a> &'a T: std::ops::Div<T, Output = T>,
{
	pub fn inv(self) -> Option<Self> {
		let mut u = self;
		let mut l: Matrix<T, D, D> = Matrix::one();
		let mut p: Matrix<T, D, D> = Matrix::one();

		for i in 0..D {
			let mut max_row = i;
			for r in (i + 1)..D {
				if Abs::abs(u[r][i].clone()) > Abs::abs(u[max_row][i].clone()) {
					max_row = r;
				}
			}

			if Zero::is_zero(&u[max_row][i]) {
				return None;
			}

			if max_row != i {
				u.data.swap(i, max_row);
				p.data.swap(i, max_row);
			}

			for j in (i + 1)..D {
				let x = &u[j][i] / &u[i][i];
				l[j][i] = x.clone();
				u[j][i] = Zero::zero();
				for k in (i + 1)..D {
					u[j][k] = &u[j][k] - &u[i][k] * &x;
				}
			}
		}

		// Solve LUx = Pb for each column of the oneentity matrix
		let mut inv = Matrix::zero();
		for col in 0..D {
			// Forward substitution: solve L y = Pb
			let mut y = Matrix::<T, D, 1>::zero();
			for i in 0..D {
				let mut sum = p[i][col].clone();
				for j in 0..i {
					sum = sum - &l[i][j] * &y[j][0];
				}
				y[i][0] = sum;
			}

			// Backward substitution: solve U x = y
			let mut x = Matrix::<T, D, 1>::zero();
			for i in (0..D).rev() {
				let mut sum = y[i][0].clone();
				for j in (i + 1)..D {
					sum = sum - &u[i][j] * &x[j][0];
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

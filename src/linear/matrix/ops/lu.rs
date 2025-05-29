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
impl<T, const D: usize> Matrix<T, D, D>
where
	T: Decimal + std::fmt::Display + std::ops::Neg<Output = T>,
	for<'a> &'a T: std::ops::Mul<&'a T, Output = T>,
	for<'a> &'a T: std::ops::Div<&'a T, Output = T>,
{
	// LU factorization with partial pivoting
	// Returns the upper and lower trinagular matrices and the
	// permutation matrix
	pub fn lu(self) -> Option<(Matrix<T, D, D>, Matrix<T, D, D>, Matrix<T, D, D>)> {
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
				u = u.swap_rows(i, max_row);
				p = p.swap_rows(i, max_row);
			}

			for j in (i + 1)..D {
				let x = &u[j][i] / &u[i][i];
				l[j][i] = x.clone();
				u[j][i] = Zero::zero();
				for k in (i + 1)..D {
					let x = &u[i][k] * &x;
					u[j][k] -= x;
				}
			}
		}

		Some((l, u, p))
	}
}

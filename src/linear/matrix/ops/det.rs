use crate::{
	linear::Matrix,
	number::{
		Abs,
		Decimal,
		One,
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
	pub fn det(self) -> T {
		let mut u = self;
		let mut det: T = One::one();
		let mut sign_flip = false;

		for i in 0..D {
			let mut max_row = i;
			for r in (i + 1)..D {
				if Abs::abs(u[r][i].clone()) > Abs::abs(u[max_row][i].clone()) {
					max_row = r;
				}
			}

			if Zero::is_zero(&u[max_row][i]) {
				return Zero::zero();
			}

			if max_row != i {
				u.data.swap(i, max_row);
				sign_flip = !sign_flip;
			}

			for j in (i + 1)..D {
				let x = &u[j][i] / &u[i][i];
				u[j][i] = Zero::zero();
				for k in (i + 1)..D {
					u[j][k] = &u[j][k] - &u[i][k] * &x;
				}
			}
		}

		for i in 0..D {
			det = &det * &u[i][i];
		}

		if sign_flip {
			det = -det;
		}

		det
	}
}

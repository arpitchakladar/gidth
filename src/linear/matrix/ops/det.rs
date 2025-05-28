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
	for<'a> &'a T: std::ops::Mul<&'a T, Output = T>,
	for<'a> &'a T: std::ops::Div<&'a T, Output = T>,
	T: std::ops::SubAssign<T>,
	for<'a> T: std::ops::MulAssign<&'a T>,
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
					let x = &u[i][k] * &x;
					u[j][k] -= x;
				}
			}
		}

		for i in 0..D {
			det *= &u[i][i];
		}

		if sign_flip {
			-det
		} else {
			det
		}
	}
}

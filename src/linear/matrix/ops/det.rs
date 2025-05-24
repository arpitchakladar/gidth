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
impl<T: Decimal + Clone + std::ops::Neg<Output = T>, const D: usize> Matrix<T, D, D> {
	pub fn det(&self) -> T {
		let mut u = self.data.clone();
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
				u.swap(i, max_row);
				sign_flip = !sign_flip;
			}

			for j in (i + 1)..D {
				let x = u[j][i].clone() / &u[i][i];
				u[j][i] = Zero::zero();
				for k in (i + 1)..D {
					u[j][k] = u[j][k].clone() - u[i][k].clone() * &x;
				}
			}
		}

		for i in 0..D {
			det = det.clone() * &u[i][i];
		}

		if sign_flip {
			det = -det;
		}

		det
	}
}


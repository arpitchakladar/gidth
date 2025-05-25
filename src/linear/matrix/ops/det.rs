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
	pub fn det(mut self) -> T {
		let mut det: T = One::one();
		let mut sign_flip = false;

		for i in 0..D {
			let mut max_row = i;
			for r in (i + 1)..D {
				if Abs::abs(self[r][i].clone()) > Abs::abs(self[max_row][i].clone()) {
					max_row = r;
				}
			}

			if Zero::is_zero(&self[max_row][i]) {
				return Zero::zero();
			}

			if max_row != i {
				self = self.swap_row(i, max_row);
				sign_flip = !sign_flip;
			}

			for j in (i + 1)..D {
				let x = self[j][i].clone() / &self[i][i];
				self[j][i] = Zero::zero();
				for k in (i + 1)..D {
					self[j][k] = self[j][k].clone() - self[i][k].clone() * &x;
				}
			}
		}

		for i in 0..D {
			det = det.clone() * &self[i][i];
		}

		if sign_flip {
			det = -det;
		}

		det
	}
}

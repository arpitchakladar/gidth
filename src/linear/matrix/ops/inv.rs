use crate::{
	linear::Matrix,
	number::{
		Abs,
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
	pub fn inv(&self) {
		let mut u = self.data.clone();
		let mut l = std::array::from_fn(
			|i| std::array::from_fn(
				|j| if i == j {
					One::one()
				} else {
					Zero::zero()
				}
			),
		);
		let mut sign_flip = false;

		for i in 0..D {
			let mut max_row = i;
			for r in (i + 1)..D {
				if Abs::abs(u[r][i].clone()) > Abs::abs(u[max_row][i].clone()) {
					max_row = r;
				}
			}

			if Zero::is_zero(&u[max_row][i]) {
				return;
			}

			if max_row != i {
				u.swap(i, max_row);
				sign_flip = !sign_flip;
			}

			for j in (i + 1)..D {
				let x = u[j][i].clone() / &u[i][i];
				l[j][i] = x.clone();
				u[j][i] = Zero::zero();
				for k in (i + 1)..D {
					u[j][k] = u[j][k].clone() - u[i][k].clone() * &x;
				}
			}
		}

		println!("{}", Into::<Matrix<T, D, D>>::into(l.clone()));
		println!("{}", Into::<Matrix<T, D, D>>::into(u.clone()));
		println!("{}", &Into::<Matrix<T, D, D>>::into(l.clone()) * &Into::<Matrix<T, D, D>>::into(u.clone()));
	}
}



use crate::numbers::Integer;

impl std::ops::Add for &Integer {
	type Output = Integer;

	fn add(self, other: Self) -> Self::Output {
		let (lint, sint) = if self.digits.len() > other.digits.len() {
			(self, other)
		} else {
			(other, self)
		};
		let mut digits = Vec::with_capacity(lint.digits.len());
		let mut reg: u128;
		let mut rem: u64 = 0;
		for i in 0..sint.digits.len() {
			reg = lint.digits[i] as u128 + sint.digits[i] as u128 + rem as u128;
			rem = (reg >> 64) as u64;
			digits.push(reg as u64);
		}

		let mut j = lint.digits.len();

		for i in sint.digits.len()..lint.digits.len() {
			reg = lint.digits[i] as u128 + rem as u128;
			rem = (reg >> 64) as u64;
			digits.push(reg as u64);
			if rem == 0 {
				j = i + 1;
				break;
			}
		}

		for i in j..lint.digits.len() {
			digits.push(lint.digits[i]);
		}

		if rem != 0 {
			digits.push(rem);
		}

		Integer {
			positive: true,
			digits,
		}
	}
}

impl std::ops::Add for Integer {
	type Output = Integer;


	fn add(self, other: Self) -> Self::Output {
		&self + &other
	}
}

impl std::ops::Add<&Integer> for Integer {
	type Output = Integer;


	fn add(self, other: &Self) -> Self::Output {
		&self + other
	}
}

impl std::ops::Add<Integer> for &Integer {
	type Output = Integer;


	fn add(self, other: Integer) -> Self::Output {
		self + &other
	}
}

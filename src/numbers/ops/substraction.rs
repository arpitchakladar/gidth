use crate::numbers::Integer;
use crate::numbers::*;

impl std::ops::Sub for &Integer {
	type Output = Integer;

	fn sub(self, other: Self) -> Self::Output {
		let (lint, sint, positive) = if self >= other {
			(self, other, true)
		} else {
			(other, self, false)
		};
		let mut digits = Vec::with_capacity(lint.digits.len());
		let mut carried = 0u8;
		for i in 0..sint.digits.len() {
			let current_lint = lint.digits[i];
			let current_sint = sint.digits[i];
			if current_lint < current_sint + carried as u64 {
				digits.push(((INTEGER_BASE + current_lint as u128) - carried as u128 - current_sint as u128) as u64);
				carried = 1u8;
			} else {
				digits.push(current_lint - current_sint - carried as u64);
			}
		}
		let i = sint.digits.len();

		if lint.digits.len() > sint.digits.len() && carried > 0 {
			digits.push(lint.digits[i] - carried as u64);
		}

		for i in (i + 1)..lint.digits.len() {
			digits.push(lint.digits[i]);
		}

		Integer {
			positive,
			digits,
		}
	}
}

impl std::ops::Sub for Integer {
	type Output = Integer;


	fn sub(self, other: Self) -> Self::Output {
		&self - &other
	}
}

impl std::ops::Sub<&Integer> for Integer {
	type Output = Integer;


	fn sub(self, other: &Self) -> Self::Output {
		&self - other
	}
}

impl std::ops::Sub<Integer> for &Integer {
	type Output = Integer;


	fn sub(self, other: Integer) -> Self::Output {
		self - &other
	}
}

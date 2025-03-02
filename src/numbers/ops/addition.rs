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
		let mut reg: u16 = 0;
		let mut rem: u8 = 0;
		for i in 0..sint.digits.len() {
			reg = lint.digits[i] as u16 + sint.digits[i] as u16 + rem as u16;
			rem = (reg >> 8) as u8;
			digits.push(reg as u8);
		}

		let mut j = 0;

		for i in sint.digits.len()..lint.digits.len() {
			reg = lint.digits[i] as u16 + rem as u16;
			rem = (reg >> 8) as u8;
			digits.push(reg as u8);
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

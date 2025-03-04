use crate::numbers::BigInt;

impl BigInt {
	pub(crate) fn unsigned_mul(&self, rhs: &BigInt) -> BigInt {
		let mut result = 0.into();
		for (i, d1) in rhs.digits.iter().enumerate() {
			let mut digits = Vec::with_capacity(i + self.digits.len() + 1);
			for _ in 0..i {
				digits.push(0);
			}
			let mut rem = 0u32;
			for d2 in self.digits.iter() {
				let reg = *d2 as u64 * *d1 as u64 + rem as u64;
				rem = (reg >> 32) as u32;
				digits.push(reg as u32);
			}
			if rem != 0 {
				digits.push(rem);
			}
			result = result + BigInt::new(digits);
		}

		result
	}
}

impl std::ops::Mul for &BigInt {
	type Output = BigInt;

	fn mul(self, other: Self) -> Self::Output {
		let mut result = BigInt::unsigned_mul(self, other);
		result.positive = self.positive == other.positive;

		result
	}
}

impl std::ops::Mul for BigInt {
	type Output = BigInt;

	fn mul(self, other: Self) -> Self::Output {
		&self * &other
	}
}

impl std::ops::Mul<&BigInt> for BigInt {
	type Output = BigInt;

	fn mul(self, other: &Self) -> Self::Output {
		&self * other
	}
}

impl std::ops::Mul<BigInt> for &BigInt {
	type Output = BigInt;

	fn mul(self, other: BigInt) -> Self::Output {
		self * &other
	}
}

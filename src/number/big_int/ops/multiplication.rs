use crate::number::BigInt;

impl BigInt {
	pub(crate) fn unsigned_mul(&self, rhs: &BigInt) -> BigInt {
		let mut result = vec![0; self.digits.len() + rhs.digits.len()];
		for (i, d1) in rhs.digits.iter().enumerate() {
			let mut carry = 0u64;
			for j in 0..self.digits.len() {
				let k = i + j;
				let sum = self.digits[j] as u64 * *d1 as u64 + carry + result[k] as u64;
				carry = sum >> 32;
				result[k] = sum as u32;
			}
			for j in (self.digits.len() + i)..result.len() {
				if carry == 0 {
					break;
				} else {
					let sum = carry + result[j] as u64;
					result[j] = sum as u32;
					carry = sum >> 32;
				}
			}
		}

		let mut result = BigInt::new(result);
		result.trim();

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

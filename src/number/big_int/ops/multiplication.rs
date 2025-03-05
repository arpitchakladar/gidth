use crate::number::BigInt;

impl BigInt {
	pub(crate) fn unsigned_mul(&self, rhs: &BigInt, result: &mut BigInt) {
		result.digits.resize(self.digits.len() + rhs.digits.len(), 0);

		for (i, d1) in rhs.digits.iter().enumerate() {
			let mut carry = 0u64;
			for j in 0..self.digits.len() {
				let k = i + j;
				let sum = self.digits[j] as u64 * *d1 as u64 + carry + result.digits[k] as u64;
				carry = sum >> 32;
				result.digits[k] = sum as u32;
			}
			for j in (self.digits.len() + i)..result.digits.len() {
				if carry == 0 {
					break;
				} else {
					let sum = carry + result.digits[j] as u64;
					result.digits[j] = sum as u32;
					carry = sum >> 32;
				}
			}
		}

		result.trim();
	}
}

impl std::ops::Mul for &BigInt {
	type Output = BigInt;

	fn mul(self, other: Self) -> Self::Output {
		let mut result = BigInt::with_capacity(self.digits.len() + other.digits.len());
		BigInt::unsigned_mul(self, other, &mut result);

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

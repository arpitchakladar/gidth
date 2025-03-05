use crate::number::BigInt;

#[inline]
fn add_slice(lhs: &mut [u32], rhs: &[u32]) {
	let mut carry: u64 = 0;
	for i in 0..rhs.len() {
		let sum = lhs[i] as u64 + rhs[i] as u64 + carry;
		lhs[i] = sum as u32;
		carry = sum >> 32;
	}
	for i in rhs.len()..lhs.len() {
		if carry == 0 {
			break;
		} else {
			let sum = lhs[i] as u64 + carry;
			lhs[i] = sum as u32;
			carry = sum >> 32;
		}
	}
}

impl BigInt {
	pub(crate) fn unsigned_mul(&self, rhs: &BigInt) -> BigInt {
		let mut result = vec![0; self.digits.len() + rhs.digits.len()];
		let mut digits = Vec::with_capacity(self.digits.len() + 1);
		for (i, d1) in rhs.digits.iter().enumerate() {
			let mut rem = 0u32;
			for d2 in self.digits.iter() {
				let reg = *d2 as u64 * *d1 as u64 + rem as u64;
				rem = (reg >> 32) as u32;
				digits.push(reg as u32);
			}
			if rem != 0 {
				digits.push(rem);
			}
			add_slice(&mut result[i..], &digits);
			digits.clear();
		}

		BigInt::new(result)
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

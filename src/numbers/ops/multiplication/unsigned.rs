use crate::numbers::BigInt;

pub(crate) fn unsigned_big_int_mul(lhs: &BigInt, rhs: &BigInt) -> BigInt {
	let mut result = 0.into();
	for (i, d1) in rhs.digits.iter().enumerate() {
		let mut digits = Vec::with_capacity(i + lhs.digits.len() + 1);
		for _ in 0..i {
			digits.push(0);
		}
		let mut rem = 0u32;
		for d2 in lhs.digits.iter() {
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

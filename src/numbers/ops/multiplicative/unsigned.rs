use crate::numbers::Integer;

pub(crate) fn unsigned_integer_mul(lhs: &Integer, rhs: &Integer) -> Integer {
	let mut result = Integer::new(0);
	for (i, d1) in rhs.digits.iter().enumerate() {
		let mut digits = Vec::with_capacity(i + lhs.digits.len() + 1);
		for j in 0..i {
			digits.push(0);
		}
		let mut rem = 0u64;
		for (j, d2) in lhs.digits.iter().enumerate() {
			let reg = *d2 as u128 * *d1 as u128 + rem as u128;
			rem = (reg >> 64) as u64;
			digits.push(reg as u64);
		}
		if rem != 0 {
			digits.push(rem);
		}
		result = result + Integer::new(digits);
	}

	result
}

use crate::numbers::Integer;

pub(crate) fn unsigned_integer_mul(lhs: &Integer, rhs: &Integer) -> Integer {
	let mut result = Integer::new(0);
	for (i, d1) in rhs.digits.iter().enumerate() {
		let mut digits = Vec::with_capacity(i + lhs.digits.len() + 1);
		for j in 0..i {
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
		result = result + Integer::new(digits);
	}

	result
}

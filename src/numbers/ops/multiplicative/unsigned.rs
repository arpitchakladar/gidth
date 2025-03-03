use crate::numbers::{
	Integer,
	INTEGER_BASE,
};

pub(crate) fn unsigned_integer_mul(lhs: &Integer, rhs: &Integer) -> Integer {
	let mut result = Integer::new(0);
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
		result = result + Integer::new(digits);
	}

	result
}

fn slice_integer_sub(lhs: &mut [u32], rhs: &[u32]) {
	let mut carry: u64 = 0;
	for i in 0..rhs.len() {
		let mut reg = carry + rhs[i] as u64;
		let l = lhs[i] as u64;
		if l >= reg {
			lhs[i] = (l - reg) as u32;
		} else {
			lhs[i] = (l + INTEGER_BASE - reg) as u32;
			carry = 1;
		}
	}
	if carry > 0 {
		lhs[lhs.len() - 1] -= carry as u32;
	}
}

fn digits_greater_than(lhs: &[u32], rhs: &[u32]) -> (bool, usize) {
	if lhs.len() > rhs.len() {
		(true, 0)
	} else if lhs.len() < rhs.len() {
		(false, 0)
	} else {
		for (i, (l, r)) in lhs.iter().rev().zip(rhs.iter().rev()).enumerate() {
			if l > r {
				return (true, i);
			} else if l < r {
				return (false, i);
			}
		}

		(true, lhs.len())
	}
}

fn small_int_mul(lhs: &mut Vec, rhs: u64) {
	let mut carry = 0u32;
	for d in lhs.iter_mut() {
		let reg = rhs * *d as u64 + carry as u64;
		carry = (reg >> 64) as u32;
		*d = reg as u32;
	}
	if carry > 0 {
		lhs.push(carry);
	}
}

use crate::numbers::{
	Integer,
	INTEGER_BASE,
	unsigned_greater_than,
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

fn slice_integer_sub(lhs: &mut [u32], rhs: &[u32]) -> usize {
	let mut carry: u64 = 0;
	for i in 0..rhs.len() {
		let reg = carry + rhs[i] as u64;
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
	for i in (0..lhs.len()).rev() {
		if lhs[i] != 0 {
			return lhs.len() - i - 1;
		}
	}
	lhs.len()
}

fn digits_greater_than(lhs: &[u32], rhs: &[u32]) -> bool {
	if lhs.len() > rhs.len() {
		true
	} else if lhs.len() < rhs.len() {
		false
	} else {
		for (_, (l, r)) in lhs.iter().rev().zip(rhs.iter().rev()).enumerate() {
			if l > r {
				return true;
			} else if l < r {
				return false;
			}
		}

		true
	}
}

fn small_int_mul(lhs: &mut Vec<u32>, rhs: u32) {
	let mut carry = 0u32;
	for d in lhs.iter_mut() {
		let reg: u64 = rhs as u64 * *d as u64 + carry as u64;
		carry = (reg >> 32) as u32;
		*d = reg as u32;
	}
	if carry > 0 {
		lhs.push(carry);
	}
}

pub fn unsigned_integer_div(lhs: &Integer, rhs: &Integer) -> (Integer, Integer) {
	if unsigned_greater_than(rhs, lhs) {
		return (Integer::new(0), lhs.clone());
	}

	let l_lhs = lhs.digits.len();
	let l_rhs = rhs.digits.len();

	let mut quotient = Vec::new();
	let sig_rhs = rhs.digits[l_rhs - 1] as u64;
	let mut digits = lhs.digits.clone();
	let mut start = l_lhs - l_rhs;
	let mut end = l_lhs;

	loop {
		let reg = &mut digits[start..end];
		if digits_greater_than(reg, &rhs.digits) {
			let sig: u64 = if reg.len() == l_rhs {
				reg[reg.len() - 1] as u64
			} else {
				((reg[reg.len() - 1] as u64) << 32) + reg[reg.len() - 2] as u64
			};
			let min = (sig / (sig_rhs + 1)) as u32;
			let max = ((sig + 1) / sig_rhs) as u32;
			for i in (min..=max).rev() {
				let mut num = rhs.clone();
				small_int_mul(&mut num.digits, i);
				if digits_greater_than(reg, &num.digits) {
					quotient.push(i);
					let offset = slice_integer_sub(reg, &num.digits);
					end -= offset;
					start = end - l_rhs;
					break;
				}
			}
		} else if start > 0 {
			start -= 1;
		} else {
			break;
		}
	}

	let quotient = Integer::new(quotient);
	let remainder = Integer::new(digits[start..end].to_vec());
	(quotient, remainder)
}

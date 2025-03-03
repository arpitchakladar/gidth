use crate::numbers::{
	Integer,
	INTEGER_BASE,
	unsigned_greater_than,
};

fn sub_from_slice(lhs: &mut [u32], rhs: &[u32]) -> usize {
	let mut borrow: u64 = 0;
	for i in 0..rhs.len() {
		let right_op = borrow + rhs[i] as u64;
		let left_op = lhs[i] as u64;
		let new_digit = if right_op > left_op {
			INTEGER_BASE + left_op - right_op
		} else {
			left_op - right_op
		};
		lhs[i] = new_digit as u32;
		borrow = new_digit >> 32;
	}
	if borrow > 0 {
		lhs[lhs.len() - 1] -= borrow as u32;
	}
	for i in (0..lhs.len()).rev() {
		if lhs[i] != 0 {
			return lhs.len() - i - 1;
		}
	}
	lhs.len()
}

fn cmp_digit_arrays(lhs: &[u32], rhs: &[u32]) -> bool {
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

fn mul_by_small_int(lhs: &mut Vec<u32>, rhs: u32) {
	let mut carry = 0u64;
	for d in lhs.iter_mut() {
		let reg: u64 = rhs as u64 * *d as u64 + carry;
		carry = reg >> 32;
		*d = reg as u32;
	}
	if carry > 0 {
		lhs.push(carry as u32);
	}
}

pub fn unsigned_integer_divmod(lhs: &Integer, rhs: &Integer) -> (Integer, Integer) {
	if unsigned_greater_than(rhs, lhs) {
		return (Integer::new(0), lhs.clone());
	}

	let l_lhs = lhs.digits.len();
	let l_rhs = rhs.digits.len();

	let mut quotient = Vec::with_capacity(l_lhs - l_rhs + 1);
	let sig_rhs = rhs.digits[l_rhs - 1] as u64;
	let mut digits = lhs.digits.clone();
	let mut start = l_lhs - l_rhs;
	let mut end = l_lhs;

	loop {
		let reg = &mut digits[start..end];
		if cmp_digit_arrays(reg, &rhs.digits) {
			let sig: u64 = if reg.len() == l_rhs {
				reg[reg.len() - 1] as u64
			} else {
				((reg[reg.len() - 1] as u64) << 32) + reg[reg.len() - 2] as u64
			};
			let min = (sig / (sig_rhs + 1)) as u32;
			let max = ((sig + 1) / sig_rhs) as u32;
			for i in (min..=max).rev() {
				let mut num = rhs.clone();
				mul_by_small_int(&mut num.digits, i);
				if cmp_digit_arrays(reg, &num.digits) {
					quotient.push(i);
					let offset = sub_from_slice(reg, &num.digits);
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

	while digits.last() == Some(&0) {
		digits.pop();
	}

	let quotient = Integer::new(quotient.into_iter().rev().collect::<Vec<u32>>());
	let remainder = Integer::new(digits);
	(quotient, remainder)
}

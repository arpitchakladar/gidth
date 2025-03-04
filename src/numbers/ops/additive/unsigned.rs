use crate::numbers::{
	Integer,
	INTEGER_BASE,
	unsigned_greater_than,
};

pub(crate) fn unsigned_integer_add(lhs: &Integer, rhs: &Integer) -> Integer {
	let (larger, smaller) = if lhs.digits.len() > rhs.digits.len() {
		(lhs, rhs)
	} else {
		(rhs, lhs)
	};
	let mut digits = Vec::with_capacity(larger.digits.len() + 1);
	let mut carry: u32 = 0;
	
	for i in 0..smaller.digits.len() {
		let sum = larger.digits[i] as u64 + smaller.digits[i] as u64 + carry as u64;
		carry = (sum >> 32) as u32;
		digits.push(sum as u32);
	}

	for i in smaller.digits.len()..larger.digits.len() {
		let sum = larger.digits[i] as u64 + carry as u64;
		carry = (sum >> 32) as u32;
		digits.push(sum as u32);
		if carry == 0 {
			digits.extend_from_slice(&larger.digits[i + 1..]);
			break;
		}
	}

	if carry != 0 {
		digits.push(carry);
	}

	Integer {
		positive: true,
		digits,
	}
}

pub(crate) fn unsigned_integer_sub(lhs: &Integer, rhs: &Integer) -> Integer {
	let (larger, smaller, positive) = if unsigned_greater_than(lhs, rhs) {
		(lhs, rhs, true)
	} else {
		(rhs, lhs, false)
	};
	let mut digits = Vec::with_capacity(larger.digits.len());
	let mut borrow = 0u64;

	for i in 0..smaller.digits.len() {
		let right_op = smaller.digits[i] as u64 + borrow;
		let left_op = larger.digits[i] as u64;
		let new_digit = if right_op > left_op {
			INTEGER_BASE + left_op - right_op
		} else {
			left_op - right_op
		};
		digits.push(new_digit as u32);
		borrow = new_digit >> 32;
	}

	for i in smaller.digits.len()..larger.digits.len() {
		let left_op = larger.digits[i] as u64;
		let new_digit = if borrow > left_op {
			INTEGER_BASE + left_op - borrow
		} else {
			left_op - borrow
		};
		digits.push(new_digit as u32);
		borrow = new_digit >> 32;
		if borrow == 0 {
			digits.extend_from_slice(&larger.digits[i + 1..]);
			break;
		}
	}

	let mut result = Integer {
		positive,
		digits,
	};
	result.trim();

	result
}

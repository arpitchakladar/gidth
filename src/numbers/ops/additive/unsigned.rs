use crate::numbers::{
	Integer,
	INTEGER_BASE,
};

pub(crate) fn unsigned_integer_add(lhs: &Integer, rhs: &Integer) -> Integer {
	let (lint, sint) = if lhs.digits.len() > rhs.digits.len() {
		(lhs, rhs)
	} else {
		(rhs, lhs)
	};
	let mut digits = Vec::with_capacity(lint.digits.len() + 1);
	let mut reg: u128;
	let mut rem: u64 = 0;
	for i in 0..sint.digits.len() {
		reg = lint.digits[i] as u128 + sint.digits[i] as u128 + rem as u128;
		rem = (reg >> 64) as u64;
		digits.push(reg as u64);
	}

	let mut j = lint.digits.len();

	for i in sint.digits.len()..lint.digits.len() {
		reg = lint.digits[i] as u128 + rem as u128;
		rem = (reg >> 64) as u64;
		digits.push(reg as u64);
		if rem == 0 {
			j = i + 1;
			break;
		}
	}

	for i in j..lint.digits.len() {
		digits.push(lint.digits[i]);
	}

	if rem != 0 {
		digits.push(rem);
	}

	Integer {
		positive: true,
		digits,
	}
}

pub(crate) fn unsigned_integer_sub(lhs: &Integer, rhs: &Integer) -> Integer {
	let (lint, sint, positive) = if lhs >= rhs {
		(lhs, rhs, true)
	} else {
		(rhs, lhs, false)
	};
	let mut digits = Vec::with_capacity(lint.digits.len());
	let mut carried = 0u8;
	for i in 0..sint.digits.len() {
		let current_lint = lint.digits[i];
		let current_sint = sint.digits[i];
		if current_lint < current_sint + carried as u64 {
			digits.push(((INTEGER_BASE + current_lint as u128) - carried as u128 - current_sint as u128) as u64);
			carried = 1u8;
		} else {
			digits.push(current_lint - current_sint - carried as u64);
		}
	}
	let i = sint.digits.len();

	if lint.digits.len() > sint.digits.len() && carried > 0 {
		digits.push(lint.digits[i] - carried as u64);
	}

	for i in (i + 1)..lint.digits.len() {
		digits.push(lint.digits[i]);
	}

	Integer {
		positive,
		digits,
	}
}

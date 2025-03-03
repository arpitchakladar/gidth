use crate::numbers::{
	Integer,
	INTEGER_BASE,
	unsigned_greater_than_equal,
};

pub(crate) fn unsigned_integer_add(lhs: &Integer, rhs: &Integer) -> Integer {
	let (lint, sint) = if lhs.digits.len() > rhs.digits.len() {
		(lhs, rhs)
	} else {
		(rhs, lhs)
	};
	let mut digits = Vec::with_capacity(lint.digits.len() + 1);
	let mut reg: u64;
	let mut rem: u32 = 0;
	for i in 0..sint.digits.len() {
		reg = lint.digits[i] as u64 + sint.digits[i] as u64 + rem as u64;
		rem = (reg >> 32) as u32;
		digits.push(reg as u32);
	}

	let mut j = lint.digits.len();

	for i in sint.digits.len()..lint.digits.len() {
		reg = lint.digits[i] as u64 + rem as u64;
		rem = (reg >> 32) as u32;
		digits.push(reg as u32);
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
	let (lint, sint, positive) = if unsigned_greater_than_equal(&lhs, &rhs) {
		(lhs, rhs, true)
	} else {
		(rhs, lhs, false)
	};
	let mut digits = Vec::with_capacity(lint.digits.len() + 1);
	let mut carried = 0u8;
	for i in 0..sint.digits.len() {
		let current_lint = lint.digits[i];
		let current_sint = sint.digits[i];
		if current_lint < current_sint + carried as u32 {
			digits.push(((INTEGER_BASE + current_lint as u64) - carried as u64 - current_sint as u64) as u32);
			carried = 1u8;
		} else {
			digits.push(current_lint - current_sint - carried as u32);
		}
	}
	let mut i = sint.digits.len();

	if carried > 0 {
		for j in i..lint.digits.len() {
			let current = lint.digits[j];
			if carried as u32 > current {
				digits.push(((INTEGER_BASE + current as u64) - carried as u64) as u32);
				carried = 1u8;
			} else {
				digits.push(current - carried as u32);
				i = j + 1;
				break;
			}
		}
	}

	for j in i..lint.digits.len() {
		digits.push(lint.digits[j]);
	}

	Integer {
		positive,
		digits,
	}
}

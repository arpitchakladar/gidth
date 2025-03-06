use crate::number::BigInt;

impl BigInt {
	pub(crate) fn unsigned_add(&self, rhs: &BigInt) -> BigInt {
		let (larger, smaller) = if self.digits.len() > rhs.digits.len() {
			(self, rhs)
		} else {
			(rhs, self)
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

		BigInt {
			positive: true,
			digits,
		}
	}

	pub(crate) fn unsigned_sub(&self, rhs: &BigInt) -> BigInt {
		let (larger, smaller, positive) = if BigInt::unsigned_greater_than(self, rhs) {
			(self, rhs, true)
		} else {
			(rhs, self, false)
		};
		let mut digits = Vec::with_capacity(larger.digits.len());
		let mut borrow = 0u64;

		for i in 0..smaller.digits.len() {
			let right_op = smaller.digits[i] as u64 + borrow;
			let left_op = larger.digits[i] as u64;
			let new_digit = if right_op > left_op {
				BigInt::BASE + left_op - right_op
			} else {
				left_op - right_op
			};
			digits.push(new_digit as u32);
			borrow = new_digit >> 32;
		}

		for i in smaller.digits.len()..larger.digits.len() {
			let left_op = larger.digits[i] as u64;
			let new_digit = if borrow > left_op {
				BigInt::BASE + left_op - borrow
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

		let mut result = BigInt {
			positive,
			digits,
		};
		result.trim();

		result
	}
}

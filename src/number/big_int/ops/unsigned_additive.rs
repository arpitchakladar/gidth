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

		let (digits, borrow) = larger.digits
			.iter()
			.copied()
			.zip(smaller.digits.iter().copied())
			.fold(
				(
					Vec::with_capacity(
						larger.digits.len(),
					),
					0u64,
				),
				|(mut digits, borrow), (left_digit, right_digit)| {
					let right_digit = right_digit as u64 + borrow;
					let (new_digit, overflowed) = left_digit
						.wrapping_sub(right_digit as u32)
						.overflowing_sub((right_digit >> 32) as u32);
					digits.push(new_digit);
					(digits, overflowed as u64)
				},
			);

		let (digits, _) = larger.digits[smaller.digits.len()..]
			.iter()
			.copied()
			.fold(
				(
					digits,
					borrow,
				),
				|(mut digits, borrow), digit| {
					let (new_digit, overflowed) = digit
						.overflowing_sub(digit);
					digits.push(new_digit);
					(digits, overflowed as u64)
				},
			);

		let mut result = BigInt {
			positive,
			digits,
		};
		result.trim();

		result
	}
}

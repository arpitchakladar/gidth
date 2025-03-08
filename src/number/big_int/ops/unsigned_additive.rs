use crate::number::BigInt;

impl BigInt {
	pub(crate) fn unsigned_add(&self, rhs: &BigInt) -> BigInt {
		let (larger, smaller) = if self.digits.len() > rhs.digits.len() {
			(self, rhs)
		} else {
			(rhs, self)
		};

		let (digits, carry) = larger.digits
			.iter()
			.copied()
			.zip(
				smaller.digits
					.iter()
					.copied(),
			)
			.fold(
				(
					Vec::with_capacity(larger.digits.len() + 1),
					0u64,
				),
				|(mut digits, carry), (left_digit, right_digit)| {
					let sum = left_digit as u64 + right_digit as u64 + carry;
					digits.push(sum as u32);
					(digits, sum >> 32)
				},
			);

		let (mut digits, carry) = larger.digits[smaller.digits.len()..]
			.iter()
			.copied()
			.fold(
				(digits, carry),
				|(mut digits, carry), digit| {
					let sum = digit as u64 + carry;
					digits.push(sum as u32);
					(digits, sum >> 32)
				},
			);

		if carry != 0 {
			digits.push(carry as u32);
		}

		BigInt::from(digits)
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
			.zip(
				smaller
					.digits
					.iter()
					.copied(),
			)
			.fold(
				(
					Vec::with_capacity(larger.digits.len()),
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
				(digits, borrow),
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

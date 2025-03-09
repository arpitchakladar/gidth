use crate::number::BigInt;

impl BigInt {
	pub(crate) fn u_add_in(&self, rhs: &BigInt, result: &mut BigInt) {
		let (larger, smaller) = if self.digits.len() > rhs.digits.len() {
			(self, rhs)
		} else {
			(rhs, self)
		};

		let carry = larger.digits
			.iter()
			.copied()
			.zip(
				smaller.digits
					.iter()
					.copied(),
			)
			.fold(
				0u64,
				|carry, (left_digit, right_digit)| {
					let sum = left_digit as u64 + right_digit as u64 + carry;
					result.digits.push(sum as u32);

					sum >> 32
				},
			);

		let carry = larger.digits[smaller.digits.len()..]
			.iter()
			.copied()
			.fold(
				carry,
				|carry, digit| {
					let sum = digit as u64 + carry;
					result.digits.push(sum as u32);

					sum >> 32
				},
			);

		if carry != 0 {
			result.digits.push(carry as u32);
		}
	}

	pub(crate) fn u_sub_in(&self, rhs: &BigInt, result: &mut BigInt) {
		let (larger, smaller, positive) = if BigInt::u_gt(self, rhs) {
			(self, rhs, true)
		} else {
			(rhs, self, false)
		};

		let borrow = larger.digits
			.iter()
			.copied()
			.zip(
				smaller
					.digits
					.iter()
					.copied(),
			)
			.fold(
				0u64,
				|borrow, (left_digit, right_digit)| {
					let (new_digit, overflowed) = (left_digit as u64)
						.overflowing_sub(right_digit as u64 + borrow);
					result.digits.push(new_digit as u32);

					overflowed as u64
				},
			);

		larger.digits[smaller.digits.len()..]
			.iter()
			.copied()
			.fold(
				borrow,
				|borrow, digit| {
					let (new_digit, overflowed) = digit
						.overflowing_sub(borrow as u32);
					result.digits.push(new_digit);

					overflowed as u64
				},
			);

		result.positive = positive;
		result.trim();
	}
}

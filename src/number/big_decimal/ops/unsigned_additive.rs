use crate::number::BigDecimal;

impl BigDecimal {
	pub(crate) fn unsigned_add(&self, rhs: &BigDecimal, result: &mut BigDecimal) {
		let (longest_decimal, shortest_decimal) = {
			if self.decimal_pos > rhs.decimal_pos {
				(self, rhs)
			} else {
				(rhs, self)
			}
		};

		result.decimal_pos = longest_decimal.decimal_pos;

		let decimal_len_diff = longest_decimal.decimal_pos - shortest_decimal.decimal_pos;
		let end_pos = decimal_len_diff.min(longest_decimal.digits.len());
		let shorter = end_pos == longest_decimal.digits.len();

		longest_decimal.digits[..end_pos]
			.iter()
			.copied()
			.for_each(|digit| result.digits.push(digit));

		if shorter {
			for _ in end_pos..decimal_len_diff {
				result.digits.push(0u32);
			}

			shortest_decimal.digits
				.iter()
				.copied()
				.for_each(|digit| result.digits.push(digit));

			return;
		}

		let carry = longest_decimal.digits[end_pos..]
			.iter()
			.copied()
			.zip(
				shortest_decimal.digits
					.iter()
					.copied(),
			)
			.fold(
				0u64,
				|carry, (ld, sd)| {
					let sum = ld as u64 + sd as u64 + carry;
					result.digits.push(sum as u32);

					sum >> 32
				},
			);

		let (longest_whole, shortest_whole) = if self.order() > rhs.order() {
			(self, rhs)
		} else {
			(rhs, self)
		};

		let remaining_start = (longest_whole.decimal_pos + shortest_whole.digits.len())
			.saturating_sub(shortest_whole.decimal_pos);
		let carry = longest_whole.digits[remaining_start..]
			.iter()
			.copied()
			.fold(
				carry,
				|carry, ld| {
					let sum = ld as u64 + carry;
					result.digits.push(sum as u32);

					sum >> 32
				},
			);

		if carry != 0u64 {
			result.digits.push(carry as u32);
		}
	}

	pub(crate) fn unsigned_sub(&self, rhs: &BigDecimal, result: &mut BigDecimal) {
		let (larger, smaller, positive) = if BigDecimal::unsigned_greater_than(self, rhs) {
			(self, rhs, true)
		} else {
			(rhs, self, false)
		};

		let (larger_remain, smaller_remain, borrow) = if larger.decimal_pos >= smaller.decimal_pos {
			let end_pos = larger.decimal_pos - smaller.decimal_pos;
			larger.digits[..end_pos]
				.iter()
				.copied()
				.for_each(|digit| result.digits.push(digit));

			(
				&larger.digits[end_pos..],
				&smaller.digits[..],
				0u64
			)
		} else {
			let decimal_pos_diff = smaller.decimal_pos - larger.decimal_pos;
			let end_pos = std::cmp::min(
				decimal_pos_diff,
				smaller.digits.len(),
			);
			let borrow = smaller.digits[..end_pos]
				.iter()
				.copied()
				.fold(
					0u64,
					|borrow, digit| {
						result.digits
							.push((BigDecimal::BASE - digit as u64 - borrow) as u32);

						1u64
					}
				);

			let borrow = if end_pos == decimal_pos_diff {
				borrow
			} else {
				result.digits.push(0u32.wrapping_sub(borrow as u32));
				for _ in (end_pos + 1)..decimal_pos_diff {
					result.digits.push(0u32);
				}

				0u64
			};

			(
				&larger.digits[..],
				&smaller.digits[end_pos..],
				borrow,
			)
		};

		let borrow = larger_remain
			.iter()
			.copied()
			.zip(
				smaller_remain
					.iter()
					.copied(),
			)
			.fold(
				borrow,
				|borrow, (ld, sd)| {
					let (new_digit, overflowed) = (ld as u64)
						.overflowing_sub(sd as u64 + borrow);
					result.digits.push(new_digit as u32);

					overflowed as u64
				},
			);
		larger_remain[smaller_remain.len()..]
			.iter()
			.copied()
			.fold(
				borrow,
				|borrow, ld| {
					let (new_digit, overflowed) = ld
						.overflowing_sub(borrow as u32);
					result.digits.push(new_digit);

					overflowed as u64
				},
			);

		result.positive = positive;
		result.decimal_pos = std::cmp::max(self.decimal_pos, rhs.decimal_pos);
	}
}

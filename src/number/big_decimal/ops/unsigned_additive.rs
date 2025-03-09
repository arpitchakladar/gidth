use crate::number::BigDecimal;

impl BigDecimal {
	pub fn unsigned_add(&self, rhs: &BigDecimal, result: &mut BigDecimal) {
		let (longest_decimal, shortest_decimal) = {
			if self.decimal_pos > rhs.decimal_pos {
				(self, rhs)
			} else {
				(rhs, self)
			}
		};

		let (longest_whole, shortest_whole) = {
			let lhs_whole_len = (self.digits.len() as isize).saturating_sub(self.decimal_pos as isize);
			let rhs_whole_len = (rhs.digits.len() as isize).saturating_sub(rhs.decimal_pos as isize);
			if lhs_whole_len > rhs_whole_len {
				(self, rhs)
			} else {
				(rhs, self)
			}
		};

		let decimal_len_diff = longest_decimal.decimal_pos - shortest_decimal.decimal_pos;
		let end_pos = decimal_len_diff.min(longest_decimal.digits.len());
		let shorter = end_pos == longest_decimal.digits.len();

		longest_decimal.digits[..end_pos]
			.iter()
			.copied()
			.for_each(
				|digit| result.digits.push(digit),
			);

		if shorter {
			for _ in end_pos..decimal_len_diff {
				result.digits.push(0);
			}

			shortest_decimal.digits
				.iter()
				.copied()
				.for_each(|digit| result.digits.push(digit));
		} else {
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

			if carry != 0 {
				result.digits.push(carry as u32);
			}
		}

		result.decimal_pos = longest_decimal.decimal_pos;
	}
}

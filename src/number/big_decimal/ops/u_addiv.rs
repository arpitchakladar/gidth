use crate::number::BigDecimal;

impl BigDecimal {
	pub(crate) fn u_add_in(&self, rhs: &BigDecimal, result: &mut BigDecimal) {
		let (longest_decimal, shortest_decimal) = {
			if self.decimal_pos > rhs.decimal_pos {
				(self, rhs)
			} else {
				(rhs, self)
			}
		};

		result.decimal_pos = longest_decimal.decimal_pos;

		let decimal_len_diff = longest_decimal.decimal_pos - shortest_decimal.decimal_pos;
		let end_pos = decimal_len_diff.min(longest_decimal.limbs.len());
		let shorter = end_pos == longest_decimal.limbs.len();

		longest_decimal.limbs[..end_pos]
			.iter()
			.copied()
			.for_each(|limb| result.limbs.push(limb));

		if shorter {
			for _ in end_pos..decimal_len_diff {
				result.limbs.push(0u32);
			}

			shortest_decimal.limbs
				.iter()
				.copied()
				.for_each(|limb| result.limbs.push(limb));

			return;
		}

		let carry = longest_decimal.limbs[end_pos..]
			.iter()
			.copied()
			.zip(
				shortest_decimal.limbs
					.iter()
					.copied(),
			)
			.fold(
				0u64,
				|carry, (ld, sd)| {
					let sum = ld as u64 + sd as u64 + carry;
					result.limbs.push(sum as u32);

					sum >> 32
				},
			);

		let (longest_whole, shortest_whole) = if self.order() > rhs.order() {
			(self, rhs)
		} else {
			(rhs, self)
		};

		let remaining_start = (longest_whole.decimal_pos + shortest_whole.limbs.len())
			.saturating_sub(shortest_whole.decimal_pos);
		let carry = longest_whole.limbs[remaining_start..]
			.iter()
			.copied()
			.fold(
				carry,
				|carry, ld| {
					let sum = ld as u64 + carry;
					result.limbs.push(sum as u32);

					sum >> 32
				},
			);

		if carry != 0u64 {
			result.limbs.push(carry as u32);
		}
	}

	pub(crate) fn u_sub_in(&self, rhs: &BigDecimal, result: &mut BigDecimal) {
		let (larger, smaller, positive) = if BigDecimal::u_gt(self, rhs) {
			(self, rhs, true)
		} else {
			(rhs, self, false)
		};

		let (larger_remain, smaller_remain, borrow) = if larger.decimal_pos >= smaller.decimal_pos {
			let end_pos = larger.decimal_pos - smaller.decimal_pos;
			larger.limbs[..end_pos]
				.iter()
				.copied()
				.for_each(|limb| result.limbs.push(limb));

			(
				&larger.limbs[end_pos..],
				&smaller.limbs[..],
				0u32
			)
		} else {
			let decimal_pos_diff = smaller.decimal_pos - larger.decimal_pos;
			let end_pos = std::cmp::min(
				decimal_pos_diff,
				smaller.limbs.len(),
			);
			let borrow = smaller.limbs[..end_pos]
				.iter()
				.copied()
				.fold(
					0u32,
					|borrow, limb| {
						result.limbs
							.push(
								0u32
									.wrapping_sub(limb)
									.wrapping_sub(borrow)
							);

						1u32
					}
				);

			let borrow = if end_pos == decimal_pos_diff {
				borrow
			} else {
				result.limbs.push(0u32.wrapping_sub(borrow as u32));
				for _ in (end_pos + 1)..decimal_pos_diff {
					result.limbs.push(0u32);
				}

				0u32
			};

			(
				&larger.limbs[..],
				&smaller.limbs[end_pos..],
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
					let (new_limb, overflowed) = (ld as u64)
						.overflowing_sub(sd as u64 + borrow as u64);
					result.limbs.push(new_limb as u32);

					overflowed as u32
				},
			);
		larger_remain[smaller_remain.len()..]
			.iter()
			.copied()
			.fold(
				borrow,
				|borrow, ld| {
					let (new_limb, overflowed) = ld
						.overflowing_sub(borrow);
					result.limbs.push(new_limb);

					overflowed as u32
				},
			);

		result.positive = positive;
		result.decimal_pos = std::cmp::max(self.decimal_pos, rhs.decimal_pos);
	}
}

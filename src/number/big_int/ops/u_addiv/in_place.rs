use crate::number::BigInt;

impl BigInt {
	pub(crate) fn u_add_in_place(
		&self,
		rhs: &BigInt,
		result: &mut BigInt,
	) {
		let (larger, smaller) = {
			if self.limbs.len() > rhs.limbs.len() {
				(self, rhs)
			} else {
				(rhs, self)
			}
		};

		let carry = larger.limbs
			.iter()
			.copied()
			.zip(
				smaller.limbs
					.iter()
					.copied(),
			)
			.fold(
				0u64,
				|carry, (left_limb, right_limb)| {
					let sum = left_limb as u64 + right_limb as u64;
					let total_sum = sum + carry;
					result.limbs.push(total_sum as u32);

					total_sum >> 32
				},
			);

		let carry = larger.limbs[smaller.limbs.len()..]
			.iter()
			.copied()
			.fold(
				carry,
				|carry, limb| {
					let sum = limb as u64 + carry;
					result.limbs.push(sum as u32);

					sum >> 32
				},
			);

		if carry != 0 {
			result.limbs.push(carry as u32);
		}
	}

	pub(crate) fn u_sub_in_place(
		&self,
		rhs: &BigInt,
		result: &mut BigInt,
	) {
		let (larger, smaller, positive) = {
			if BigInt::u_gt(self, rhs) {
				(self, rhs, true)
			} else {
				(rhs, self, false)
			}
		};

		let borrow = larger.limbs
			.iter()
			.copied()
			.zip(
				smaller
					.limbs
					.iter()
					.copied(),
			)
			.fold(
				0u64,
				|borrow, (left_limb, right_limb)| {
					let (new_limb, overflowed) = (left_limb as u64)
						.overflowing_sub(right_limb as u64 + borrow);
					result.limbs.push(new_limb as u32);

					overflowed as u64
				},
			);

		larger.limbs[smaller.limbs.len()..]
			.iter()
			.copied()
			.fold(
				borrow,
				|borrow, limb| {
					let (new_limb, overflowed) = limb
						.overflowing_sub(borrow as u32);
					result.limbs.push(new_limb);

					overflowed as u64
				},
			);

		result.positive = positive;
		result.trim();
	}
}

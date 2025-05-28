use crate::number::BigInt;

impl BigInt {
	pub(crate) fn u_add_assign(
		&mut self,
		rhs: &BigInt,
	) {
		let (s_len, no_inv) = {
			if self.limbs.len() > rhs.limbs.len() {
				(rhs.limbs.len(), true)
			} else {
				(self.limbs.len(), false)
			}
		};
		let mut carry = 0;
		let mut i = 0;
		while i < s_len {
			let sum = self.limbs[i] as u64 + rhs.limbs[i] as u64 + carry;
			self.limbs[i] = sum as u32;
			carry = sum >> 32;
			i += 1;
		}
		if no_inv {
			while i < self.limbs.len() && carry != 0 {
				let sum = self.limbs[i] as u64 + carry;
				self.limbs[i] = sum as u32;
				carry = sum >> 32;
				i += 1;
			}
		} else {
			while i < rhs.limbs.len() && carry != 0 {
				let sum = rhs.limbs[i] as u64 + carry;
				self.limbs.push(sum as u32);
				carry = sum >> 32;
				i += 1;
			}
			while i < rhs.limbs.len() {
				self.limbs.push(rhs.limbs[i]);
				i += 1;
			}
		}
		if carry != 0 {
			self.limbs.push(carry as u32);
		}
	}

	pub(crate) fn u_sub_in(
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

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

	pub(crate) fn u_sub_assign(
		&mut self,
		rhs: &BigInt,
	) {
		let mut borrow = 0;
		let mut i = 0;
		if BigInt::u_gt(self, rhs) {
			while i < rhs.limbs.len() {
				let (new_limb, overflowed) = (self.limbs[i] as u64)
					.overflowing_sub(rhs.limbs[i] as u64 + borrow);
				self.limbs[i] = new_limb as u32;
				borrow = overflowed as u64;
				i += 1;
			}
			while i < self.limbs.len() && borrow != 0 {
				let (new_limb, overflowed) = (self.limbs[i] as u64)
					.overflowing_sub(borrow);
				self.limbs[i] = new_limb as u32;
				borrow = overflowed as u64;
				i += 1;
			}
			self.positive = true;
		} else {
			while i < self.limbs.len() {
				let (new_limb, overflowed) = (rhs.limbs[i] as u64)
					.overflowing_sub(self.limbs[i] as u64 + borrow);
				self.limbs[i] = new_limb as u32;
				borrow = overflowed as u64;
				i += 1;
			}
			while i < rhs.limbs.len() && borrow != 0 {
				let (new_limb, overflowed) = (rhs.limbs[i] as u64)
					.overflowing_sub(borrow);
				self.limbs.push(new_limb as u32);
				borrow = overflowed as u64;
				i += 1;
			}
			self.limbs.extend(rhs.limbs[i..].iter());
			self.positive = false;
		}
		self.trim();
	}
}

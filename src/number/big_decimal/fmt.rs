use crate::number::BigDecimal;

impl BigDecimal {
	fn estimate_max_digits(&self) -> usize {
		// u32 ranges between 0 and 2^32 - 1 (10 digits)
		const DIGITS_PER_INT_LIMB: usize = 10usize;
		// for any integer if we multiply it by 10, 32 times it becomes
		// a multiple of 2^32
		const DIGITS_PER_FRAC_LIMB: usize = 32usize;

		1usize +
		std::cmp::max(
			self.limbs.len().saturating_sub(self.decimal_pos) * DIGITS_PER_INT_LIMB,
			1usize,
		) +
		std::cmp::max(
			self.decimal_pos * DIGITS_PER_FRAC_LIMB,
			1usize,
		)
	}
}

impl std::fmt::Display for BigDecimal {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut result = String::with_capacity(
			self.estimate_max_digits(),
		);
		let last_index = self.decimal_pos.min(self.limbs.len());

		let mut int_limbs: Vec<u32> = self.limbs[last_index..].to_vec();

		// Convert integer part to string
		while int_limbs.iter().any(|&x| x != 0) {
			let remainder = int_limbs.iter_mut().rev().fold(0u64, |carry, limb| {
				let current = (carry << 32) + *limb as u64;
				*limb = (current / 10) as u32; // Store quotient back
				current % 10 // New carry (remainder)
			});

			result.push((b'0' + remainder as u8) as char);
		}

		if result.is_empty() {
			result.push('0');
		}

		unsafe {
			result
				.as_mut_vec()
				.reverse();
		}

		result.push('.');

		// Prepare fractional part
		let expected_size = last_index + self.decimal_pos.saturating_sub(self.limbs.len());
		let mut frac_limbs = Vec::with_capacity(expected_size);
		frac_limbs.extend(self.limbs[..last_index].iter().copied());
		frac_limbs.extend(
			std::iter::repeat(0)
				.take(self.decimal_pos.saturating_sub(self.limbs.len()))
		);

		if frac_limbs.is_empty() {
			result.push('0');
		}

		// Convert fractional part to string
		while frac_limbs.iter().any(|&x| x != 0) {
			let carry = frac_limbs
				.iter_mut()
				.fold(
					0u64,
					|carry, limb| {
						let current = *limb as u64 * 10 + carry;
						*limb = current as u32;

						current >> 32
					}
				);

			result.push((b'0' + carry as u8) as char);
		}

		let sign = if self.positive {
			""
		} else {
			"-"
		};

		write!(f,
			"{}{}",
			sign,
			result,
		)
	}
}

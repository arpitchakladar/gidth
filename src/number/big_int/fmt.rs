use crate::number::BigInt;

impl std::fmt::Display for BigInt {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut result = String::with_capacity(
			std::cmp::max(
				self.limbs.len() * 10usize,
				1usize,
			),
		);
		let mut temp_limbs = self.limbs.clone();
		while temp_limbs.iter().any(|&x| x != 0) {
			let carry = temp_limbs
				.iter_mut()
				.rev()
				.fold(
					0u64,
					|carry, byte| {
						// Combine carry and byte
						let current = (carry << 32) + *byte as u64;
						*byte = (current / 10) as u32; // Quotient back into the byte

						current % 10 // New carry is the remainder
					},
				);

			result.push((b'0' + carry as u8) as char);
		}

		if result.is_empty() {
			result.push('0');
		}

		let sign = if self.positive {
			""
		} else {
			"-"
		};

		write!(
			f,
			"{}{}",
			sign,
			result
				.chars()
				.rev()
				.collect::<String>(),
		)
	}
}

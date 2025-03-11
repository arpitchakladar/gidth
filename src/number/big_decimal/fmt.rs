use crate::number::BigDecimal;

// TODO: Properly display the floating point

impl std::fmt::Display for BigDecimal {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut result = String::new();
		let last_index = std::cmp::min(
			self.decimal_pos,
			self.digits.len(),
		);
		let mut temp_digits = self.digits[last_index..self.digits.len()]
			.iter()
			.copied()
			.collect::<Vec<u32>>();
		while temp_digits.iter().any(|&x| x != 0) {
			let carry = temp_digits
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

		let mut result = result
			.chars()
			.rev()
			.collect::<String>();

		result.push('.');

		let mut temp_digits = self.digits[..last_index]
			.iter()
			.copied()
			.collect::<Vec<u32>>();

		while temp_digits.iter().any(|&x| x != 0) {
			let carry = temp_digits
				.iter_mut()
				.fold(
					0u64,
					|carry, byte| {
						// Combine carry and byte
						let current = (carry + *byte as u64) * 10;
						*byte = current as u32;

						current >> 32
					},
				);

			result.push((b'0' + carry as u8) as char);
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
			result,
		)
	}
}

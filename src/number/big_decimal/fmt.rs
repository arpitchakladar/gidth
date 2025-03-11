use crate::number::BigDecimal;

impl std::fmt::Display for BigDecimal {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut integer_part = String::new();
		let last_index = self.decimal_pos.min(self.digits.len());

		let mut int_digits: Vec<u32> = self.digits[last_index..].to_vec();

		// Convert integer part to string
		while int_digits.iter().any(|&x| x != 0) {
			let remainder = int_digits.iter_mut().rev().fold(0u64, |carry, digit| {
				let current = (carry << 32) + *digit as u64;
				*digit = (current / 10) as u32; // Store quotient back
				current % 10 // New carry (remainder)
			});

			integer_part.push((b'0' + remainder as u8) as char);
		}

		if integer_part.is_empty() {
			integer_part.push('0');
		}

		let mut result = integer_part
			.chars()
			.rev()
			.collect::<String>();
		result.push('.');

		// Prepare fractional part
		let mut frac_digits = self.digits[..last_index]
			.iter()
			.copied()
			.chain(
				std::iter::repeat(0)
					.take(
						self.decimal_pos.saturating_sub(self.digits.len()),
					),
			)
			.collect::<Vec<u32>>();

		if frac_digits.is_empty() {
			result.push('0');
		}

		// Convert fractional part to string
		while frac_digits.iter().any(|&x| x != 0) {
			let carry = frac_digits
				.iter_mut()
				.fold(0u64, |carry, digit| {
					let current = (carry + *digit as u64) * 10;
					*digit = current as u32;
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

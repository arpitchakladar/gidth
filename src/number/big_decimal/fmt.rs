use crate::number::BigDecimal;

// TODO: Properly display the floating point

impl std::fmt::Display for BigDecimal {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut result = String::new();
		let mut temp_digits = self.digits.clone();
		while temp_digits.iter().any(|&x| x != 0) {
			let mut carry = 0u32;
			for byte in temp_digits.iter_mut().rev() {
				let current = ((carry as u64) << 32) + *byte as u64; // Combine carry and byte
				*byte = (current / 10) as u32; // Quotient back into the byte
				carry = (current % 10) as u32; // New carry is the remainder
			}

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
		write!(f, "{}{} {}", sign, result.chars().rev().collect::<String>(), self.decimal_pos)
	}
}

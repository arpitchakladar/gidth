use crate::number::{
	BigDecimal,
	BigInt,
	Abs,
};

// TODO: Add conversion from floating point

macro_rules! impl_from_int {
	($($t:ty),*) => {
		// Signed types
		$(
		impl From<$t> for BigDecimal {
			fn from(n: $t) -> Self {
				let mut limbs = Vec::new();
				let mut num = n.abs() as u64;
				while num > 0 {
					limbs.push(num as u32);
					num >>= 32;
				}
				if limbs.is_empty() {
					limbs.push(0);
				}
				BigDecimal {
					#[allow(unused_comparisons)]
					positive: n >= 0,
					limbs,
					decimal_pos: 0,
				}
			}
		}
		)*
	};
}

impl_from_int!(u8, u16, u32, u64, i8, i16, i32, i64);

impl From<Vec<u32>> for BigDecimal {
	fn from(limbs: Vec<u32>) -> Self {
		BigDecimal {
			positive: true,
			limbs,
			decimal_pos: 0,
		}
	}
}

impl From<&str> for BigDecimal {
	fn from(s: &str) -> Self {
		let mut limbs = Vec::new();
		let positive = !s.starts_with('-');
		let mut temp_digit_chunks: Vec<u32> = s.as_bytes()
			.rchunks(9)
			.rev()
			.filter_map(|chunk| std::str::from_utf8(chunk).ok()?.parse().ok())
			.collect();
		const CHUNK_SIZE: u64 = 1_000_000_000u64;

		while temp_digit_chunks.iter().any(|&x| x != 0) {
			let mut carry = 0u32;
			for byte in temp_digit_chunks.iter_mut() {
				let current = (carry as u64) * CHUNK_SIZE + *byte as u64;
				*byte = (current >> 32) as u32;
				carry = current as u32;
			}
			limbs.push(carry);
		}

		if limbs.is_empty() {
			limbs.push(0);
		}

		BigDecimal {
			positive,
			limbs,
			decimal_pos: 0,
		}
	}
}

impl From<String> for BigDecimal {
	fn from(s: String) -> Self {
		s.as_str().into()
	}
}

impl From<&String> for BigDecimal {
	fn from(s: &String) -> Self {
		s.as_str().into()
	}
}

impl From<BigInt> for BigDecimal {
	fn from(num: BigInt) -> Self {
		BigDecimal {
			positive: num.positive,
			limbs: num.limbs,
			decimal_pos: 0,
		}
	}
}

use crate::number::{
	BigDecimal,
	BigInt,
	Abs,
};

macro_rules! impl_big_decimal_from_int {
	($($t:ty),*) => {
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

impl_big_decimal_from_int!(u8, u16, u32, u64, i8, i16, i32, i64);

macro_rules! impl_big_decimal_from_large_int {
	($($t:ty),*) => {
		$(
		impl From<$t> for BigDecimal {
			fn from(n: $t) -> Self {
				let mut limbs = Vec::new();
				let mut num = n.abs() as u128;
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

impl_big_decimal_from_large_int!(u128, i128);

macro_rules! impl_big_decimal_from_float {
	($($t:ty),*) => {
		// floating point types
		$(
		impl From<$t> for BigDecimal {
			fn from(n: $t) -> Self {
				let mut limbs = Vec::new();
				let u_n = n.abs() as f64;
				let mut fractional_part = u_n.fract();
				let decimal_pos = 1; // FIXED FOR NOW
				for _ in 0..decimal_pos {
					fractional_part *= BigDecimal::BASE as f64;
					limbs.push(fractional_part.trunc() as u32);
					fractional_part = fractional_part.fract();
				}
				limbs.reverse();
				let mut integer_part = u_n.trunc() as u64;
				while integer_part > 0 {
					limbs.push(integer_part as u32);
					integer_part >>= 32;
				}
				if limbs.is_empty() {
					limbs.push(0);
				}
				BigDecimal {
					#[allow(unused_comparisons)]
					positive: n >= 0.0,
					limbs,
					decimal_pos,
				}
			}
		}
		)*
	};
}

impl_big_decimal_from_float!(f32, f64);

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
		let (chunk_decimal_point, chunk_decimal_pos) = if let Some(index) = s.find('.') {
			(1, index)
		} else {
			(0, 0)
		};

		const CHUNK_SIZE: u64 = 1_000_000_000u64;

		let mut temp_frac_chunks = Vec::with_capacity(s.len() / 9);
		temp_frac_chunks.extend(
			s[(chunk_decimal_pos + chunk_decimal_point)..]
				.as_bytes()
				.chunks(9)
				.filter_map(
					|chunk|
						std::str::from_utf8(chunk)
						.ok()?
						.parse::<u32>()
						.ok(),
				)
		);

		if let Some(last_digit) = temp_frac_chunks.last_mut() {
			loop {
				let current = *last_digit as u64 * 10u64;
				if current >= CHUNK_SIZE {
					break;
				} else {
					*last_digit = current as u32;
				}
			}
		}

		let decimal_pos = temp_frac_chunks.len() + 1;

		for _ in 0..decimal_pos {
			let carry = temp_frac_chunks
				.iter_mut()
				.rev()
				.fold(
					0u64,
					|carry, chunk| {
						let prod = ((*chunk as u64) << 32) + carry;
						*chunk = (prod % CHUNK_SIZE) as u32;

						prod / CHUNK_SIZE
					},
				);

			limbs.push(carry as u32);
		}

		limbs.reverse();

		let mut temp_int_chunks = Vec::with_capacity(s.len() / 9);
		temp_int_chunks.extend(
			s[..chunk_decimal_pos]
				.as_bytes()
				.rchunks(9)
				.rev()
				.filter_map(
					|chunk|
						std::str::from_utf8(chunk)
						.ok()?
						.parse::<u32>()
						.ok(),
				)
		);

		while temp_int_chunks.iter().any(|&x| x != 0) {
			let mut carry = 0u32;
			for byte in temp_int_chunks.iter_mut() {
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
			decimal_pos,
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

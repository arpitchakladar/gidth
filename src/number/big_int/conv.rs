use crate::number::{
	BigInt,
	Abs,
};

macro_rules! impl_from_int {
	($($t:ty),*) => {
		// Unsigned types
		$(
		impl From<$t> for BigInt {
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
				BigInt {
					#[allow(unused_comparisons)]
					positive: n >= 0,
					limbs,
				}
			}
		}
		)*
	};
}

impl_from_int!(u8, u16, u32, u64, i8, i16, i32, i64);

impl From<Vec<u32>> for BigInt {
	fn from(limbs: Vec<u32>) -> Self {
		BigInt {
			positive: true,
			limbs,
		}
	}
}

macro_rules! impl_from_limbs {
	($($t:ty),*) => {
		// Unsigned types
		$(
		impl From<Vec<$t>> for BigInt {
			fn from(limbs: Vec<$t>) -> Self {
				BigInt::from(
					limbs
						.into_iter()
						.map(
							|limb|
								limb as u32
						)
						.collect::<Vec<u32>>()
				)
			}
		}

		impl<const N: usize> From<[$t; N]> for BigInt {
			fn from(arr: [$t; N]) -> Self {
				BigInt::from(
					arr
						.into_iter()
						.map(
							|limb|
								limb as u32
						)
						.collect::<Vec<u32>>()
				)
			}
		}
		)*
	};
}

impl_from_limbs!(u8, u16, i8, i16);

impl From<&str> for BigInt {
	fn from(s: &str) -> Self {
		let mut limbs = Vec::new();
		let positive = !s.starts_with('-');
		let mut temp_digit_chunks: Vec<u32> = s
			.as_bytes()
			.rchunks(9)
			.rev()
			.filter_map(
				|chunk|
					std::str::from_utf8(chunk)
						.ok()?
						.parse()
						.ok()
			)
			.collect();
		const CHUNK_SIZE: u64 = 1_000_000_000u64;

		while temp_digit_chunks.iter().any(|&x| x != 0) {
			let mut carry = 0u64;
			for byte in temp_digit_chunks.iter_mut() {
				let current = carry * CHUNK_SIZE + *byte as u64;
				*byte = (current >> 32) as u32;
				carry = current & 0xFFFFFFFF;
			}
			limbs.push(carry as u32);
		}

		if limbs.is_empty() {
			limbs.push(0);
		}

		BigInt {
			positive,
			limbs,
		}
	}
}

impl From<String> for BigInt {
	fn from(s: String) -> Self {
		s.as_str().into()
	}
}

impl From<&String> for BigInt {
	fn from(s: &String) -> Self {
		s.as_str().into()
	}
}

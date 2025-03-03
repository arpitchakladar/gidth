#[derive(Debug, Clone)]
pub struct BigInt {
	pub(crate) positive: bool,
	pub(crate) digits: Vec<u32>,
}

pub const INTEGER_BASE: u64 = u32::MAX as u64 + 1;

impl BigInt {
	pub fn new<T>(value: T) -> Self
	where
		T: Into<BigInt>,
	{
		value.into()
	}

	pub fn abs(mut self) -> Self {
		self.positive = true;
		self
	}

	pub fn trim(&mut self) {
		while self.digits.last() == Some(&0) {
			self.digits.pop();
		}
	}
}

impl std::fmt::Display for BigInt {
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
		write!(f, "{}{}", sign, result.chars().rev().collect::<String>())
	}
}

fn to_positive<T>(n: T) -> u128
where
	T: PartialOrd + std::ops::Neg<Output = T> + Copy + Default,
	i128: From<T>,
{
	let n: i128 = n.into();
	if n >= 0 {
		n as u128
	} else {
		(-n) as u128
	}
}

macro_rules! impl_from_int {
	($($t:ty),*; $($s:ty),*) => {
		// Unsigned types
		$(
		impl From<$t> for BigInt {
			fn from(n: $t) -> Self {
				let mut digits = Vec::new();
				let mut num = n as u128;
				while num > 0 {
					digits.push(num as u32);
					num = num >> 32;
				}
				if digits.is_empty() {
					digits.push(0);
				}
				BigInt {
					positive: true,
					digits,
				}
			}
		}
		)*

		// Signed types
		$(
		impl From<$s> for BigInt {
			fn from(n: $s) -> Self {
				let mut digits = Vec::new();
				let mut num = to_positive(n);
				while num > 0 {
					digits.push(num as u32);
					num = num >> 32;
				}
				if digits.is_empty() {
					digits.push(0);
				}
				BigInt {
					positive: n >= 0,
					digits,
				}
			}
		}
		)*
	};
}

impl_from_int!(u8, u16, u32, u64, u128; i8, i16, i32, i64, i128);

impl From<Vec<u32>> for BigInt {
	fn from(digits: Vec<u32>) -> Self {
		BigInt {
			positive: true,
			digits,
		}
	}
}

impl From<&str> for BigInt {
	fn from(s: &str) -> Self {
		let mut digits = Vec::new();
		let positive = !s.starts_with('-');
		let mut temp_digits: Vec<u32> = s.as_bytes()
			.rchunks(9)
			.rev()
			.filter_map(|chunk| std::str::from_utf8(chunk).ok()?.parse().ok())
			.collect();
		const CHUNK_SIZE: u64 = 1_000_000_000u64;

		while temp_digits.iter().any(|&x| x != 0) {
			let mut carry = 0u32;
			for byte in temp_digits.iter_mut() {
				let current = (carry as u64) * CHUNK_SIZE + *byte as u64;
				*byte = (current >> 32) as u32;
				carry = current as u32;
			}
			digits.push(carry);
		}

		if digits.is_empty() {
			digits.push(0);
		}

		BigInt {
			positive,
			digits,
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

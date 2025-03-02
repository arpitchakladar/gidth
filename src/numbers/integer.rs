#[derive(Debug)]
pub struct Integer {
	pub(crate) positive: bool,
	pub(crate) digits: Vec<u8>,
}

impl Integer {
	pub fn new<T>(value: T) -> Self
	where
		T: Into<Integer>,
	{
		value.into()
	}
}

impl std::fmt::Display for Integer {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut result = String::new();
		let mut temp_digits = self.digits.clone();
		while temp_digits.iter().any(|&x| x != 0) {
			let mut carry = 0u8;
			for byte in temp_digits.iter_mut().rev() {
				let current = ((carry as u16) << 8) + *byte as u16; // Combine carry and byte
				*byte = (current / 10) as u8; // Quotient back into the byte
				carry = (current % 10) as u8; // New carry is the remainder
			}

			result.push((b'0' + carry) as char);
		}

		if result.is_empty() {
			result.push('0');
		}

		write!(f, "{}", result.chars().rev().collect::<String>())
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
		impl From<$t> for Integer {
			fn from(n: $t) -> Self {
				let mut digits = Vec::new();
				let mut num = n as u64;
				while num > 0 {
					digits.push(num as u8);
					num = num >> 8;
				}
				if digits.is_empty() {
					digits.push(0);
				}
				Integer {
					positive: true,
					digits,
				}
			}
		}
		)*

		// Signed types
		$(
		impl From<$s> for Integer {
			fn from(n: $s) -> Self {
				let mut digits = Vec::new();
				let mut num = to_positive(n);
				while num > 0 {
					digits.push(num as u8);
					num = num >> 8;
				}
				if digits.is_empty() {
					digits.push(0);
				}
				Integer {
					positive: n >= 0,
					digits,
				}
			}
		}
		)*
	};
}

impl_from_int!(u8, u16, u32, u64, u128; i8, i16, i32, i64, i128);

impl From<Vec<u8>> for Integer {
	fn from(digits: Vec<u8>) -> Self {
		Integer {
			positive: true,
			digits,
		}
	}
}

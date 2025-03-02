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
		write!(f, "{:?}", self.digits)
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

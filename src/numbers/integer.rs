pub struct Integer {
	positive: bool,
	digits: Vec<u8>,
}

impl Integer {
	pub fn new<T>(value: T) -> Self
	where
		T: Into<Integer>,
	{
		value.into()
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
				let mut num = n.abs() as u64;
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

impl_from_int!(u8, u16, u32, u64, usize; i8, i16, i32, i64, isize);

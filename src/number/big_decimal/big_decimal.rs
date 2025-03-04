#[derive(Clone)]
pub struct BigDecimal {
	pub(crate) positive: bool,
	pub(crate) digits: Vec<u32>,
	pub(crate) decimal_pos: usize,
}

impl BigDecimal {
	pub const BASE: u64 = u32::MAX as u64 + 1;

	pub fn new<T>(value: T) -> Self
	where
		T: Into<BigDecimal>,
	{
		value.into()
	}

	pub fn trim(&mut self) {
		while self.digits.last() == Some(&0) {
			self.digits.pop();
		}
	}
}

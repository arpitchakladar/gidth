#[derive(Clone)]
pub struct BigDecimal {
	pub(crate) positive: bool,
	pub digits: Vec<u32>,
	pub decimal_pos: usize,
}

impl BigDecimal {
	pub const BASE: u64 = u32::MAX as u64 + 1;

	pub fn new(positive: bool, digits: Vec<u32>, decimal_pos: usize) -> Self {
		BigDecimal {
			positive,
			digits,
			decimal_pos,
		}
	}

	pub fn with_capacity(len: usize) -> Self {
		Self::new(true, Vec::with_capacity(len), 0)
	}

	pub fn trim(&mut self) {
		while self.digits.last() == Some(&0) {
			self.digits.pop();
		}
	}
}

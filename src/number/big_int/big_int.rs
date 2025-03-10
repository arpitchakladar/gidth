use crate::number::Integer;

#[derive(Clone)]
pub struct BigInt {
	pub(crate) positive: bool,
	pub(crate) digits: Vec<u32>,
}

impl BigInt {
	pub const BASE: u64 = u32::MAX as u64 + 1;

	pub fn new(positive: bool, digits: Vec<u32>) -> Self {
		Self {
			positive,
			digits,
		}
	}

	pub fn with_capacity(len: usize) -> Self {
		Self {
			positive: true,
			digits: Vec::with_capacity(len),
		}
	}

	pub fn clear(&mut self) {
		self.digits.clear();
	}

	pub fn trim(&mut self) {
		while self.digits.last() == Some(&0) {
			self.digits.pop();
		}
	}
}

impl Integer for BigInt {}

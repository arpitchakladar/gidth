use crate::number::Int;

#[derive(Clone)]
pub struct BigInt {
	pub(crate) positive: bool,
	pub(crate) limbs: Vec<u32>,
}

impl BigInt {
	pub const BASE: u64 = u32::MAX as u64 + 1;

	pub fn with_capacity(len: usize) -> Self {
		Self {
			positive: true,
			limbs: Vec::with_capacity(len),
		}
	}

	pub fn clear(&mut self) {
		self.limbs.clear();
	}

	pub fn trim(&mut self) {
		while self.limbs.last() == Some(&0) {
			self.limbs.pop();
		}
	}
}

impl Int for BigInt {}

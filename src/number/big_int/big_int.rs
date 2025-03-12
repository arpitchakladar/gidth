use crate::number::{
	Int,
	Zero,
	One,
};

#[derive(Clone)]
pub struct BigInt {
	pub(crate) positive: bool,
	pub(crate) limbs: Vec<u32>,
}

impl BigInt {
	pub const BASE: u64 = u32::MAX as u64 + 1;

	pub fn zero() -> Self {
		Self {
			positive: true,
			limbs: vec![0u32],
		}
	}

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

impl Zero for BigInt {
	fn zero() -> Self {
		Self {
			positive: true,
			limbs: vec![0u32],
		}
	}

	fn is_zero(&self) -> bool {
		!self.limbs.iter().copied().any(|x| x != 0)
	}
}

impl One for BigInt {
	fn one() -> Self {
		Self {
			positive: true,
			limbs: vec![1u32],
		}
	}

	fn is_one(&self) -> bool {
		self.limbs.len() > 0 &&
		self.limbs[0] == 1 &&
		!self.limbs[1..]
			.iter()
			.copied()
			.any(|x| x != 0)
	}
}

impl Int for BigInt {}

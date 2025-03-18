use inherent::inherent;

use gidth_macros::satisfies;

use crate::number::{
	Int,
	Real,
	Zero,
	One,
};

#[derive(Clone, Debug)]
#[satisfies(Real, Int)]
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

#[inherent]
impl Zero for BigInt {
	pub fn zero() -> Self {
		Self {
			positive: true,
			limbs: vec![0u32],
		}
	}

	pub fn is_zero(&self) -> bool {
		!self.limbs.iter().copied().any(|x| x != 0)
	}
}

#[inherent]
impl One for BigInt {
	pub fn one() -> Self {
		Self {
			positive: true,
			limbs: vec![1u32],
		}
	}

	pub fn is_one(&self) -> bool {
		self.limbs.len() > 0 &&
		self.limbs[0] == 1 &&
		!self.limbs[1..]
			.iter()
			.copied()
			.any(|x| x != 0)
	}
}

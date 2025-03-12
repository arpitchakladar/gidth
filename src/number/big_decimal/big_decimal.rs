use crate::number::{
	Zero,
	Real,
	One,
};

#[derive(Clone, Debug)]
pub struct BigDecimal {
	pub(crate) positive: bool,
	pub(crate) limbs: Vec<u32>,
	pub(crate) decimal_pos: usize,
}

impl BigDecimal {
	pub const BASE: u64 = u32::MAX as u64 + 1;

	pub fn with_capacity(len: usize) -> Self {
		Self {
			positive: true,
			limbs: Vec::with_capacity(len),
			decimal_pos: 0,
		}
	}

	pub fn trim(&mut self) {
		while self.limbs.last() == Some(&0) {
			self.limbs.pop();
		}
	}

	// Gives the place value of the most significant limb
	pub(crate) fn order(&self) -> isize {
		self.limbs.len() as isize - self.decimal_pos as isize
	}
}

impl Zero for BigDecimal {
	fn zero() -> Self {
		Self {
			positive: true,
			limbs: vec![0u32],
			decimal_pos: 0,
		}
	}

	fn is_zero(&self) -> bool {
		!self.limbs.iter().copied().any(|x| x != 0)
	}
}

impl One for BigDecimal {
	fn one() -> Self {
		Self {
			positive: true,
			limbs: vec![1u32],
			decimal_pos: 0,
		}
	}

	fn is_one(&self) -> bool {
		self.limbs.len() > 0 &&
		self.order() >= 0 &&
		self.limbs[self.decimal_pos] == 1 &&
		!self.limbs[self.decimal_pos + 1..]
			.iter()
			.copied()
			.any(|x| x != 0)
	}
}

impl Real for BigDecimal {}

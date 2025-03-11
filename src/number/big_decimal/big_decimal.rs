#[derive(Clone, Debug)]
pub struct BigDecimal {
	// TODO: Make these accessibilty pub(crate)
	pub positive: bool,
	pub limbs: Vec<u32>,
	pub decimal_pos: usize,
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

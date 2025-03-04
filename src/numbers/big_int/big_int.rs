#[derive(Clone)]
pub struct BigInt {
	pub(crate) positive: bool,
	pub(crate) digits: Vec<u32>,
}


impl BigInt {
	pub const BASE: u64 = u32::MAX as u64 + 1;

	pub fn new<T>(value: T) -> Self
	where
		T: Into<BigInt>,
	{
		value.into()
	}

	pub fn abs(mut self) -> Self {
		self.positive = true;
		self
	}

	pub fn trim(&mut self) {
		while self.digits.last() == Some(&0) {
			self.digits.pop();
		}
	}
}

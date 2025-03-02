use crate::numbers::Integer;

impl PartialEq for Integer {
	fn eq(&self, other: &Self) -> bool {
		self.positive == other.positive && self.digits == other.digits
	}
}

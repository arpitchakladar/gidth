use crate::numbers::BigInt;

impl PartialEq for BigInt {
	fn eq(&self, other: &Self) -> bool {
		self.positive == other.positive && self.digits == other.digits
	}
}

impl Eq for BigInt {}

impl PartialOrd for BigInt {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if self.positive != other.positive {
			return Some(if self.positive { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less });
		}

		if self.digits.len() != other.digits.len() {
			let ord = self.digits.len().cmp(&other.digits.len());
			return Some(if self.positive { ord } else { ord.reverse() });
		}

		for (a, b) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
			match a.cmp(b) {
				std::cmp::Ordering::Equal => continue,
				ord => return Some(if self.positive { ord } else { ord.reverse() }),
			}
		}

		Some(std::cmp::Ordering::Equal)
	}
}

impl Ord for BigInt {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl BigInt {
	pub(crate) fn unsigned_greater_than(&self, rhs: &BigInt) -> bool {
		if self.digits.len() > rhs.digits.len() {
			true
		} else if self.digits.len() < rhs.digits.len() {
			false
		} else {
			for (a, b) in self.digits.iter().rev().zip(rhs.digits.iter().rev()) {
				if a > b {
					return true;
				} else if a < b {
					return false;
				}
			}

			false
		}
	}
}

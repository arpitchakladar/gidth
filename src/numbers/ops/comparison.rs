use crate::numbers::Integer;

impl PartialEq for Integer {
	fn eq(&self, other: &Self) -> bool {
		self.positive == other.positive && self.digits == other.digits
	}
}

impl Eq for Integer {}

impl PartialOrd for Integer {
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

impl Ord for Integer {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

// impl PartialOrd for &Integer {
// 	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
// 		(*self).partial_cmp(*other)
// 	}
// }
//
// impl Ord for &Integer {
// 	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
// 		(*self).cmp(*other)
// 	}
// }

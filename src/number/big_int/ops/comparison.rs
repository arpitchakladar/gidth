use crate::number::BigInt;

impl PartialEq for BigInt {
	fn eq(&self, other: &Self) -> bool {
		self.positive == other.positive && self.digits == other.digits
	}
}

impl Eq for BigInt {}

impl PartialOrd for BigInt {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		let ord = {
			if self.positive != other.positive {
				std::cmp::Ordering::Greater
			} else if self.digits.len() != other.digits.len() {
				self.digits
					.len()
					.cmp(&other.digits.len())
			} else {
				self.digits
					.iter()
					.rev()
					.zip(other.digits.iter().rev())
					.find_map(|(left, right)|
						match left.cmp(right) {
							std::cmp::Ordering::Equal => None,
							ord => Some(ord)
						}
					)
					.unwrap_or(std::cmp::Ordering::Equal)
			}
		};

		Some(
			if self.positive {
				ord
			} else {
				ord.reverse()
			}
		)
	}
}

impl Ord for BigInt {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl BigInt {
	pub(crate) fn unsigned_greater_than(&self, rhs: &BigInt) -> bool {
		match self.digits.len().cmp(&rhs.digits.len()) {
			std::cmp::Ordering::Greater => true,
			std::cmp::Ordering::Less => false,
			std::cmp::Ordering::Equal =>
				self.digits
					.iter()
					.rev()
					.zip(rhs.digits.iter().rev())
					.find(|(left, right)| left != right)
					.map(|(left, right)| left > right)
					.unwrap_or(false),
		}
	}
}

use crate::number::BigDecimal;

impl PartialEq for BigDecimal {
	fn eq(&self, other: &Self) -> bool {
		self.positive == other.positive &&
		self.digits == other.digits &&
		self.decimal_pos == other.decimal_pos
	}
}

impl Eq for BigDecimal {}

impl PartialOrd for BigDecimal {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		let ord = {
			if self.positive != other.positive {
				std::cmp::Ordering::Greater
			} else {
				let self_order = self.order();
				let other_order = other.order();

				if self_order != other_order {
					self_order
						.cmp(&other_order)
				} else {
					self.digits
						.iter()
						.rev()
						.zip(other.digits.iter().rev())
						.find_map(|(left, right)|
							match left.cmp(right) {
								std::cmp::Ordering::Equal => None,
								ord => Some(ord),
							}
						)
						.unwrap_or(std::cmp::Ordering::Equal)
				}
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

impl Ord for BigDecimal {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl BigDecimal {
	pub(crate) fn unsigned_greater_than(&self, rhs: &BigDecimal) -> bool {
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

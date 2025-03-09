use crate::number::BigDecimal;

impl PartialEq for BigDecimal {
	fn eq(&self, rhs: &Self) -> bool {
		self.positive == rhs.positive &&
		self.digits == rhs.digits &&
		self.decimal_pos == rhs.decimal_pos
	}
}

impl Eq for BigDecimal {}

impl PartialOrd for BigDecimal {
	fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
		let ord = {
			if self.positive != rhs.positive {
				std::cmp::Ordering::Greater
			} else {
				let lhs_order = self.order();
				let rhs_order = rhs.order();

				if lhs_order != rhs_order {
					lhs_order
						.cmp(&rhs_order)
				} else {
					self.digits
						.iter()
						.rev()
						.zip(rhs.digits.iter().rev())
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
	fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
		self.partial_cmp(rhs).unwrap()
	}
}

impl BigDecimal {
	pub(crate) fn unsigned_greater_than(&self, rhs: &BigDecimal) -> bool {
		match self.order().cmp(&rhs.order()) {
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

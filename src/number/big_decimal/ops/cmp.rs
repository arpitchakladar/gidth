use std::cmp::{
	PartialOrd,
	PartialEq,
	Ord,
	Eq,
	Ordering,
};
use crate::number::BigDecimal;

impl PartialEq for BigDecimal {
	fn eq(&self, rhs: &Self) -> bool {
		self.positive == rhs.positive &&
		self.limbs == rhs.limbs &&
		self.decimal_pos == rhs.decimal_pos
	}
}

impl Eq for BigDecimal {}

impl PartialOrd for BigDecimal {
	fn partial_cmp(
		&self,
		rhs: &Self,
	) -> Option<Ordering> {
		let ord = {
			if self.positive != rhs.positive {
				Ordering::Greater
			} else {
				let lhs_order = self.order();
				let rhs_order = rhs.order();

				if lhs_order != rhs_order {
					lhs_order
						.cmp(&rhs_order)
				} else {
					self.limbs
						.iter()
						.rev()
						.zip(rhs.limbs.iter().rev())
						.find_map(|(left, right)|
							match left.cmp(right) {
								Ordering::Equal => None,
								ord => Some(ord),
							}
						)
						.unwrap_or(Ordering::Equal)
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
	fn cmp(&self, rhs: &Self) -> Ordering {
		self.partial_cmp(rhs).unwrap()
	}
}

impl BigDecimal {
	pub fn u_gt(&self, rhs: &BigDecimal) -> bool {
		match self.order().cmp(&rhs.order()) {
			Ordering::Greater => true,
			Ordering::Less => false,
			Ordering::Equal =>
				self.limbs
					.iter()
					.rev()
					.zip(rhs.limbs.iter().rev())
					.find(|(left, right)| left != right)
					.map(|(left, right)| left > right)
					.unwrap_or_else(|| {
						self.limbs.len() > rhs.limbs.len() &&
						self.limbs[rhs.limbs.len()..]
							.iter()
							.copied()
							.any(|limb| limb != 0)
					}),
		}
	}
}

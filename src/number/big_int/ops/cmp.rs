use crate::number::BigInt;

impl PartialEq for BigInt {
	fn eq(&self, other: &Self) -> bool {
		self.positive == other.positive && self.limbs == other.limbs
	}
}

impl Eq for BigInt {}

impl PartialOrd for BigInt {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		let ord = {
			if self.positive != other.positive {
				std::cmp::Ordering::Greater
			} else if self.limbs.len() != other.limbs.len() {
				self.limbs
					.len()
					.cmp(&other.limbs.len())
			} else {
				self.limbs
					.iter()
					.rev()
					.zip(other.limbs.iter().rev())
					.find_map(|(left, right)|
						match left.cmp(right) {
							std::cmp::Ordering::Equal => None,
							ord => Some(ord),
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
	pub(crate) fn u_gt(&self, rhs: &BigInt) -> bool {
		match self.limbs.len().cmp(&rhs.limbs.len()) {
			std::cmp::Ordering::Greater => true,
			std::cmp::Ordering::Less => false,
			std::cmp::Ordering::Equal =>
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

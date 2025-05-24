use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + Clone> PartialEq for Ratio<T> {
	// TODO: Incomplete fix this
	fn eq(&self, rhs: &Self) -> bool {
		self.num.clone() * &rhs.den == self.den.clone() * &rhs.num
	}
}

impl<T: Int + Clone> Eq for Ratio<T> {}

impl<T: Int + Clone + Ord> PartialOrd for Ratio<T> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl<T: Int + Clone + Ord> Ord for Ratio<T> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		(self.num.clone() * &other.den).cmp(&(other.num.clone() * &self.den))
	}
}

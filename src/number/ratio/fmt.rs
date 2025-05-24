use crate::number::{
	Int,
	Ratio,
};

impl<T: Int + std::fmt::Display> std::fmt::Display for Ratio<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let num_str = format!("{}", self.num);
		let den_str = format!("{}", self.den);

		// Make the bar as long as the longer of the two strings
		let width = num_str.len().max(den_str.len()) + 2;
		let bar = "-".repeat(width);

		// Center-align numerator and denominator
		let num_padded = format!("{:^width$}", num_str, width = width);
		let den_padded = format!("{:^width$}", den_str, width = width);

		write!(f, "{}\n{}\n{}", num_padded, bar, den_padded)
	}
}

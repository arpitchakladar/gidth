use crate::{
	linear::Matrix,
	number::Real,
};

impl<T: Real + std::fmt::Display, const R: usize, const C: usize> std::fmt::Display for Matrix<T, R, C> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Step 1: Compute max width per column
		let mut col_widths = [0usize; C];
		for col in 0..C {
			let max_width = (0..R)
				.map(|row| format!("{}", self.data[row][col]).len())
				.max()
				.unwrap_or(0);
			col_widths[col] = max_width;
		}

		// Helper fn to print horizontal lines
		fn print_line(
			f: &mut std::fmt::Formatter<'_>,
			left: char,
			mid: char,
			right: char,
			col_widths: &[usize],
		) -> std::fmt::Result {
			write!(f, "{}", left)?;
			for (i, width) in col_widths.iter().enumerate() {
				write!(f, "{}", "─".repeat(*width + 2))?; // +2 for padding spaces
				if i == col_widths.len() - 1 {
					write!(f, "{}", right)?;
				} else {
					write!(f, "{}", mid)?;
				}
			}
			writeln!(f)
		}

		// Top border
		print_line(f, '┌', '┬', '┐', &col_widths)?;

		// Rows
		for row in 0..R {
			write!(f, "│")?;
			for col in 0..C {
				let val = format!("{}", self.data[row][col]);
				// Pad left and right spaces to center content a bit
				let padding = col_widths[col].saturating_sub(val.len());
				let left_pad = padding / 2;
				let right_pad = padding - left_pad;
				write!(
					f,
					" {}{}{} ",
					" ".repeat(left_pad),
					val,
					" ".repeat(right_pad)
				)?;
				write!(f, "│")?;
			}
			writeln!(f)?;

			if row != R - 1 {
				print_line(f, '├', '┼', '┤', &col_widths)?;
			}
		}

		// Bottom border
		print_line(f, '└', '┴', '┘', &col_widths)
	}
}

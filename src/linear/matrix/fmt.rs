use crate::{
	linear::Matrix,
	number::Real,
};

impl<T: Real + std::fmt::Display, const R: usize, const C: usize> std::fmt::Display for Matrix<T, R, C> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Step 1: Compute max width per column
		let mut col_widths = [0usize; C];
		let mut row_heights = [0usize; R];
		for col in 0..C {
			let mut col_width = 0;
			for row in 0..R {
				let entry = format!("{}", self.data[row][col]);
				let mut row_height = 1;
				let mut current_col_width = 0;
				for c in entry.chars() {
					if c == '\n' {
						row_height += 1;
						col_width = col_width.max(current_col_width);
						current_col_width = 0;
					}
					current_col_width += 1;
				}
				col_width = col_width.max(current_col_width);
				row_heights[row] = row_heights[row].max(row_height);
			}
			col_widths[col] = col_width;
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
				write!(f, "{}", " ".repeat(*width + 2))?; // +2 for padding spaces
				if i == col_widths.len() - 1 {
					write!(f, "{}", right)?;
				} else {
					write!(f, "{}", mid)?;
				}
			}
			writeln!(f)
		}

		// Top border
		print_line(f, '┌', ' ', '┐', &col_widths)?;

		for row in 0..R {
			let max_row_height = row_heights[row];

			// Collect lines per column for this row
			let mut row_lines = vec![vec![String::new(); max_row_height]; C];
			for col in 0..C {
				let entry = format!("{}", self.data[row][col]);
				let lines: Vec<&str> = entry.lines().collect();
				let pad_top = (max_row_height - lines.len()) / 2;
				let pad_bottom = max_row_height - lines.len() - pad_top;

				let mut padded_lines = Vec::new();
				padded_lines.extend((0..pad_top).map(|_| ""));
				padded_lines.extend(lines);
				padded_lines.extend((0..pad_bottom).map(|_| ""));

				for (i, line) in padded_lines.iter().enumerate() {
					row_lines[col][i] = line.to_string();
				}
			}

			// Write row line-by-line
			for line_idx in 0..max_row_height {
				write!(f, "│")?;
				for col in 0..C {
					let line = &row_lines[col][line_idx];
					let padding = col_widths[col].saturating_sub(line.len());
					let left_pad = padding / 2;
					let right_pad = padding - left_pad;
					write!(
						f,
						" {}{}{} ",
						" ".repeat(left_pad),
						line,
						" ".repeat(right_pad)
					)?;
					write!(f, "{}", if col == C - 1 { "│" } else { " " })?;
				}
				writeln!(f)?;
			}

			if row != R - 1 {
				print_line(f, '│', ' ', '│', &col_widths)?;
			}
		}

		// Bottom border
		print_line(f, '└', ' ', '┘', &col_widths)
	}
}

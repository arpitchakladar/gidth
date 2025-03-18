use crate::number::BigDecimal;
use crate::number::utils::ops::div::{
	sub_from_slice,
	cmp_limb_arrays,
	mul_by_small_int,
	adj_guess_for_div,
};

impl BigDecimal {
	// NOTE: self is assumed to be the clone of the lhs with zeros
	// at the beginning, also its decimal_pos value acts as the precision for
	// the result
	pub(crate) fn u_div_in(
		&mut self,
		rhs: &BigDecimal,
		quotient: &mut BigDecimal,
	) {
		let l_lhs = self.limbs.len();
		let l_rhs = rhs.limbs.len();

		let sig_rhs = rhs.limbs[l_rhs - 1] as u64;
		let mut start = l_lhs - l_rhs;
		let mut end = l_lhs;

		let mut decimal_pos = None;
		loop {
			let reg = &mut self.limbs[start..end];
			if decimal_pos.is_none() && self.decimal_pos > start {
				decimal_pos = Some(quotient.limbs.len());
			}
			if cmp_limb_arrays(reg, &rhs.limbs) {
				let sig: u64 = if reg.len() == l_rhs {
					reg[reg.len() - 1] as u64
				} else {
					((reg[reg.len() - 1] as u64) << 32) +
					reg[reg.len() - 2] as u64
				};
				let mut guess = (sig / sig_rhs + 1) as u32;
				let mut num_limbs =
					Vec::with_capacity(
						rhs.limbs.len() + 1,
					);

				loop {
					num_limbs.clear();
					num_limbs.extend(rhs.limbs.iter());
					mul_by_small_int(&mut num_limbs, guess);
					if cmp_limb_arrays(reg, &num_limbs) {
						quotient.limbs.push(guess);
						sub_from_slice(reg, &num_limbs);
						end -= 1;
						start = end.saturating_sub(l_rhs);
						break;
					} else {
						guess -=
							adj_guess_for_div(
								reg,
								&rhs.limbs[..],
								&num_limbs[..],
							);
					}
				}
			} else if end >= start + rhs.limbs.len() {
				start = start.saturating_sub(1);
				if reg[reg.len() - 1] == 0 {
					end -= 1;
				}
				quotient.limbs.push(0u32);
			} else {
				break;
			}
		}

		quotient.limbs.reverse();
		quotient.decimal_pos = decimal_pos
			.map(|decimal_pos| quotient.limbs.len() - decimal_pos)
			.unwrap_or(0);
	}
}

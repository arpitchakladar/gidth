use crate::number::BigDecimal;
use crate::number::utils::{
	sub_from_slice,
	cmp_limb_arrays,
	mul_by_small_int,
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
						let guess_adjustment = {
							let (num_limb, sig_reg, sig_rhs) = {
								match (num_limbs.len(), reg.len()) {
									(num_len, reg_len) if num_len > reg_len => (
										((num_limbs[num_len - 1] as u64) << 32) +
											num_limbs[num_len - 2] as u64,
										reg[reg_len - 1] as u64,
										rhs.limbs[l_rhs - 1] as u64,
									),
									(num_len, reg_len) if reg_len > 1 => (
										((num_limbs[num_len - 1] as u64) << 32) +
											num_limbs[num_len - 2] as u64,
										((reg[reg_len - 1] as u64) << 32) +
											reg[reg_len - 2] as u64,
										((rhs.limbs[l_rhs - 1] as u64) << 32) +
											rhs.limbs[l_rhs - 2] as u64,
									),
									(num_len, reg_len) => (
										num_limbs[num_len - 1] as u64,
										reg[reg_len - 1] as u64,
										rhs.limbs[l_rhs - 1] as u64,
									),
								}
							};

							// Calcualte if the guess way too far off
							// The 2 is to make sure we don't overshoot it
							((num_limb - sig_reg) / (sig_rhs * 2)) as u32
						};

						// Only decrement by 1 if we are somewhat close
						guess -= std::cmp::max(1, guess_adjustment);
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

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

		loop {
			let reg = &mut self.limbs[start..end];
			if cmp_limb_arrays(reg, &rhs.limbs) {
				let sig: u64 = if reg.len() == l_rhs {
					reg[reg.len() - 1] as u64
				} else {
					((reg[reg.len() - 1] as u64) << 32) + reg[reg.len() - 2] as u64
				};
				let min = (sig / sig_rhs) as u32;
				let max = ((sig + sig_rhs - 1) / sig_rhs) as u32;

				for i in (min..=max).rev() {
					let mut num = rhs.clone();
					mul_by_small_int(&mut num.limbs, i);
					if cmp_limb_arrays(reg, &num.limbs) {
						quotient.limbs.push(i);
						let offset = sub_from_slice(reg, &num.limbs);
						end -= offset;
						start = end.saturating_sub(l_rhs);
						break;
					}
				}
			} else if start > 0 {
				start -= 1;
			} else {
				break;
			}
		}

		quotient.limbs.reverse();
		quotient.decimal_pos = self.decimal_pos;
	}
}

use crate::number::BigInt;
use crate::number::utils::ops::div::{
	sub_from_slice,
	cmp_limb_arrays,
	mul_by_small_int,
	adj_guess_for_div,
};

// Using macro for better removal of dead code
macro_rules! bigint_division {
	// NOTE: Quotient is an empty BigInt and remainder is a clone of self
	(
		$self:expr,
		$rhs:expr,
		$quotient:expr,
		$has_quotient:expr,
		$has_remainder:expr,
	) => {{
		if BigInt::u_gt($rhs, $self) {
			$quotient.limbs.push(0u32);
			return;
		} else {
			let l_lhs = $self.limbs.len();
			let l_rhs = $rhs.limbs.len();

			let sig_rhs = $rhs.limbs[l_rhs - 1] as u64;
			let mut start = l_lhs - l_rhs;
			let mut end = l_lhs;

			loop {
				let reg = &mut $self.limbs[start..end];
				if cmp_limb_arrays(reg, &$rhs.limbs) {
					let sig: u64 = if reg.len() == l_rhs {
						reg[reg.len() - 1] as u64
					} else {
						((reg[reg.len() - 1] as u64) << 32) +
						reg[reg.len() - 2] as u64
					};
					let mut guess = (sig / sig_rhs + 1) as u32;
					let mut num_limbs =
						Vec::with_capacity(
							$rhs.limbs.len() + 1,
						);

					loop {
						num_limbs.clear();
						num_limbs.extend($rhs.limbs.iter());
						mul_by_small_int(&mut num_limbs, guess);
						if cmp_limb_arrays(reg, &num_limbs) {
							$quotient.limbs.push(guess);
							sub_from_slice(reg, &num_limbs);
							end -= 1;
							start = end.saturating_sub(l_rhs);
							break;
						} else {
							guess -=
								adj_guess_for_div(
									reg,
									&$rhs.limbs[..],
									&num_limbs[..],
								);
						}
					}
				} else if end >= start + $rhs.limbs.len() {
					start = start.saturating_sub(1);
					if reg[reg.len() - 1] == 0 {
						end -= 1;
					}
					$quotient.limbs.push(0u32);
				} else {
					break;
				}
			}

			if $has_quotient {
				$quotient.limbs.reverse();
			}
			if $has_remainder {
				$self.trim();
			}
		}
	}};
}

impl BigInt {
	pub(crate) fn u_divmod_in(
		&mut self, rhs: &BigInt,
		quotient: &mut BigInt,
	) {
		bigint_division!(
			self,
			rhs,
			quotient,
			true,
			true,
		);
	}

	pub(crate) fn u_div_in(
		&mut self,
		rhs: &BigInt,
		quotient: &mut BigInt,
	) {
		bigint_division!(
			self,
			rhs,
			quotient,
			true,
			false,
		);
	}

	pub(crate) fn u_rem_in(
		&mut self,
		rhs: &BigInt,
	) {
		#[allow(deref_nullptr)]
		unsafe {
			bigint_division!(
				self,
				rhs,
				*std::ptr::null_mut::<BigInt>(),
				false,
				true,
			);
		}
	}

	pub(crate) fn u_divmod_small(&self, rhs: u32) -> (BigInt, u32) {
		let rhs = rhs as u64;
		let (quotient, remainder) = self.limbs
			.iter()
			.rev()
			.fold(
				(
					Vec::with_capacity(
						self.limbs.len(),
					),
					0u64,
				),
				|(mut quotient, mut remainder), &byte| {
					let current = (remainder << 32) + byte as u64;
					quotient.push((current / rhs) as u32);
					remainder = current % rhs;
					(quotient, remainder)
				},
			);

		(
			BigInt::from(
				quotient
					.into_iter()
					.rev()
					.collect::<Vec<u32>>(),
			),
			remainder as u32,
		)
	}
}

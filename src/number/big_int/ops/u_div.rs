use crate::number::BigInt;

#[inline]
fn sub_from_slice(lhs: &mut [u32], rhs: &[u32]) -> usize {
	let borrow = lhs
		.iter_mut()
		.zip(rhs.iter().copied())
		.fold(
			0u64,
			|borrow, (left_limb, right_limb)| {
				let (new_limb, overflowed) = (*left_limb as u64)
					.overflowing_sub(right_limb as u64 + borrow);
				*left_limb = new_limb as u32;
				overflowed as u64
			},
		);
	
	if borrow > 0 {
		if let Some(last) = lhs.last_mut() {
			*last -= borrow as u32;
		}
	}
	
	lhs.iter()
		.rposition(|&x| x != 0)
		.map(|i| lhs.len() - i - 1)
		.unwrap_or(lhs.len())
}

#[inline]
fn cmp_limb_arrays(lhs: &[u32], rhs: &[u32]) -> bool {
	match lhs.len().cmp(&rhs.len()) {
		std::cmp::Ordering::Greater => return true,
		std::cmp::Ordering::Less => return false,
		_ =>
			lhs.iter()
				.rev()
				.zip(rhs.iter().rev())
				.find(|(l, r)| l != r)
				.map(|(l, r)| l > r)
				.unwrap_or(true),
	}
}

#[inline]
fn mul_by_small_int(lhs: &mut Vec<u32>, rhs: u32) {
	let carry = lhs
		.iter_mut()
		.fold(
			0u64,
			|carry, d| {
				let reg: u64 = rhs as u64 * *d as u64 + carry;
				*d = reg as u32;
				reg >> 32
			}
		);
	
	if carry > 0 {
		lhs.push(carry as u32);
	}
}

// Using macro for better removal of dead code
macro_rules! bigint_division {
	($self:expr, $rhs:expr, $quotient:expr, $remainder:expr, $has_quotient:expr, $has_remainder:expr) => {{
		if BigInt::u_gt($rhs, $self) {
			return;
		} else {
			let l_lhs = $self.limbs.len();
			let l_rhs = $rhs.limbs.len();

			let sig_rhs = $rhs.limbs[l_rhs - 1] as u64;
			let mut start = l_lhs - l_rhs;
			let mut end = l_lhs;

			loop {
				let reg = &mut $remainder.limbs[start..end];
				if cmp_limb_arrays(reg, &$rhs.limbs) {
					let sig: u64 = if reg.len() == l_rhs {
						reg[reg.len() - 1] as u64
					} else {
						((reg[reg.len() - 1] as u64) << 32) + reg[reg.len() - 2] as u64
					};
					let min = (sig / (sig_rhs + 1)) as u32;
					let max = ((sig + 1) / sig_rhs) as u32;

					for i in (min..=max).rev() {
						let mut num = $rhs.clone();
						mul_by_small_int(&mut num.limbs, i);
						if cmp_limb_arrays(reg, &num.limbs) {
							if $has_quotient {
								$quotient.limbs.push(i);
							}
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

			if $has_quotient {
				$quotient.limbs.reverse();
			}
			if $has_remainder {
				$remainder.trim();
			}
		}
	}};
}

impl BigInt {
	pub(crate) fn u_divmod_in(
		&self, rhs: &BigInt,
		quotient: &mut BigInt,
		remainder: &mut BigInt,
	) {
		bigint_division!(
			self,
			rhs,
			quotient,
			remainder,
			true,
			true
		);
	}

	pub(crate) fn u_div_in(
		&self,
		rhs: &BigInt,
		quotient: &mut BigInt,
		remainder: &mut BigInt,
	) {
		bigint_division!(
			self,
			rhs,
			quotient,
			remainder,
			true,
			false
		);
	}

	pub(crate) fn u_rem_in(
		&self,
		rhs: &BigInt,
		remainder: &mut BigInt,
	) {
		#[allow(deref_nullptr)]
		unsafe {
			bigint_division!(
				self,
				rhs,
				*std::ptr::null_mut::<BigInt>(),
				remainder,
				false,
				true
			);
		}
	}

	pub(crate) fn u_divmod_base(&self, rhs: u32) -> (BigInt, u32) {
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

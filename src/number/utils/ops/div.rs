pub(crate) fn sub_from_slice(
	lhs: &mut [u32],
	rhs: &[u32],
) {
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
}

// checks if lhs >= rhs
pub(crate) fn cmp_limb_arrays(
	lhs: &[u32],
	rhs: &[u32],
) -> bool {
	match lhs.len().cmp(&rhs.len()) {
		std::cmp::Ordering::Greater => return true,
		std::cmp::Ordering::Less => return false,
		std::cmp::Ordering::Equal =>
			lhs
				.iter()
				.rev()
				.zip(rhs.iter().rev())
				.find(|(left, right)| left != right)
				.map(|(left, right)| left > right)
				.unwrap_or(true),
	}
}

pub(crate) fn mul_by_small_int(
	lhs: &mut Vec<u32>,
	rhs: u32,
) {
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

pub(crate) fn adj_guess_for_div(
	reg: &[u32],
	rhs_limbs: &[u32],
	num_limbs: &[u32],
) -> u32 {
	let l_rhs = rhs_limbs.len();
	let (num_limb, sig_reg, sig_rhs) = {
		match (num_limbs.len(), reg.len()) {
			(num_len, reg_len) if num_len > reg_len => (
				((num_limbs[num_len - 1] as u64) << 32) +
					num_limbs[num_len - 2] as u64,
				reg[reg_len - 1] as u64,
				rhs_limbs[l_rhs - 1] as u64,
			),
			(num_len, reg_len) if reg_len > 1 => (
				((num_limbs[num_len - 1] as u64) << 32) +
					num_limbs[num_len - 2] as u64,
				((reg[reg_len - 1] as u64) << 32) +
					reg[reg_len - 2] as u64,
				((rhs_limbs[l_rhs - 1] as u64) << 32) +
					rhs_limbs[l_rhs - 2] as u64,
			),
			(num_len, reg_len) => (
				num_limbs[num_len - 1] as u64,
				reg[reg_len - 1] as u64,
				rhs_limbs[l_rhs - 1] as u64,
			),
		}
	};

	std::cmp::max(
		// If we are close only decrement by 1
		1u32,
		// Calcualte if the guess way too far off
		// The 2 is to make sure we don't overshoot it
		((num_limb - sig_reg) / (sig_rhs * 2)) as u32,
	)
}

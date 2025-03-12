pub(crate) fn sub_from_slice(lhs: &mut [u32], rhs: &[u32]) -> usize {
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

pub(crate) fn cmp_limb_arrays(lhs: &[u32], rhs: &[u32]) -> bool {
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

pub(crate) fn mul_by_small_int(lhs: &mut Vec<u32>, rhs: u32) {
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

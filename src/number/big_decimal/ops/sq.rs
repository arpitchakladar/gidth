use crate::number::{
	BigDecimal,
	Square,
};

fn sq_term(num: &BigDecimal, res: u128, i: usize, carry: u128) -> u128 {
	let start = i.checked_sub(num.limbs.len())
		.map(|x| x + 1)
		.unwrap_or(0);
	
	let x = (start..((i + 1) / 2))
		.map(|j| num.limbs[j] as u128 * num.limbs[i - j] as u128)
		.sum::<u128>();

	(x << 1) + carry + res
}

impl BigDecimal {
	pub(crate) fn sq_in(num: &BigDecimal, result: &mut BigDecimal) {
		num.limbs
			.iter()
			.copied()
			.enumerate()
			.fold(
				0u128,
				|carry, (i, limb)| {
					let limb = limb as u128;
					let x = i * 2;
					let res = sq_term(num, limb * limb, x, carry);
					result.limbs.push(res as u32);
					let res = sq_term(num, 0, x + 1, res >> 32);
					result.limbs.push(res as u32);
					res >> 32
				},
			);

		result.decimal_pos = num.decimal_pos << 1;
		result.trim();
	}
}

impl Square for BigDecimal {
	fn sq(self) -> Self {
		let mut result = BigDecimal::with_capacity(self.limbs.len() * 2);
		BigDecimal::sq_in(
			&self,
			&mut result,
		);

		result
	}
}

use crate::number::{
	BigInt,
	Square,
};

fn square_term(num: &BigInt, res: u128, i: usize, carry: u128) -> u128 {
	let start = i.checked_sub(num.digits.len())
		.map(|x| x + 1)
		.unwrap_or(0);
	
	let x = (start..((i + 1) / 2))
		.map(|j| num.digits[j] as u128 * num.digits[i - j] as u128)
		.sum::<u128>();

	(x << 1) + carry + res
}

impl BigInt {
	pub(crate) fn square_inplace(num: &BigInt, result: &mut BigInt) {
		num.digits
			.iter()
			.copied()
			.enumerate()
			.fold(
				0u128,
				|carry, (i, digit)| {
					let digit = digit as u128;
					let x = i * 2;
					let res = square_term(num, digit * digit, x, carry);
					result.digits.push(res as u32);
					let res = square_term(num, 0, x + 1, res >> 32);
					result.digits.push(res as u32);
					res >> 32
				},
			);

		result.trim();
	}
}

impl Square for BigInt {
	fn sq(self) -> Self {
		let mut result = BigInt::with_capacity(self.digits.len() * 2);
		BigInt::square_inplace(
			&self,
			&mut result,
		);

		result
	}
}

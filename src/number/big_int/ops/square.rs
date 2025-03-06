use crate::number::BigInt;
use crate::utils::Square;

fn square_term(num: &BigInt, reg: u128, i: usize, carry: u128) -> u128 {
	let mut x = 0u128;
	let start = if i >= num.digits.len() {
		i - num.digits.len() + 1
	} else {
		0
	};
	for j in start..((i + 1) / 2) {
		x += num.digits[j] as u128 * num.digits[i - j] as u128;
	}

	(x << 1) + carry + reg as u128
}

impl BigInt {
	pub(crate) fn square_inplace(num: &BigInt, result: &mut BigInt) {
		let mut carry = 0u128;
		for i in 0..num.digits.len() {
			let d = num.digits[i] as u128;
			let x = i * 2;
			let reg = square_term(&num, d * d, x, carry);
			result.digits.push(reg as u32);
			let reg = square_term(&num, 0, x + 1, reg >> 32);
			result.digits.push(reg as u32);
			carry = reg >> 32;
		}

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

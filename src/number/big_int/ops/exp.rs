use crate::number::BigInt;

fn half_big_int(num: &mut BigInt) -> bool {
	num.digits
		.iter_mut()
		.fold(
			0u64,
			|remainder, digit| {
				let reg = (remainder << 32) + *digit as u64;
				*digit = (reg >> 1) as u32;
				reg & 1
			},
		) == 1
}

fn u_exp_in(base: &BigInt, power: &mut BigInt, result: BigInt, current: BigInt) -> (BigInt, BigInt) {
	if power.digits.len() == 1 && power.digits[0] == 0 {
		return (current, result);
	}

	let remainder = half_big_int(power);
	let (mut current, mut result) = u_exp_in(base, power, current, result);
	result.digits.clear();
	BigInt::sq_in(&current, &mut result);

	if remainder {
		current.digits.clear();
		BigInt::u_mul_in(&result, base, &mut current);
		(current, result)
	} else {
		(result, current)
	}
}

impl BigInt {
	pub(crate) fn u_exp(&self, mut power: BigInt) -> BigInt {
		let buf_size = (self.digits.len() + 1) * power.digits.len();
		let mut buf1 = BigInt::with_capacity(buf_size);
		let mut buf2 = BigInt::with_capacity(buf_size);
		buf1.digits.push(1);
		buf2.digits.push(1);
		u_exp_in(self, &mut power, buf1, buf2).0
	}
}

impl BigInt {
	pub fn exp(&self, power: &BigInt) -> BigInt {
		BigInt::u_exp(self, power.clone())
	}
}

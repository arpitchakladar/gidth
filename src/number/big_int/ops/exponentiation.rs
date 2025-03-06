use crate::number::BigInt;

fn half_big_int(num: &mut BigInt) -> bool {
	let mut remainder = 0u64;
	for d in num.digits.iter_mut() {
		let reg = (remainder << 32) + *d as u64;
		*d = (reg >> 1) as u32;
		remainder = reg & 1;
	}

	remainder == 1
}

fn inplace_exp(base: &BigInt, power: &mut BigInt, result: BigInt, current: BigInt) -> (BigInt, BigInt) {
	if power.digits.len() == 1 && power.digits[0] == 0 {
		return (current, result);
	}

	let remainder = half_big_int(power);
	let (mut current, mut result) = inplace_exp(base, power, current, result);
	result.digits.clear();
	BigInt::square_inplace(&current, &mut result);

	if remainder {
		current.digits.clear();
		BigInt::unsigned_inplace_mul(&result, base, &mut current);
		(current, result)
	} else {
		(result, current)
	}
}

impl BigInt {
	pub(crate) fn unsigned_exp(&self, mut power: BigInt) -> BigInt {
		let mut buf1 = BigInt::with_capacity((self.digits.len() + 1) * power.digits.len());
		let mut buf2 = BigInt::with_capacity((self.digits.len() + 1) * power.digits.len());
		buf1.digits.push(1);
		buf2.digits.push(1);
		inplace_exp(self, &mut power, buf1, buf2).0
	}
}

impl BigInt {
	pub fn exp(&self, power: &BigInt) -> BigInt {
		BigInt::unsigned_exp(self, power.clone())
	}
}

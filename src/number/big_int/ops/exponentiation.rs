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

fn exp(base: &BigInt, power: &mut BigInt, result: &mut BigInt) {
	if power.digits.len() == 1 && power.digits[0] == 0 {
		return;
	}

	let remainder = half_big_int(power);
	exp(base, power, result);
	let prod = &*result * &*result;

	*result = if remainder {
		base * prod
	} else {
		prod
	};
}

impl BigInt {
	pub(crate) fn unsigned_exp(&self, mut power: BigInt) -> BigInt {
		let mut result = BigInt::new(1);
		exp(self, &mut power, &mut result);
		result
	}
}

impl BigInt {
	pub fn exp(&self, power: &BigInt) -> BigInt {
		BigInt::unsigned_exp(self, power.clone())
	}
}

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

fn fast_square_term(num: &BigInt, reg: u128, i: usize, carry: u128) -> u128 {
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

pub fn fast_square(num: &BigInt, result: &mut BigInt) {
	let mut carry = 0u128;
	for i in 0..num.digits.len() {
		let d = num.digits[i] as u128;
		let x = i * 2;
		let reg = fast_square_term(num, d * d, x, carry);
		result.digits.push(reg as u32);
		let reg = fast_square_term(num, 0, x + 1, reg >> 32);
		result.digits.push(reg as u32);
		carry = reg >> 32;
	}
}

fn inplace_exp(base: &BigInt, power: &mut BigInt, result: &mut BigInt, current: &mut BigInt) {
	if power.digits.len() == 1 && power.digits[0] == 0 {
		return;
	}

	let remainder = half_big_int(power);
	inplace_exp(base, power, current, result);
	result.digits.clear();
	fast_square(&*current, result);

	if remainder {
		*result = &*result * base;
	}
}

impl BigInt {
	pub(crate) fn unsigned_exp(&self, mut power: BigInt) -> BigInt {
		let mut buf1 = BigInt::with_capacity((self.digits.len() + 1) * power.digits.len());
		let mut buf2 = BigInt::with_capacity((self.digits.len() + 1) * power.digits.len());
		buf1.digits.push(1);
		buf2.digits.push(1);
		inplace_exp(self, &mut power, &mut buf1, &mut buf2);
		if buf1.digits.len() > buf2.digits.len() {
			buf1
		} else {
			buf2
		}
	}
}

impl BigInt {
	pub fn exp(&self, power: &BigInt) -> BigInt {
		BigInt::unsigned_exp(self, power.clone())
	}
}

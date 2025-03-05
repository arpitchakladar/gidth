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

pub fn fast_square(num: &BigInt) -> BigInt {
	let result_len = num.digits.len() * 2;
	let mut result = Vec::with_capacity(result_len);
	let mut sum = 0u64;
	for i in 0..num.digits.len() {
		let d = num.digits[i] as u64;
		sum += d;
		let reg = d * d;
		result.push(reg as u32);
		result.push((reg >> 32) as u32);
	}
	let mut carry = 0u128;
	for i in 1..(result_len - 1) {
		let mut x = 0u128;
		let start = if i >= num.digits.len() {
			i - num.digits.len() + 1
		} else {
			0
		};
		for j in start..((i + 1) / 2) {
			x += num.digits[j] as u128 * num.digits[i - j] as u128;
		}
		x <<= 1;
		let sum = x + carry + result[i] as u128;
		carry = sum >> 32;
		result[i] = sum as u32;
	}
	result[result_len - 1] += carry as u32;

	BigInt::new(result)
}

fn inplace_exp(base: &BigInt, power: &mut BigInt, result: &mut BigInt) {
	if power.digits.len() == 1 && power.digits[0] == 0 {
		return;
	}

	let remainder = half_big_int(power);
	inplace_exp(base, power, result);
	let prod = fast_square(&*result);

	*result = if remainder {
		base * prod
	} else {
		prod
	};
}

impl BigInt {
	pub(crate) fn unsigned_exp(&self, mut power: BigInt) -> BigInt {
		let mut result = BigInt::new(1);
		inplace_exp(self, &mut power, &mut result);
		result
	}
}

impl BigInt {
	pub fn exp(&self, power: &BigInt) -> BigInt {
		BigInt::unsigned_exp(self, power.clone())
	}
}

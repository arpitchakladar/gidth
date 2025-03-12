use crate::number::BigInt;

fn u_exp_small_in(base: &BigInt, power: usize, result: BigInt, current: BigInt) -> (BigInt, BigInt) {
	if power == 0 {
		return (current, result);
	}

	let (power, remainder) = (power / 2, power % 2);
	let (mut current, mut result) = u_exp_small_in(base, power, current, result);
	result.limbs.clear();
	BigInt::sq_in(&current, &mut result);

	if remainder == 0 {
		(result, current)
	} else {
		current.limbs.clear();
		BigInt::u_mul_in(&result, base, &mut current);
		(current, result)
	}
}

impl BigInt {
	pub fn exp(&self, power: usize) -> BigInt {
		let buf_size = (self.limbs.len() + 1) * power;
		let mut buf1 = BigInt::with_capacity(buf_size);
		let mut buf2 = BigInt::with_capacity(buf_size);
		buf1.limbs.push(1);
		buf2.limbs.push(1);
		u_exp_small_in(self, power, buf1, buf2).0
	}
}

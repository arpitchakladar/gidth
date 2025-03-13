use crate::number::BigDecimal;

fn u_pow_small_in(
	base: &BigDecimal,
	power: usize,
	result: BigDecimal,
	current: BigDecimal,
) -> (BigDecimal, BigDecimal) {
	if power == 0 {
		return (current, result);
	}

	let (power, remainder) = (power / 2, power % 2);
	let (mut current, mut result) = u_pow_small_in(
		base,
		power,
		current,
		result,
	);
	result.limbs.clear();
	BigDecimal::sq_in(&current, &mut result);

	if remainder == 0 {
		(result, current)
	} else {
		current.limbs.clear();
		BigDecimal::u_mul_in(&result, base, &mut current);

		(current, result)
	}
}

impl BigDecimal {
	pub fn pow(&self, power: usize) -> BigDecimal {
		let buf_size = (self.limbs.len() + 1) * power;
		let mut buf1 = BigDecimal::with_capacity(buf_size);
		let mut buf2 = BigDecimal::with_capacity(buf_size);
		buf1.limbs.push(1);
		buf2.limbs.push(1);
		let mut result = u_pow_small_in(self, power, buf1, buf2).0;
		result.decimal_pos = self.decimal_pos * power;

		result
	}
}

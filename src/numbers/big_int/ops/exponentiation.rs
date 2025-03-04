use crate::numbers::BigInt;

impl BigInt {
	pub(crate) fn unsigned_exp(&self, power: BigInt) -> BigInt {
		if power.digits.len() == 1 && power.digits[0] == 0 {
			return 1.into();
		}

		let (quotient, remainder) = BigInt::unsigned_divmod_by_small_int(&power, 2u32);
		let exp_half_res = BigInt::unsigned_exp(self, quotient);
		let result = &exp_half_res * &exp_half_res;

		if remainder == 1 {
			self * &result
		} else {
			result
		}
	}
}

impl BigInt {
	pub fn exp(&self, power: &BigInt) -> BigInt {
		BigInt::unsigned_exp(self, power.clone())
	}
}

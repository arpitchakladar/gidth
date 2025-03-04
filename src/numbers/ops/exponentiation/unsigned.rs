use crate::numbers::{
	BigInt,
	unsigned_big_int_divmod_by_small_int,
};

pub(crate) fn unsigned_big_int_exp(base: &BigInt, power: BigInt) -> BigInt {
	if power.digits.len() == 1 && power.digits[0] == 0 {
		return 1.into();
	}

	let (quotient, remainder) = unsigned_big_int_divmod_by_small_int(&power, 2u32);
	let exp_half_res = unsigned_big_int_exp(base, quotient);
	let result = &exp_half_res * &exp_half_res;

	if remainder == 1 {
		base * &result
	} else {
		result
	}
}

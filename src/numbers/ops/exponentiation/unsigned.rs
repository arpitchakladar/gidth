use crate::numbers::{
	BigInt,
	divmod,
};

pub(crate) fn unsigned_big_int_exp(base: &BigInt, power: BigInt) -> BigInt {
	if power == 0.into() {
		return 1.into();
	}

	let two = 2.into();
	let (quotient, remainder) = divmod(&power, &two);

	let x = unsigned_big_int_exp(base, quotient);
	if remainder == 1.into() {
		base * &x * &x
	} else {
		&x * &x
	}
}

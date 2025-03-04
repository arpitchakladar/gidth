use crate::numbers::{
	BigInt,
	unsigned_big_int_exp,
};

pub fn exp(base: &BigInt, power: &BigInt) -> BigInt {
	unsigned_big_int_exp(base, power.clone())
}

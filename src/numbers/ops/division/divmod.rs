use crate::numbers::{
	BigInt,
	unsigned_big_int_divmod,
};

#[inline(always)]
pub fn divmod(lhs: &BigInt, rhs: &BigInt) -> (BigInt, BigInt) {
	unsigned_big_int_divmod(lhs, rhs)
}

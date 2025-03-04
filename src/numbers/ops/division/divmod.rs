use crate::numbers::{
	Integer,
	unsigned_integer_divmod,
};

#[inline(always)]
pub fn divmod(lhs: &Integer, rhs: &Integer) -> (Integer, Integer) {
	unsigned_integer_divmod(lhs, rhs)
}

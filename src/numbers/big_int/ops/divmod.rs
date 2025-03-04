use crate::numbers::BigInt;

impl BigInt {
	#[inline]
	pub fn divmod(&self, rhs: &BigInt) -> (BigInt, BigInt) {
		BigInt::unsigned_divmod(self, rhs)
	}
}

use crate::number::BigInt;

impl BigInt {
	#[inline]
	pub fn divmod(&self, rhs: &BigInt) -> (BigInt, BigInt) {
		BigInt::unsigned_divmod(self, rhs)
	}
}

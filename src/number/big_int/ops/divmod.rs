use crate::number::{
	BigInt,
	DivMod,
};

impl DivMod<BigInt> for BigInt {
	#[inline]
	fn divmod(self, rhs: BigInt) -> (BigInt, BigInt) {
		BigInt::unsigned_divmod(&self, &rhs)
	}
}

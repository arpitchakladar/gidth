use crate::number::{
	BigInt,
	DivMod,
};

impl DivMod<BigInt> for BigInt {
	#[inline]
	fn divmod(self, rhs: BigInt) -> (BigInt, BigInt) {
		BigInt::u_divmod(&self, &rhs)
	}
}

impl DivMod<u32> for BigInt {
	#[inline]
	fn divmod(self, rhs: u32) -> (BigInt, u32) {
		BigInt::u_divmod_base(&self, rhs)
	}
}

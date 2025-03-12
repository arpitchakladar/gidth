use crate::number::{
	BigInt,
	DivMod,
};

impl DivMod<BigInt> for BigInt {
	#[inline]
	fn divmod(self, rhs: BigInt) -> (BigInt, BigInt) {
		let mut quotient = BigInt::with_capacity(
			self.limbs.len()
				.saturating_sub(rhs.limbs.len()) + 1,
		);
		let mut remainder = self.clone();
		BigInt::u_divmod_in(&mut remainder, &rhs, &mut quotient);

		(quotient, remainder)
	}
}

impl DivMod<u32> for BigInt {
	#[inline]
	fn divmod(self, rhs: u32) -> (BigInt, u32) {
		BigInt::u_divmod_base(&self, rhs)
	}
}

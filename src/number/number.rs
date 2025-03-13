use gidth_macros::register_trait;

#[register_trait]
pub(crate) trait Abs {
	fn abs(self) -> Self;
}

#[register_trait]
pub(crate) trait Square {
	fn sq(self) -> Self;
}

#[register_trait]
pub(crate) trait Zero {
	fn zero() -> Self;
	fn is_zero(&self) -> bool;
}

#[register_trait]
pub(crate) trait One {
	fn one() -> Self;
	fn is_one(&self) -> bool;
}

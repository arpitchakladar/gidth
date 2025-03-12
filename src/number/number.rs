pub trait Abs {
	fn abs(self) -> Self;
}

pub trait Square {
	fn sq(self) -> Self;
}

pub trait Zero {
	fn zero() -> Self;
	fn is_zero(&self) -> bool;
}

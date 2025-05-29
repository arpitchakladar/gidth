use std::ops::{
	Add,
	Sub,
	Mul,
	Div,
	AddAssign,
	SubAssign,
	MulAssign,
	DivAssign,
};
use std::cmp::{
	PartialEq,
	PartialOrd,
};
use crate::number::{
	Abs,
	Square,
	Zero,
	One,
};
use gidth_macros::siphon_traits;

#[siphon_traits]
pub trait Real:
	Clone +
	Add<Output = Self> +
	Sub<Output = Self> +
	Mul<Output = Self> +
	Div<Output = Self> +
	for<'a> Add<&'a Self, Output = Self> +
	for<'a> Sub<&'a Self, Output = Self> +
	for<'a> Div<&'a Self, Output = Self> +
	for<'a> Mul<&'a Self, Output = Self> +
	AddAssign<Self> +
	SubAssign<Self> +
	MulAssign<Self> +
	DivAssign<Self> +
	for<'a> AddAssign<&'a Self> +
	for<'a> SubAssign<&'a Self> +
	for<'a> MulAssign<&'a Self> +
	for<'a> DivAssign<&'a Self> +
	PartialEq +
	PartialOrd +
	Zero +
	One +
	Abs +
	Square +
	Sized
{}

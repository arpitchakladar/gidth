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
	std::ops::Add<Output = Self> +
	std::ops::Sub<Output = Self> +
	std::ops::Mul<Output = Self> +
	std::ops::Div<Output = Self> +
	for<'a> std::ops::Add<&'a Self, Output = Self> +
	for<'a> std::ops::Sub<&'a Self, Output = Self> +
	for<'a> std::ops::Div<&'a Self, Output = Self> +
	for<'a> std::ops::Mul<&'a Self, Output = Self> +
	std::cmp::PartialEq +
	std::cmp::PartialOrd +
	Zero +
	One +
	Abs +
	Square +
	Sized
{}

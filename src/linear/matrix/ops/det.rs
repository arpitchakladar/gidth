use crate::{
	linear::Matrix,
	number::Real,
};

// remove + Clone
impl<T: Real + Clone, const D: usize>
	&Matrix<T, D, D>
{
	pub fn det(&self) -> T {
		// implement using LU decomposition
	}
}

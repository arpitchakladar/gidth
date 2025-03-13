#[macro_export]
macro_rules! impl_big_int_binop_variants {
	($trait:ident, $method:ident, $op:tt) => {
		use crate::impl_binop_variants;
		impl_binop_variants!(BigInt, $trait, $method, $op);
	};
}

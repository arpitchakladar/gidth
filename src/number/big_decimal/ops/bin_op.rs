#[macro_export]
macro_rules! impl_big_decimal_binop_variants {
	($trait:ident, $method:ident, $op:tt) => {
		use crate::impl_binop_variants;
		impl_binop_variants!(BigDecimal, $trait, $method, $op);
	};
}

#[macro_export]
macro_rules! impl_big_decimal_binop_assign_variants {
	($trait:ident, $method:ident, $op:tt) => {
		use crate::impl_binop_assign_variants;
		impl_binop_assign_variants!(BigDecimal, $trait, $method, $op);
	};
}

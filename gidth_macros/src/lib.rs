use proc_macro::TokenStream;

mod register_trait;
mod satisfy;
mod siphon_traits;
mod trait_analysis;

#[proc_macro]
pub fn satisfy(input: TokenStream) -> TokenStream {
	satisfy::satisfy(input)
}

#[proc_macro_attribute]
pub fn register_trait(
	attr: TokenStream,
	item: TokenStream,
) -> TokenStream {
	register_trait::register_trait(attr, item)
}

#[proc_macro_attribute]
pub fn siphon_traits(
	attr: TokenStream,
	item: TokenStream,
) -> TokenStream {
	siphon_traits::siphon_traits(attr, item)
}

#[proc_macro_attribute]
pub fn satisfies(
	attr: TokenStream,
	item: TokenStream,
) -> TokenStream {
	satisfy::satisfies(attr, item)
}

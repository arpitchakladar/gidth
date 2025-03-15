use proc_macro::TokenStream;

mod find_modules;
mod place_hidden;
mod register_trait;
mod satisfy;
mod siphon_traits;

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

#[proc_macro]
pub fn place_hidden(item: TokenStream) -> TokenStream {
	place_hidden::place_hidden(item)
}

#[proc_macro]
pub fn satisfy(input: TokenStream) -> TokenStream {
	satisfy::satisfy(input)
}

#[proc_macro_attribute]
pub fn satisfies(attr: TokenStream, item: TokenStream) -> TokenStream {
	satisfy::satisfies(attr, item)
}

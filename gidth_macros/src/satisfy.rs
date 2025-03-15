use proc_macro::{
	TokenStream,
	TokenTree,
};
use syn::{
	parse_macro_input,
	Ident,
	ItemStruct,
	punctuated::Punctuated,
	parse::Parser,
};
use quote::{
	quote,
	format_ident,
};
use proc_macro2::Span;

pub fn satisfy(input: TokenStream) -> TokenStream {
	let mut tokens = input.into_iter();
	let target_type = match tokens.next() {
		Some(TokenTree::Group(group)) => {
			let stream = group.stream();
			parse_macro_input!(stream as Ident)
		},
		_ => panic!("Expected an identifier as the first parameter"),
	};
	match tokens.next() {
		Some(TokenTree::Punct(p)) if p.as_char() == ';' => {},
		_ => panic!("Expected a semicolon `;` after the first parameter"),
	}
	let implementations = tokens
		.filter_map(|token| {
			match token {
				TokenTree::Ident(ident) => {
					let current_trait = Ident::new(
						&ident.to_string(),
						Span::call_site(),
					);
					let satisfy_trait = format_ident!(
						"Satisfy{}",
						&current_trait,
					);

					Some(
						quote! {
							const _: () = {
								let _ = {
									struct _Check<T: #current_trait>(T);
								};
							};
							impl #satisfy_trait for #target_type {}
						}
					)
				},
				// Skip commas
				TokenTree::Punct(p) if p.as_char() == ',' => None,
				_ => panic!("Unexpected token in type list"),
			}
		});

	let expanded = quote! {
		pub use crate::__hidden::*;
		#(#implementations)*
	};

	TokenStream::from(expanded)
}

pub fn satisfies(attr: TokenStream, item: TokenStream) -> TokenStream {
	// Parse the struct
	let input = parse_macro_input!(item as ItemStruct);
	let struct_name = &input.ident;

	// Parse the attribute arguments
	type AttrsType = Punctuated::<syn::Path, syn::Token![,]>;
	let attr_parsed = AttrsType::parse_terminated
		.parse(attr)
		.unwrap();

	let implementations = attr_parsed
		.iter()
		.map(|arg| {
			let target_trait = &arg.segments
				.last()
				.unwrap().ident;
			let satisfy_trait = format_ident!(
				"Satisfy{}",
				target_trait,
			);

			quote! {
				const _: () = {
					let _ = {
						struct _Check<T: #target_trait>(T);
					};
				};
				impl #satisfy_trait for #struct_name {}
			}
		});

	// Generate the struct + implementations
	let expanded = quote! {
		#input
		use crate::__hidden::*;
		#(#implementations)*
	};

	TokenStream::from(expanded)
}

use proc_macro::{
	TokenStream,
	TokenTree,
};
use proc_macro2::{
	Span,
	TokenStream as TokenStream2,
};
use quote::quote;
use syn::{
	parse_macro_input,
	parse2,
	parse_str,
	Ident,
	ItemStruct,
	punctuated::Punctuated,
	parse::Parser,
};

use crate::siphon_traits::DERIVED_TRAIT_REGISTRY;

// Parses the first identifier from the input token stream.
fn parse_target_type(
	tokens: &mut impl Iterator<Item = TokenTree>,
) -> Ident {
	match tokens.next() {
		Some(TokenTree::Group(group)) => {
			parse2::<Ident>(
				Into::<TokenStream2>::into(
					group
						.stream(),
				),
			).expect("Expected an identifier inside the group")
		}
		_ => panic!("Expected an identifier as the first parameter"),
	}
}


// Ensures the next token is a semicolon, otherwise panics.
fn expect_semicolon(
	tokens: &mut impl Iterator<Item = TokenTree>,
) {
	match tokens.next() {
		Some(TokenTree::Punct(p)) if p.as_char() == ';' => {}
		_ => panic!("Expected a semicolon `;` after the first parameter"),
	}
}

// Macro to enforce that a type satisfies a list of traits.
pub fn satisfy(input: TokenStream) -> TokenStream {
	let mut tokens = input.into_iter();
	let target_type = parse_target_type(&mut tokens);
	expect_semicolon(&mut tokens);
	let impls =
		tokens.filter_map(|token| {
			match token {
				TokenTree::Ident(target_trait) => {
					let target_trait =
						Ident::new(
							&target_trait.to_string(),
							Span::call_site(),
						);
					let impls = DERIVED_TRAIT_REGISTRY.lock().unwrap()
						.get(&target_trait.to_string())
						.unwrap()
						.iter()
						.map(|(_, implt)| {
							parse_str::<TokenStream2>(implt)
								.unwrap()
						})
						.collect::<Vec<_>>();

					Some(quote! {
						impl #target_trait for #target_type {
							#(#impls)*
						}
					})
				},
				TokenTree::Punct(p) if p.as_char() == ',' => None, // Ignore commas
				_ => panic!("Unexpected token in type list"),
			}
		})
		.collect::<Vec<_>>();

	let expanded = quote! {
		#(#impls)*
	};

	TokenStream::from(expanded)
}

// Macro to enforce trait satisfaction for a struct using an attribute.
pub fn satisfies(
	attr: TokenStream,
	item: TokenStream,
) -> TokenStream {
	let input = parse_macro_input!(item as ItemStruct);
	let struct_name = &input.ident;

	// Parse the attribute arguments
	type AttrsType = Punctuated<syn::Path, syn::Token![,]>;
	let attr_parsed = AttrsType::parse_terminated
		.parse(attr)
		.unwrap();

	let derived_trait_registry = DERIVED_TRAIT_REGISTRY.lock().unwrap();

	let implementations = attr_parsed
		.iter()
		.map(|arg| {
			let target_trait = &arg.segments.last().unwrap().ident;
			let target_trait_name = target_trait.to_string();
			if let Some(impls) = derived_trait_registry.get(&target_trait_name) {
				let impls = impls
					.iter()
					.map(|(implt, _)| {
						parse_str::<TokenStream2>(implt)
							.unwrap()
					});

				quote! {
					impl #target_trait for #struct_name {
						#(#impls)*
					}
				}
			} else {
				panic!("Invalid trait name.")
			}
		})
		.collect::<Vec<_>>();

	let expanded = quote! {
		#input
		#(#implementations)*
	};

	TokenStream::from(expanded)
}

use proc_macro::{
	TokenStream,
	TokenTree,
};
use proc_macro2::{
	Span,
	TokenStream as TokenStream2,
};
use quote::{
	format_ident,
	quote,
};
use syn::{
	parse_macro_input,
	parse2,
	Ident,
	ItemStruct,
	punctuated::Punctuated,
	parse::Parser,
};

/// Parses the first identifier from the input token stream.
fn parse_target_type(tokens: &mut impl Iterator<Item = TokenTree>) -> Ident {
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


/// Ensures the next token is a semicolon, otherwise panics.
fn expect_semicolon(tokens: &mut impl Iterator<Item = TokenTree>) {
	match tokens.next() {
		Some(TokenTree::Punct(p)) if p.as_char() == ';' => {}
		_ => panic!("Expected a semicolon `;` after the first parameter"),
	}
}

/// Generates trait satisfaction checks for a given type.
fn generate_satisfaction_checks(target_type: &Ident, tokens: impl Iterator<Item = TokenTree>) -> Vec<proc_macro2::TokenStream> {
	tokens.filter_map(|token| {
		match token {
			TokenTree::Ident(ident) => {
				let target_trait = Ident::new(&ident.to_string(), Span::call_site());
				let satisfy_trait = format_ident!("Satisfy{}", target_trait);

				Some(quote! {
					const _: () = {
						let _ = { struct _Check<T: #target_trait>(T); };
					};
					impl #satisfy_trait for #target_type {}
				})
			}
			TokenTree::Punct(p) if p.as_char() == ',' => None, // Ignore commas
			_ => panic!("Unexpected token in type list"),
		}
	}).collect()
}

/// Macro to enforce that a type satisfies a list of traits.
pub fn satisfy(input: TokenStream) -> TokenStream {
	let mut tokens = input.into_iter();
	let target_type = parse_target_type(&mut tokens);
	expect_semicolon(&mut tokens);
	let implementations = generate_satisfaction_checks(&target_type, tokens);

	let expanded = quote! {
		pub use crate::__hidden::*;
		#(#implementations)*
	};

	TokenStream::from(expanded)
}

/// Macro to enforce trait satisfaction for a struct using an attribute.
pub fn satisfies(attr: TokenStream, item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as ItemStruct);
	let struct_name = &input.ident;

	// Parse the attribute arguments
	type AttrsType = Punctuated<syn::Path, syn::Token![,]>;
	let attr_parsed = AttrsType::parse_terminated.parse(attr).unwrap();

	let implementations = attr_parsed.iter().map(|arg| {
		let target_trait = &arg.segments.last().unwrap().ident;
		let satisfy_trait = format_ident!("Satisfy{}", target_trait);

		quote! {
			const _: () = {
				let _ = { struct _Check<T: #target_trait>(T); };
			};
			impl #satisfy_trait for #struct_name {}
		}
	});

	let expanded = quote! {
		#input
		use crate::__hidden::*;
		#(#implementations)*
	};

	TokenStream::from(expanded)
}


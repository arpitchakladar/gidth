use proc_macro::TokenStream;
use quote::{quote, format_ident};

use crate::find_modules::find_modules;
use crate::siphon_traits::DERIVED_TRAIT_REGISTRY;

pub fn place_hidden(_input: TokenStream) -> TokenStream {
	let module_paths = find_modules();

	let trait_registry = DERIVED_TRAIT_REGISTRY.lock().unwrap();

	let generated_traits: Vec<_> = trait_registry
		.iter()
		.map(|trait_name| {
			let trait_ident = format_ident!("{}", trait_name);
			let satisfy_trait_ident = format_ident!("Satisfy{}", trait_ident);
			quote! {
				pub trait #satisfy_trait_ident {}
			}
		})
		.collect();

	let expanded = quote! {
		pub mod __hidden {
			#(use #module_paths::*;)*
			#(#generated_traits)*
		}
	};

	TokenStream::from(expanded)
}

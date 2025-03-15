use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use syn::{
	parse_macro_input,
	ItemTrait,
};
use quote::{
	quote,
	format_ident,
};

use crate::trait_analysis::{
	extract_base_traits,
	extract_method_signatures,
};

lazy_static! {
	pub(crate) static ref DERIVED_TRAIT_REGISTRY: Mutex<HashMap<String, (Vec<String>, Vec<String>, String)>> = Mutex::new(HashMap::new());
}

// TODO: Make it work with traits with generics and lifetimes
pub fn siphon_traits(
	_attr: TokenStream,
	trait_item: TokenStream,
) -> TokenStream {
	let parsed_trait = parse_macro_input!(trait_item as ItemTrait);
	let trait_ident = &parsed_trait.ident;
	let trait_name = trait_ident.to_string();

	let base_traits = extract_base_traits(&parsed_trait);
	let method_signatures = extract_method_signatures(&parsed_trait);

	let satisfy_trait = format_ident!("Satisfy{}", trait_name);
	let supertraits = &parsed_trait.supertraits;

	// Store extracted traits and methods in the registry
	if let Ok(mut registry) = DERIVED_TRAIT_REGISTRY.lock() {
		registry.insert(
			trait_name.clone(),
			(
				base_traits,
				method_signatures,
				quote! { #supertraits }.to_string(),
			),
		);
	}

	let expanded = quote! {
		pub use crate::__hidden::{ #satisfy_trait, #trait_ident };
	};

	TokenStream::from(expanded)
}

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use syn::{
	parse_macro_input,
	ItemTrait,
};
use quote::quote;

use crate::trait_analysis::extract_method_signatures;

lazy_static! {
	pub(crate) static ref METHOD_REGISTRY: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

// TODO: Make it work with traits with generics and lifetimes
pub(crate) fn register_trait(
	_attr: TokenStream,
	trait_item: TokenStream,
) -> TokenStream {
	let parsed_trait = parse_macro_input!(trait_item as ItemTrait);
	let trait_name = parsed_trait.ident.to_string();

	// Extract method signatures
	let method_signatures: Vec<String> =
		extract_method_signatures(
			&parsed_trait,
		);

	// Store method signatures in the registry
	if let Ok(mut registry) = METHOD_REGISTRY.lock() {
		registry.insert(trait_name, method_signatures);
	}

	TokenStream::from(
		quote! {
			#parsed_trait
		},
	)
}

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro_error::abort_call_site;
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
	let mut method_registry = METHOD_REGISTRY
		.lock()
		.unwrap_or_else(|e| {
			abort_call_site!(
				"Failed to lock access METHOD_REGISTRY: {}",
				e,
			)
		});
	method_registry
		.insert(
			trait_name.clone(),
			method_signatures,
		)
		// if the a trait with same name already exists
		// then abort
		.map(|_| {
			abort_call_site!(
				"Duplicate traits with name \"{}\" already exists in METHOD_REGISTRY.",
				&trait_name,
			)
		});

	TokenStream::from(
		quote! {
			#parsed_trait
		},
	)
}

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro_error::abort_call_site;
use syn::{
	parse_macro_input,
	ItemTrait,
	TraitItemFn,
	FnArg,
	parse_str,
};
use quote::{
	quote,
	format_ident,
};

use crate::trait_analysis::{
	extract_base_traits,
	extract_full_where_clause,
	extract_method_signatures,
};

use crate::register_trait::METHOD_REGISTRY;

lazy_static! {
	pub(crate) static ref DERIVED_TRAIT_REGISTRY: Mutex<HashMap<String, Vec<(String, String)>>> = Mutex::new(HashMap::new());
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
	let where_clause = extract_full_where_clause(&parsed_trait);
	let current_trait_methods = extract_method_signatures(&parsed_trait);
	let method_registry = METHOD_REGISTRY
		.lock()
		.unwrap_or_else(|e| {
			abort_call_site!(
				"Failed to lock access METHOD_REGISTRY: {}",
				e,
			)
		});

	let (
		trait_method_signatures,
		method_implementations,
	): (Vec<_>, Vec<_>) = base_traits
		.iter()
		.filter_map(|base_trait| {
			match method_registry.get(base_trait) {
				Some(method_registry) => Some((base_trait, method_registry)),
				None => None
			}
		})
		.map(|(trait_name, method_signatures)| {
			let trait_ident = format_ident!("{}", &trait_name);
			method_signatures
				.iter()
				.filter_map(move |method_signature| {
					match parse_str::<TraitItemFn>(method_signature) {
						Ok(parsed_method) => {
							let method_name = &parsed_method.sig.ident;
							let method_params = &parsed_method.sig.inputs;
							let return_type = &parsed_method.sig.output;

							// Extract parameter names for delegation
							let parameter_names: Vec<_> = method_params
								.iter()
								.map(|param| match param {
									FnArg::Receiver(_) => quote! { self },
									FnArg::Typed(pattern) => {
										let param_name = &pattern.pat;
										quote! { #param_name }
									}
								})
								.collect();

							// Generate method delegation dynamically
							let method_implementation_trait = quote! {
								#[inline(always)]
								fn #method_name(#method_params) #return_type {
									#trait_ident::#method_name(#(#parameter_names),*)
								}
							}.to_string();
							let method_implementation_self = quote! {
								#[inline(always)]
								fn #method_name(#method_params) #return_type {
									Self::#method_name(#(#parameter_names),*)
								}
							}.to_string();

							// Add the method signature to the trait
							let trait_method_signature = quote! {
								fn #method_name(#method_params) #return_type;
							};

							Some(
								(
									trait_method_signature,
									(
										method_implementation_self,
										method_implementation_trait,
									),
								)
							)
						},
						Err(_) => None,
					}
				})
		})
		.flatten()
		.unzip();

	let trait_defined_method_signature = current_trait_methods
		.iter()
		.map(|method_signature| {
			parse_str::<TraitItemFn>(method_signature)
				.unwrap_or_else(|e| {
					abort_call_site!(
						"Failed to parse method signature: {}",
						e,
					)
				})
		})
		.collect::<Vec<_>>();

	DERIVED_TRAIT_REGISTRY.lock()
		.unwrap_or_else(|e| {
			abort_call_site!(
				"Failed to lock access DERIVED_TRAIT_REGISTRY: {}",
				e,
			)
		})
		.insert(
			trait_name.to_string(),
			method_implementations.clone(),
		);

	let supertraits = &parsed_trait.supertraits;

	let expanded = quote! {
		pub trait #trait_ident: #supertraits #where_clause {
			#(#trait_method_signatures)*
			#(#trait_defined_method_signature)*
		}
	};

	TokenStream::from(expanded)
}

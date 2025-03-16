use std::sync::Mutex;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
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
	extract_method_signatures,
};

use crate::register_trait::METHOD_REGISTRY;

lazy_static! {
	pub(crate) static ref DERIVED_TRAIT_REGISTRY: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

// TODO: Make it work with traits with generics and lifetimes
pub fn siphon_traits(
	_attr: TokenStream,
	trait_item: TokenStream,
) -> TokenStream {
	let parsed_trait = parse_macro_input!(trait_item as ItemTrait);
	let trait_ident = &parsed_trait.ident;
	let trait_name = trait_ident.to_string();
	if let Ok(mut derived_trait_registry) = DERIVED_TRAIT_REGISTRY.lock() {
		derived_trait_registry.push(trait_name.clone());
	}
	let base_traits = extract_base_traits(&parsed_trait);
	let current_trait_methods = extract_method_signatures(&parsed_trait);
	let method_registry = METHOD_REGISTRY.lock().unwrap();

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
							let method_implementation = quote! {
								#[inline(always)]
								fn #method_name(#method_params) #return_type {
									#trait_ident::#method_name(#(#parameter_names),*)
								}
							};

							// Add the method signature to the trait
							let trait_method_signature = quote! {
								fn #method_name(#method_params) #return_type;
							};

							Some((trait_method_signature, method_implementation))
						},
						Err(_) => None,
					}
				})
		})
		.flatten()
		.unzip();

	let trait_defined_method_signature = current_trait_methods
		.iter()
		.filter_map(|method_signature| {
			match parse_str::<TraitItemFn>(method_signature) {
				Ok(method_signature) => Some(method_signature),
				Err(_) => None,
			}
		})
		.collect::<Vec<_>>();

	let satisfy_trait = format_ident!("Satisfy{}", trait_name);
	let supertraits = &parsed_trait.supertraits;

	let expanded = quote! {
		use crate::__hidden::#satisfy_trait;
		pub trait #trait_ident: #supertraits {
			#(#trait_method_signatures)*
			#(#trait_defined_method_signature)*
		}

		impl<T: #satisfy_trait + #supertraits> #trait_ident for T {
			#(#method_implementations)*
		}
	};

	TokenStream::from(expanded)
}

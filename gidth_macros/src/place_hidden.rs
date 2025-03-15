use std::str::FromStr;
use proc_macro::TokenStream;
use quote::{
	quote,
	format_ident,
};
use syn::{
	FnArg,
	parse_str,
	TraitItemFn,
};

use crate::find_modules::find_modules;
use crate::siphon_traits::DERIVED_TRAIT_REGISTRY;
use crate::register_trait::METHOD_REGISTRY;

pub fn place_hidden(_item: TokenStream) -> TokenStream {
	let modules = find_modules();

	let derived_trait_registry = DERIVED_TRAIT_REGISTRY
		.lock()
		.unwrap();
	let method_registry = METHOD_REGISTRY.lock().unwrap();
	let trait_definitions: Vec<_> = derived_trait_registry
		.iter()
		.map(|(trait_name, (supertraits, method_signatures, x))| {
			let trait_name = format_ident!("{}", trait_name);
			let satisfy_trait_ident = format_ident!("Satisfy{}", &trait_name);
			let mut method_impls = Vec::new();
			let mut x_method_signatures = method_signatures
				.iter()
				.map(|method_signature| {
					let parsed_str = parse_str::<TraitItemFn>(&method_signature)
						.expect("Failed to parse method signature");
					quote! { #parsed_str }
				})
				.collect::<Vec<_>>();
			for trait_ident in supertraits.iter() {
				let method_signatures = method_registry
					.get(trait_ident)
					.cloned()
					.unwrap_or_default();
				let trait_ident = format_ident!("{}", trait_ident);

				for method_signature in method_signatures.iter() {
					let parsed_fn: TraitItemFn = parse_str(method_signature)
						.expect("Failed to parse method signature");

					let method_name = &parsed_fn.sig.ident;
					let inputs = &parsed_fn.sig.inputs;
					let output = &parsed_fn.sig.output;

					// Extract parameter names
					let param_names: Vec<_> = inputs
						.iter()
						.map(|arg| {
							match arg {
								// Handle `self` properly
								FnArg::Receiver(_) => quote!{ self },
								FnArg::Typed(pat_type) => {
									let pat = &pat_type.pat;
									quote! { #pat }
								}
							}
						}).collect();

					// TODO: Avoid reimplmenting conflicting methods
					// Generate method delegation dynamically
					method_impls.push(quote! {
						#[inline(always)]
						fn #method_name(#inputs) #output {
							#trait_ident::#method_name(#(#param_names),*)
						}
					});

					x_method_signatures.push(
						quote! {
							fn #method_name(#inputs) #output;
						}
					);
				}
			}

			let supertraits = proc_macro2::TokenStream::from_str(x).unwrap();

			quote! {
				pub trait #satisfy_trait_ident {}
				pub trait #trait_name: #supertraits {
					#(#x_method_signatures)*
				}
				impl<T: #satisfy_trait_ident + #supertraits> #trait_name for T {
					#(#method_impls)*
				}
			}
		})
		.collect();

	let expanded = quote! {
		pub mod __hidden {
			#(use #modules::*;)*
			#(#trait_definitions)*
		}
	};

	TokenStream::from(expanded)
}

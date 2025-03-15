use std::str::FromStr;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{FnArg, parse_str, TraitItemFn};

use crate::find_modules::find_modules;
use crate::siphon_traits::DERIVED_TRAIT_REGISTRY;
use crate::register_trait::METHOD_REGISTRY;

pub fn place_hidden(_input: TokenStream) -> TokenStream {
	let module_paths = find_modules();

	let trait_registry = DERIVED_TRAIT_REGISTRY.lock().unwrap();
	let method_registry = METHOD_REGISTRY.lock().unwrap();

	let generated_traits: Vec<_> = trait_registry
		.iter()
		.map(|(trait_name, (supertraits, method_signatures, supertraits_tokens))| {
			let trait_ident = format_ident!("{}", trait_name);
			let satisfy_trait_ident = format_ident!("Satisfy{}", trait_ident);
			let mut method_implementations = Vec::new();
			let mut trait_method_signatures: Vec<_> = method_signatures
				.iter()
				.filter_map(|signature| parse_str::<TraitItemFn>(signature).ok())
				.map(|parsed_signature| quote! { #parsed_signature })
				.collect();

			for supertrait_name in supertraits.iter() {
				let inherited_method_signatures = method_registry
					.get(supertrait_name)
					.cloned()
					.unwrap_or_default();
				let supertrait_ident = format_ident!("{}", supertrait_name);

				for method_signature in inherited_method_signatures.iter() {
					if let Ok(parsed_method) = parse_str::<TraitItemFn>(method_signature) {
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
						method_implementations.push(quote! {
							#[inline(always)]
							fn #method_name(#method_params) #return_type {
								#supertrait_ident::#method_name(#(#parameter_names),*)
							}
						});

						// Add the method signature to the trait
						trait_method_signatures.push(
							quote! {
								fn #method_name(#method_params) #return_type;
							},
						);
					}
				}
			}

			let supertraits_tokens =
				proc_macro2::TokenStream::from_str(
					supertraits_tokens,
				).unwrap();

			quote! {
				pub trait #satisfy_trait_ident {}
				pub trait #trait_ident: #supertraits_tokens {
					#(#trait_method_signatures)*
				}
				impl<T: #satisfy_trait_ident + #supertraits_tokens> #trait_ident for T {
					#(#method_implementations)*
				}
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

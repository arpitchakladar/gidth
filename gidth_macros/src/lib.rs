
use proc_macro::TokenStream;
use quote::{
	quote,
	ToTokens,
};
use syn::{parse_macro_input, parse_str, ItemTrait, TraitBound, TraitItem, ReturnType, FnArg, TypeParamBound, TraitItemFn};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
	static ref METHOD_REGISTRY: Mutex<std::collections::HashMap<String, Vec<String>>> = 
		Mutex::new(std::collections::HashMap::new());
}

#[proc_macro_attribute]
pub fn register_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as ItemTrait);
	let trait_name = input.ident.to_string();

	// Extract full method signatures
	let method_signatures: Vec<String> = input.items.iter().filter_map(|item| {
		if let TraitItem::Fn(TraitItemFn { sig, .. }) = item {
			let method_name = sig.ident.to_string();

			// Extract parameter types
			let params: Vec<String> = sig.inputs.iter().map(|arg| {
				match arg {
					FnArg::Receiver(rec) => {
						let has_ref = if rec.reference.is_none() {
							""
						} else {
							"&"
						};
						let if_mut = if rec.mutability.is_none() {
							""
						} else {
							"mut"
						};
						format!("{}{} self", has_ref, if_mut)
					},
					FnArg::Typed(pat_type) => format!("{}", quote! { #pat_type }),
				}
			}).collect();

			// Extract return type
			let return_type = match &sig.output {
				ReturnType::Default => "-> ()".to_string(),
				ReturnType::Type(_, ty) => format!("-> {}", quote! { #ty }),
			};

			Some(format!("fn {}({}) {};", method_name, params.join(", "), return_type))
		} else {
			None
		}
	}).collect();

	// Store methods in the registry
	METHOD_REGISTRY.lock().unwrap().insert(trait_name.clone(), method_signatures.clone());

	let expanded = quote! {
		#input
	};

	TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn siphon_traits(_attr: TokenStream, item: TokenStream) -> TokenStream {
	// Parse the trait definition
	let mut input = parse_macro_input!(item as ItemTrait);
	let trait_name = &input.ident;

	let mut method_impls = Vec::new();

	let method_registry = METHOD_REGISTRY.lock().unwrap();

	// Iterate over supertraits
	for supertrait in &input.supertraits {
		if let TypeParamBound::Trait(TraitBound { path, .. }) = supertrait {
			let method_signatures = method_registry.get(&path.into_token_stream().to_string()).cloned().unwrap_or_default();

			for method_signature in method_signatures.into_iter() {
				let parsed_fn: TraitItemFn = parse_str(&method_signature)
					.expect("Failed to parse method signature");
				input.items.push(syn::TraitItem::Fn(parsed_fn.clone()));

				let method_name = &parsed_fn.sig.ident;
				let inputs = &parsed_fn.sig.inputs;
				let output = &parsed_fn.sig.output;

				// Extract parameter names
				let param_names: Vec<_> = inputs.iter().map(|arg| {
					match arg {
						FnArg::Receiver(_) => quote!{ self }, // Handle `self` properly
						FnArg::Typed(pat_type) => {
							let pat = &pat_type.pat;
							quote! { #pat }
						}
					}
				}).collect();

				// Generate method delegation dynamically
				method_impls.push(quote! {
					#[inline(always)]
					fn #method_name(#inputs) #output {
						#path::#method_name(#(#param_names),*)
					}
				});
			}
		}
	}

	let supertraits = &input.supertraits;

	let expanded = quote! {
		#input
		impl<T: #supertraits> #trait_name for T {
			#(#method_impls)*
		}
	};

	TokenStream::from(expanded)
}


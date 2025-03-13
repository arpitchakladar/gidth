
use proc_macro::TokenStream;
use quote::{
	quote,
	format_ident,
};
use syn::{parse_macro_input, ItemTrait, TraitBound, TypeParamBound, TraitItem, TraitItemFn};

#[proc_macro_attribute]
pub fn siphon_traits(_attr: TokenStream, item: TokenStream) -> TokenStream {
	// Parse the trait definition
	let input = parse_macro_input!(item as ItemTrait);
	let trait_name = &input.ident;

	let mut method_impls = Vec::new();

	// Iterate over all inherited traits

	// Iterate over items in the supertrait
	for item in &input.items {
		if let TraitItem::Fn(TraitItemFn { sig, .. }) = item {
			let method_name = &sig.ident;
			let inputs = &sig.inputs;
			let output = &sig.output;

			let param_names: Vec<_> = inputs.iter().filter_map(|arg| {
				if let syn::FnArg::Typed(pat_type) = arg {
					if let syn::Pat::Ident(pat_ident) = *pat_type.pat.clone() {
						Some(pat_ident.ident)
					} else {
						None
					}
				} else {
					None
				}
			}).collect();

			// Generate method delegation dynamically
			method_impls.push(quote! {
				#[inline(always)]
				fn #method_name(#inputs) #output {
					#trait_name::#method_name(#(#param_names),*)
				}
			});
		}
	}

	let temp_trait = format_ident!("Impl{}", &trait_name);
	let supertraits = &input.supertraits;

	let expanded = quote! {
		use crate::number::*;
		use std::ops::*;
		pub trait #temp_trait: #supertraits {}
		#input

		impl<T: #temp_trait> #trait_name for T {
			#(#method_impls)*
		}
	};

	println!("{}", quote! { #expanded });

	TokenStream::from(expanded)
}


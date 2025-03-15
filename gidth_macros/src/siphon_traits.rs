use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use syn::{
	parse_macro_input,
	TypeParamBound,
	TraitBound,
	TraitItem,
	TraitItemFn,
	ItemTrait,
	FnArg,
	ReturnType,
};
use quote::{
	quote,
	format_ident,
};

use crate::register_trait::METHOD_REGISTRY;

lazy_static! {
	pub(crate) static ref DERIVED_TRAIT_REGISTRY: Mutex<HashMap<String, (Vec<String>, Vec<String>, String)>> =
		Mutex::new(HashMap::new());
}

pub fn siphon_traits(
	_attr: TokenStream,
	item: TokenStream,
) -> TokenStream {
	// Parse the trait definition
	let input = parse_macro_input!(item as ItemTrait);
	let trait_name = &input.ident;

	let method_registry = METHOD_REGISTRY.lock().unwrap();
	let mut derived_trait_registry = DERIVED_TRAIT_REGISTRY.lock().unwrap();
	let mut base_traits = Vec::new();

	// Iterate over supertraits
	for supertrait in &input.supertraits {
		if let TypeParamBound::Trait(TraitBound { path, .. }) = supertrait {
			let trait_ident = path.segments
				.last()
				.unwrap()
				.ident.to_string();

			let method_signatures = method_registry
				.get(&trait_ident);
			if let Some(_) = &method_signatures {
				base_traits.push(trait_ident.clone());
			}
		}
	}
	let method_signatures: Vec<String> = input.items
		.iter()
		.filter_map(|item| {
			if let TraitItem::Fn(TraitItemFn { sig, .. }) = item {
				let method_name = sig.ident.to_string();

				// Extract parameter types
				let params: Vec<String> = sig.inputs
					.iter()
					.map(|arg| {
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
							FnArg::Typed(pat_type) => {
								format!("{}", quote! { #pat_type })
							},
						}
					}).collect();

				// Extract return type
				let return_type = match &sig.output {
					ReturnType::Default => "-> ()".to_string(),
					ReturnType::Type(_, ty) => format!("-> {}", quote! { #ty }),
				};

				Some(
					format!(
						"fn {}({}) {};",
						method_name,
						params.join(", "),
						return_type,
					),
				)
			} else {
				None
			}
		}).collect();

	let satisfy_trait = format_ident!("Satisfy{}", &trait_name);
	let supertraits = &input.supertraits;
	derived_trait_registry.insert(
		trait_name.to_string(),
		(
			base_traits,
			method_signatures,
			(quote! { #supertraits })
				.to_string(),
		),
	);

	let expanded = quote! {
		pub use crate::__hidden::{ #satisfy_trait, #trait_name };
	};

	TokenStream::from(expanded)
}

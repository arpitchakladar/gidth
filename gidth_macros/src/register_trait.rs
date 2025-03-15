use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use syn::{
	parse_macro_input,
	TraitItem,
	TraitItemFn,
	FnArg,
	ReturnType,
	ItemTrait,
};
use quote::quote;

lazy_static! {
	pub(crate) static ref METHOD_REGISTRY: Mutex<HashMap<String, Vec<String>>> =
		Mutex::new(HashMap::new());
}

pub(crate) fn register_trait(
	_attr: TokenStream,
	item: TokenStream,
) -> TokenStream {
	let input_item = item.clone();
	let input = parse_macro_input!(input_item as ItemTrait);
	let trait_name = input.ident.to_string();

	// Extract full method signatures
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

	// Store methods in the registry
	METHOD_REGISTRY
		.lock()
		.unwrap()
		.insert(trait_name.clone(), method_signatures.clone());

	item
}

use proc_macro_error::{
	abort_call_site,
	abort,
};
use syn::{
	FnArg,
	ReturnType,
	TraitItem,
	ItemTrait,
	TraitBound,
	TypeParamBound,
};
use quote::quote;

use crate::register_trait::METHOD_REGISTRY;

// TODO: Make it work with traits with generics and lifetimes

// Formats function parameters into strings
fn format_fn_arg(arg: &FnArg) -> String {
	match arg {
		FnArg::Receiver(receiver) => {
			let reference_prefix = {
				if receiver.reference.is_some() {
					"&"
				} else {
					""
				}
			};
			let mutability = {
				if receiver.mutability.is_some() {
					"mut "
				} else {
					""
				}
			};

			format!(
				"{}{}self",
				reference_prefix,
				mutability,
			)
		}
		FnArg::Typed(pat_type) => format!("{}", quote! { #pat_type }),
	}
}

// Formats return type into a string
fn format_return_type(return_type: &ReturnType) -> String {
	match return_type {
		ReturnType::Default => "-> ()".to_string(),
		ReturnType::Type(_, ty) => format!("-> {}", quote! { #ty }),
	}
}

// Extracts base trait names from the given trait definition.
pub(crate) fn extract_base_traits(
	trait_def: &ItemTrait,
) -> Vec<String> {
	let method_registry = METHOD_REGISTRY
		.lock()
		.unwrap_or_else(|e| {
			abort_call_site!(
				"Failed to lock access METHOD_REGISTRY: {}",
				e,
			)
		});
	trait_def.supertraits
		.iter()
		.filter_map(|supertrait| {
			if let TypeParamBound::Trait(TraitBound { path, .. }) = supertrait {
				let trait_ident = path.segments
					.last()
					.unwrap_or_else(|| {
						abort!(
							supertrait,
							"Failed to parse trait name of \"{}\".",
							quote! { #supertrait }.to_string(),
						)
					})
					.ident.to_string();
				let method_signatures = method_registry
					.get(&trait_ident);
				if method_signatures.is_some() {
					return Some(trait_ident.clone());
				}
			}

			None
		})
		.collect::<Vec<_>>()
}

// Extracts method signatures from the given trait definition.
pub(crate) fn extract_method_signatures(
	trait_def: &ItemTrait,
) -> Vec<String> {
	trait_def
		.items
		.iter()
		.filter_map(|item| match item {
			TraitItem::Fn(method) => {
				let method_name = method.sig.ident.to_string();
	
				// Extract parameters
				let parameter_list: Vec<String> = method.sig.inputs
					.iter()
					.map(format_fn_arg)
					.collect();
				let return_type =
					format_return_type(
						&method.sig.output,
					);

				Some(
					format!(
						"fn {}({}) {};",
						method_name,
						parameter_list.join(", "),
						return_type,
					),
				)
			},
			_ => None,
		})
		.collect()
}

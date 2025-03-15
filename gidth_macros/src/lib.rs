use std::collections::HashMap;
use std::str::FromStr;
use proc_macro::{
	TokenStream,
	TokenTree,
};
use proc_macro2::Span;
use quote::{
	quote,
	format_ident,
};
use syn::{
	parse_macro_input,
	parse_str,
	parse::Parser,
	punctuated::Punctuated,
	Ident,
	ItemTrait,
	TraitBound,
	TraitItem,
	ReturnType,
	FnArg,
	TypeParamBound,
	TraitItemFn,
	ItemStruct,
};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
	static ref METHOD_REGISTRY: Mutex<HashMap<String, Vec<String>>> =
		Mutex::new(HashMap::new());

	static ref DERIVED_TRAIT_REGISTRY: Mutex<HashMap<String, (Vec<String>, Vec<String>, String)>> =
		Mutex::new(HashMap::new());
}

#[proc_macro_attribute]
pub fn register_trait(
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

#[proc_macro_attribute]
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

#[proc_macro]
pub fn place_hidden(_item: TokenStream) -> TokenStream {
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
				use crate::number::*;
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
			#(#trait_definitions)*
		}
	};

	TokenStream::from(expanded)
}

#[proc_macro]
pub fn satisfy(input: TokenStream) -> TokenStream {
	let mut tokens = input.into_iter();
	let target_type = match tokens.next() {
		Some(TokenTree::Group(group)) => {
			let stream = group.stream();
			parse_macro_input!(stream as Ident)
		},
		_ => panic!("Expected an identifier as the first parameter"),
	};
	match tokens.next() {
		Some(TokenTree::Punct(p)) if p.as_char() == ';' => {},
		_ => panic!("Expected a semicolon `;` after the first parameter"),
	}
	let implementations = tokens
		.filter_map(|token| {
			match token {
				TokenTree::Ident(ident) => {
					let current_trait = Ident::new(
						&ident.to_string(),
						Span::call_site(),
					);
					let satisfy_trait = format_ident!(
						"Satisfy{}",
						&current_trait,
					);

					Some(
						quote! {
							const _: () = {
								let _ = {
									struct _Check<T: #current_trait>(T);
								};
							};
							impl #satisfy_trait for #target_type {}
						}
					)
				},
				// Skip commas
				TokenTree::Punct(p) if p.as_char() == ',' => None,
				_ => panic!("Unexpected token in type list"),
			}
		});

	let expanded = quote! {
		pub use crate::__hidden::*;
		#(#implementations)*
	};

	TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn satisfies(attr: TokenStream, item: TokenStream) -> TokenStream {
	// Parse the struct
	let input = parse_macro_input!(item as ItemStruct);
	let struct_name = &input.ident;

	// Parse the attribute arguments
	type AttrsType = Punctuated::<syn::Path, syn::Token![,]>;
	let attr_parsed = AttrsType::parse_terminated
		.parse(attr)
		.unwrap();

	let implementations = attr_parsed
		.iter()
		.map(|arg| {
			let target_trait = &arg.segments
				.last()
				.unwrap().ident;
			let satisfy_trait = format_ident!(
				"Satisfy{}",
				target_trait,
			);

			quote! {
				const _: () = {
					let _ = {
						struct _Check<T: #target_trait>(T);
					};
				};
				impl #satisfy_trait for #struct_name {}
			}
		});

	// Generate the struct + implementations
	let expanded = quote! {
		#input
		use crate::__hidden::*;
		#(#implementations)*
	};

	TokenStream::from(expanded)
}

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use regex::Regex;
use cargo_metadata::MetadataCommand;

// Retrieves the root path of the crate using `cargo_metadata`.
fn get_crate_root() -> PathBuf {
	MetadataCommand::new()
		.exec()
		.map(|metadata| PathBuf::from(metadata.workspace_root))
		// Fallback to "src" if metadata fails
		.unwrap_or_else(|_| PathBuf::from("src"))
}

// Checks if a module is public in the given file.
fn is_public_module(file_path: &Path, regex: &Regex) -> bool {
	fs::read_to_string(file_path)
		.map(|content| regex.is_match(&content))
		.unwrap_or(false)
}

// Extracts public module names from a file using regex.
fn parse_public_modules_from_file(
	file_path: &Path,
	regex: &Regex,
) -> Vec<String> {
	fs::read_to_string(file_path)
		.map(|content| {
			content
				.lines()
				.filter_map(|line| regex.captures(line))
				.map(|cap| cap[1].to_string())
				.collect()
		})
		.unwrap_or_default()
}

// Recursively finds public modules in the given directory.
fn find_modules_rec(
	path: &Path,
	prefix: &str,
	modules: &mut Vec<String>,
	regex: &Regex,
) {
	if let Ok(entries) = fs::read_dir(path) {
		for entry in entries.flatten() {
			process_directory_entry(
				&entry.path(),
				prefix,
				modules,
				regex,
			);
		}
	}
}

// Processes a directory entry and determines if it's a public module.
fn process_directory_entry(
	path: &Path,
	prefix: &str,
	modules: &mut Vec<String>,
	regex: &Regex,
) {
	if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
		if path.is_dir() {
			let mod_rs_path = path.join("mod.rs");
			if mod_rs_path.exists() && is_public_module(&mod_rs_path, regex) {
				let mod_path = format!("{prefix}::{file_stem}");
				modules.push(mod_path.clone());
				find_modules_rec(&path, &mod_path, modules, regex);
			}
		} else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") &&
			file_stem != "mod"
		{
			let parent_mod_rs = path.parent().map(|p| p.join("mod.rs"));
			if parent_mod_rs.as_ref().map(|p| !p.exists()).unwrap_or(true) ||
				parent_mod_rs.as_ref()
					.map_or(false, |p| is_public_module(p, regex))
			{
				modules.push(format!("{prefix}::{file_stem}"));
			}
		}
	}
}

// Finds public modules in the crate root (not just "src/lib.rs").
pub(crate) fn find_modules() -> Vec<proc_macro2::TokenStream> {
	let crate_root = get_crate_root();
	// Assume "src/lib.rs" as the main entry point
	let lib_rs_path = crate_root.join("src/lib.rs");
	let mut modules = Vec::new();

	let regex =
		Regex::new(
			r"(?m)^\s*pub(?:\s*\(crate\))?\s*mod\s+(\w+);",
		).unwrap();

	if lib_rs_path.exists() {
		let public_modules =
			parse_public_modules_from_file(
				&lib_rs_path,
				&regex,
			);
		for module_name in public_modules {
			let module_path = format!("crate::{module_name}");
			modules.push(module_path.clone());
			find_modules_rec(
				&crate_root.join("src")
					.join(&module_name),
				&module_path,
				&mut modules,
				&regex,
			);
		}
	}

	modules
		.iter()
		.filter_map(|module| {
			proc_macro2::TokenStream::from_str(module).ok()
		})
		.collect()
}

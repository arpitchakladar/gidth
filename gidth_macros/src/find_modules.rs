use std::str::FromStr;
use std::fs;
use std::path::Path;

fn is_public_module(file_path: &Path, module_name: &str) -> bool {
	if let Ok(content) = fs::read_to_string(file_path) {
		let pub_mod_decl = format!("pub mod {}", module_name);
		let pub_crate_mod_decl = format!("pub(crate) mod {}", module_name);
		content.contains(&pub_mod_decl) || content.contains(&pub_crate_mod_decl)
	} else {
		false
	}
}

/// Recursively finds modules, checking if they are public
fn find_modules_rec(path: &Path, prefix: &str, modules: &mut Vec<String>) {
	if let Ok(entries) = fs::read_dir(path) {
		for entry in entries.flatten() {
			let path = entry.path();
			let file_name = path.file_stem().unwrap().to_string_lossy().to_string();

			if path.is_dir() {
				let mod_rs_path = path.join("mod.rs");
				if mod_rs_path.exists() && is_public_module(&mod_rs_path, &file_name) {
					let mod_path = format!("{}::{}", prefix, file_name);
					modules.push(mod_path.clone());
					find_modules_rec(&path, &mod_path, modules);
				}
			} else if path.extension().map(|e| e == "rs").unwrap_or(false) {
				if file_name != "mod" {
					let parent_mod_rs = path.parent().unwrap().join("mod.rs");
					if !parent_mod_rs.exists() || is_public_module(&parent_mod_rs, &file_name) {
						modules.push(format!("{}::{}", prefix, file_name));
					}
				}
			}
		}
	}
}

pub(crate) fn find_modules() -> Vec<proc_macro2::TokenStream> {
	let mut modules = Vec::new();
	let lib_rs_path = Path::new("src/lib.rs");
	if lib_rs_path.exists() {
		if let Ok(content) = fs::read_to_string(lib_rs_path) {
			for line in content.lines() {
				if let Some(start) = line.find("pub mod ") {
					let end = line[start + 8..].find(';');
					if let Some(end) = end {
						let module_name = &line[start + 8..start + 8 + end];
						let module_path = format!("crate::{}", module_name);
						modules.push(module_path.clone());
						find_modules_rec(&Path::new("src").join(module_name), &module_path, &mut modules);
					}
				}
			}
		}
	}

	modules
		.iter()
		.map(|x| proc_macro2::TokenStream::from_str(x).unwrap())
		.collect::<Vec<_>>()
}

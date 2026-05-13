use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn main() {
    activate_git_hooks();

    let crate_name = env::var("CARGO_PKG_NAME").unwrap();
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=src/");

    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3) // target/<PROFILE>
        .expect("Failed to find target dir");

    let include_dir = target_dir.join("include").join(&crate_name);
    fs::create_dir_all(&include_dir).unwrap();

    let config_path = Path::new(&crate_dir).join("cbindgen.toml");
    let base_config =
        cbindgen::Config::from_file(&config_path).expect("Failed to read cbindgen.toml");

    // Scan src/std_impls/ for modules; each <name>.rs containing
    // `export_extern_type!(<Ty>)` becomes a dedicated header.
    let modules = scan_std_impls(Path::new(&crate_dir).join("src/std_impls"));

    let all_std_items: Vec<&str> = modules
        .iter()
        .flat_map(|m| m.items.iter().map(String::as_str))
        .collect();

    // ezffi.h — core (excludes everything emitted by std_impls).
    build_header(
        &crate_dir,
        &base_config,
        &all_std_items,
        None,
        &include_dir.join(format!("{}.h", crate_name)),
    );

    // One header per std_impls module — includes ezffi.h for the core.
    for module in &modules {
        let exclude: Vec<&str> = all_std_items
            .iter()
            .copied()
            .filter(|item| !module.items.iter().any(|i| i == item))
            .collect();

        build_header(
            &crate_dir,
            &base_config,
            &exclude,
            Some(&format!("#include \"{}.h\"", crate_name)),
            &include_dir.join(format!("{}.h", module.name)),
        );
    }
}

struct StdImplModule {
    name: String,
    items: Vec<String>,
}

fn scan_std_impls(dir: PathBuf) -> Vec<StdImplModule> {
    let Ok(entries) = fs::read_dir(&dir) else {
        return Vec::new();
    };

    let mut modules = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if stem == "mod" || stem.is_empty() {
            continue;
        }

        let Ok(src) = fs::read_to_string(&path) else {
            continue;
        };

        let Some(ty) = find_extern_type(&src) else {
            continue;
        };

        // Derive item names from naming convention; keep in sync with
        // crates/ezffi-macros/src/namer.rs + crates/ezffi/ezffi.toml.
        let ffi_struct = format!("EzFfi{ty}");
        let free_fn = to_snake_case(&format!("{ffi_struct}_free"));

        modules.push(StdImplModule {
            name: stem.to_string(),
            items: vec![ffi_struct, free_fn],
        });
    }

    modules
}

fn find_extern_type(src: &str) -> Option<String> {
    const MARKER: &str = "export_extern_type!(";
    let start = src.find(MARKER)? + MARKER.len();
    let rest = &src[start..];
    let end = rest.find(')')?;
    let inner = rest[..end].trim();
    // Strip generic params: `Vec<T>` -> `Vec`.
    let ident = inner.split('<').next()?.trim();
    if ident.is_empty() {
        None
    } else {
        Some(ident.to_string())
    }
}

fn to_snake_case(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 4);
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn build_header(
    crate_dir: &str,
    base_config: &cbindgen::Config,
    exclude_items: &[&str],
    after_include: Option<&str>,
    output_path: &Path,
) {
    let profile = match env::var("PROFILE").as_deref() {
        Ok("release") => cbindgen::Profile::Release,
        _ => cbindgen::Profile::Debug,
    };

    let mut builder = cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(base_config.clone())
        .with_parse_expand_profile(profile);

    // Forward this build's enabled features to cargo expand so cbindgen sees
    // the same items the compiled lib does (e.g. `generics` gates `Vec<T>`).
    let mut features: Vec<String> = Vec::new();
    if env::var_os("CARGO_FEATURE_GENERICS").is_some() {
        features.push("generics".into());
    }
    if !features.is_empty() {
        builder = builder.with_parse_expand_features(&features);
    }

    for item in exclude_items {
        builder = builder.exclude_item(*item);
    }

    if let Some(line) = after_include {
        builder = builder.with_after_include(format!("\n{line}"));
    }

    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(output_path);
}

fn activate_git_hooks() {
    let Some(manifest_dir) = env::var("CARGO_MANIFEST_DIR").ok() else {
        return;
    };
    let mut dir = PathBuf::from(manifest_dir);
    let workspace_root = loop {
        if dir.join(".git").exists() {
            break dir;
        }
        if !dir.pop() {
            return;
        }
    };

    let hooks_dir = workspace_root.join(".githooks");
    if !hooks_dir.is_dir() {
        return;
    }

    let _ = Command::new("git")
        .args(["config", "core.hooksPath", ".githooks"])
        .current_dir(&workspace_root)
        .status()
        .ok();
}

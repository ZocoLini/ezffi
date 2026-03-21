use std::{env, fs, path::Path, process::Command};

fn main() {
    let crate_name = env::var("CARGO_PKG_NAME").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .expect("Failed to find target dir");

    let include_dir = target_dir.join("include").join(&crate_name);

    // Crear directorios
    fs::create_dir_all(&include_dir).unwrap();

    let header_path = include_dir.join("test.h");

    let status = Command::new("cbindgen")
        .arg("-c")
        .arg("cbindgen.toml")
        .arg("--output")
        .arg(header_path)
        .status()
        .expect("failed to run cbindgen");

    if !status.success() {
        panic!("cbindgen failed");
    }
}

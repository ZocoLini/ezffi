use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=force-rebuild");

    let status = Command::new("cbindgen")
        .arg("-c")
        .arg("cbindgen.toml")
        .arg("--output")
        .arg("include/test.h")
        .status()
        .expect("failed to run cbindgen");

    if !status.success() {
        panic!("cbindgen failed");
    }
}

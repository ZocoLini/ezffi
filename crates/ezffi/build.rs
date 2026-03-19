use std::process::Command;

fn main() {
    let status = Command::new("cbindgen")
        .arg("-c")
        .arg("cbindgen.toml")
        .arg("--output")
        .arg("include/ezffi.h")
        .status()
        .expect("failed to run cbindgen");

    if !status.success() {
        panic!("cbindgen failed");
    }
}

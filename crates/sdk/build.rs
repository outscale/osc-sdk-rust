use std::env;
use std::path::PathBuf;
use std::process::Command;

fn run_generator(target: &str) {
    if env::var_os(format!("CARGO_FEATURE_{}", target.to_uppercase())).is_none() {
        return;
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(&out_dir).join(format!("{}.rs", target));
    let config_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join(format!("../../config/{}/cfg.yaml", target));

    println!("cargo:rerun-if-changed={}", config_path.to_string_lossy());

    osc_sdk_rust_build::run_or_panic(&[
        "--config",
        config_path.to_str().unwrap(),
        "--output",
        dest_path.to_str().unwrap(),
    ]);

    Command::new("rustfmt")
        .args(["--edition", "2024", dest_path.to_str().unwrap()])
        .output()
        .unwrap();
}

fn main() {
    println!(
        "cargo::rerun-if-changed={}",
        osc_sdk_rust_build::binary_path().to_string_lossy()
    );

    run_generator("osc");
    run_generator("oks");
}

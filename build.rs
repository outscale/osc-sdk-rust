use std::env;
use std::path::Path;
use std::process::Command;

fn run_generator(target: &str) {
    if env::var_os(format!("CARGO_FEATURE_{}", target.to_uppercase())).is_none() {
        return;
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(format!("{}.rs", target));

    Command::new("go")
        .args([
            "run",
            "./hacks/generator/",
            "--config",
            &format!("./config/{}/cfg.yaml", target),
            "--output",
            dest_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    Command::new("rustfmt")
        .args(["--edition", "2024", dest_path.to_str().unwrap()])
        .output()
        .unwrap();
}

fn main() {
    println!("cargo::rerun-if-changed=hacks/generator");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=config");

    run_generator("osc");
    run_generator("oks");
}

use std::path::Path;
use std::process::Command;

const BIN_PATH: &str = env!("GENERATOR_BIN_PATH");

pub fn binary_path() -> &'static Path {
    Path::new(BIN_PATH)
}

pub fn run_or_panic(args: &[&str]) {
    let output = Command::new(BIN_PATH)
        .args(args)
        .output()
        .expect("Failed to execute generator");
    if !output.status.success() {
        panic!(
            "Generator failed (exit {}):\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

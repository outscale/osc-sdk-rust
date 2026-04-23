use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const GENERATOR_NAME: &str = "generator";
const GENERATOR_VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_REPO: &str = "outscale/osc-sdk-rust";
const GO_SOURCE_DIR: &str = "../../hacks/generator";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={GO_SOURCE_DIR}");
    println!("cargo:rerun-if-env-changed=GENERATOR_BIN");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let binary_name = binary_file_name();
    let final_bin = out_dir.join(&binary_name);

    // 0. User-provided generator binary path
    if let Ok(generator_bin) = env::var("GENERATOR_BIN") {
        let p = PathBuf::from(&generator_bin);
        if p.exists() {
            eprintln!("[build.rs] Using user-provided binary: {}", p.display());
            emit_env(&p);
            return;
        } else {
            eprintln!(
                "[build.rs] user-provided binary {} does not exist. falling back.",
                p.display()
            );
        }
    }

    // 1. Try downloading a pre-built binary from GitHub Releases
    if try_download_from_github(&final_bin) {
        eprintln!("[build.rs] Downloaded pre-built binary from GitHub.");
        emit_env(&final_bin);
        return;
    }

    // 2. Build the generator from source
    if try_build_from_source(&final_bin) {
        eprintln!("[build.rs] Built generator from source.");
        emit_env(&final_bin);
        return;
    }

    // 3. Nothing worked
    panic!(
        "\n\n\
        ============================================================\n\
        ERROR: Could not obtain the `{GENERATOR_NAME}` binary.\n\n\
        Tried (in order):\n\
          1. $GENERATOR_BIN environment variable\n\
          2. GitHub release download ({repo} {ver})\n\
          3. Building from Go source in ./{src}\n\n\
        Options:\n\
          • Install Go (https://go.dev/dl/) so we can build it.\n\
          • Manually download the binary and set GENERATOR_BIN=/path/to/it\n\
          • Check your internet connection for the GitHub download.\n\
        ============================================================\n\n",
        repo = GITHUB_REPO,
        ver = GENERATOR_VERSION,
        src = GO_SOURCE_DIR,
    );
}

fn emit_env(bin_path: &Path) {
    println!("cargo:rustc-env=GENERATOR_BIN_PATH={}", bin_path.display());
    eprintln!("[build.rs] Generator ready: {}", bin_path.display());
}

fn go_arch() -> &'static str {
    match () {
        _ if cfg!(target_arch = "aarch64") => "arm64",
        _ if cfg!(target_arch = "x86_64") => "amd64",
        _ => panic!("unsupported target architecture for Go generator"),
    }
}

fn go_os() -> &'static str {
    match () {
        _ if cfg!(target_os = "linux") => "linux",
        _ if cfg!(target_os = "macos") => "darwin",
        _ if cfg!(target_os = "windows") => "windows",
        _ => panic!("unsupported target OS for Go generator"),
    }
}

fn binary_file_name() -> String {
    let ext = if go_os() == "windows" { ".exe" } else { "" };
    format!(
        "{GENERATOR_NAME}-{GENERATOR_VERSION}-{}-{}{ext}",
        go_os(),
        go_arch()
    )
}

fn try_download_from_github(dest: &Path) -> bool {
    let file = binary_file_name();
    let url =
        format!("https://github.com/{GITHUB_REPO}/releases/download/{GENERATOR_VERSION}/{file}");

    eprintln!("[build.rs] Attempting download: {url}");

    if try_download_curl(&url, dest) {
        return true;
    }

    // Try wget as fallback
    if try_download_wget(&url, dest) {
        return true;
    }

    eprintln!("[build.rs] All download methods failed.");
    false
}

fn try_download_curl(url: &str, dest: &Path) -> bool {
    Command::new("curl")
        .args([
            "--fail",
            "--silent",
            "--show-error",
            "--location",
            "--output",
            &dest.to_string_lossy(),
            url,
        ])
        .status()
        .map(|s| {
            if s.success() {
                true
            } else {
                // Clean up partial file
                let _ = fs::remove_file(dest);
                false
            }
        })
        .unwrap_or(false)
}

fn try_download_wget(url: &str, dest: &Path) -> bool {
    Command::new("wget")
        .args(["--quiet", "-O", &dest.to_string_lossy(), url])
        .status()
        .map(|s| {
            if s.success() {
                true
            } else {
                let _ = fs::remove_file(dest);
                false
            }
        })
        .unwrap_or(false)
}

fn try_build_from_source(dest: &Path) -> bool {
    let go_version = Command::new("go").arg("version").output();
    match go_version {
        Ok(output) if output.status.success() => {
            eprintln!(
                "[build.rs] Found Go: {}",
                String::from_utf8_lossy(&output.stdout).trim()
            );
        }
        _ => {
            eprintln!("[build.rs] `go` not found on PATH, skipping source build.");
            return false;
        }
    }

    let source_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(GO_SOURCE_DIR);
    if !source_dir.join("go.mod").exists() {
        eprintln!(
            "[build.rs] No go.mod found in {}, skipping source build.",
            source_dir.display()
        );
        return false;
    }

    eprintln!(
        "[build.rs] Building Go source in {} → {}",
        source_dir.display(),
        dest.display()
    );

    let status = Command::new("go")
        .arg("build")
        .arg("-trimpath")
        .args(["-ldflags", "-s -w"])
        .arg("-o")
        .arg(dest)
        .arg(".")
        .current_dir(&source_dir)
        .env("CGO_ENABLED", "0")
        .env("GOOS", go_os())
        .env("GOARCH", go_arch())
        .status();

    match status {
        Ok(s) if s.success() => true,
        Ok(s) => {
            eprintln!("[build.rs] `go build` exited with status: {s}");
            let _ = fs::remove_file(dest);
            false
        }
        Err(e) => {
            eprintln!("[build.rs] `go build` failed to execute: {e}");
            false
        }
    }
}

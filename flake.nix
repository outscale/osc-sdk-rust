{
  description = "Outscale SDK for Rust — offline-capable build";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};

        # Builds the code generator from hacks/generator/ so it can be
        # injected into the Rust build via GENERATOR_BIN. This eliminates
        # the need for network access (GitHub releases, Go module proxy)
        # during the Rust crate build phase.
        #
        # First build will fail with a hash mismatch. Copy the expected
        # hash from the error message into `vendorHash` and rebuild.
        generator = pkgs.buildGoModule {
          pname = "generator";
          version = "2.0.0-alpha.0";
          src = ./hacks/generator;

          vendorHash = "sha256-B1C3u6iQNMqIuJ3rUQiXI9XOGMPIL5gItmCMuhEfC9Q=";

          # The go.mod declares `module gogo`, so buildGoModule produces
          # $out/bin/gogo. Rename to the name sdk-build expects.
          postInstall = ''
            if [ -f "$out/bin/gogo" ]; then
              mv "$out/bin/gogo" "$out/bin/generator"
            fi
          '';

          env.CGO_ENABLED = "0";
          ldflags = ["-s" "-w"];
        };

        # Builds the SDK crate and its examples. GENERATOR_BIN points
        # sdk-build at the Nix-built generator — both the GitHub download
        # and the `go build` fallback are skipped entirely.
        #
        # The cargoLock pre-fetches every Rust crate dependency into the
        # Nix store. The build phase itself runs with no network access.
        #
        # Examples at the workspace root import `crates/sdk`; Cargo only
        # discovers examples under a package's own examples/ directory,
        # so we symlink them there in postPatch.
        examples = pkgs.rustPlatform.buildRustPackage {
          pname = "osc-sdk-rust-examples";
          version = "2.0.0-alpha.0";
          src = ./.;
          cargoLock = {lockFile = ./Cargo.lock;};

          nativeBuildInputs = [pkgs.rustfmt];

          GENERATOR_BIN = "${generator}/bin/generator";

          cargoBuildFlags = "--examples";

          installPhase = ''
            mkdir -p $out/bin
            bins=$(find /build \
                -type f \
                -executable | grep "release/examples")

            echo "$bins"
            for bin in $bins; do
                cp "$bin" "$out/bin/$(basename "$bin")"
            done
          '';

          doCheck = false;
        };
      in {
        packages = {
          inherit generator examples;
          default = examples;
        };

        devShells.default = pkgs.mkShellNoCC {
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            go
            golangci-lint
          ];

          GENERATOR_BIN = "${generator}/bin/generator";

          shellHook = ''
            echo "osc-sdk-rust development shell"
            echo "  GENERATOR_BIN = $GENERATOR_BIN"
            echo
            echo "Usage:"
            echo "  cargo run --example vm       # OSC example"
            echo "  cargo run --example project  # OKS example"
          '';
        };
      }
    );
}

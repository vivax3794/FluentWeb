{
  description = "FluentWeb";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
            buildInputs = [
                ( rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
                    extensions = ["rust-src" "llvm-tools-preview"];
                    targets = ["wasm32-unknown-unknown"];
                }) )
                rust-analyzer
                mold

                # TESTS
                cargo-nextest

                # WASM TESTS
                wasm-pack
                chromium
                firefox
                chromedriver

                # COVERAGE
                cargo-llvm-cov
                glibc_multi
            ];
        };
    }
    );
}

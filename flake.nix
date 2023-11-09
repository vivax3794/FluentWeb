{
  description = "FluentWeb";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [];
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            clippy
          ];
        };

        packages.default = pkgs.stdenv.mkDerivation {
          pname = "fluent_web";
          version = "0.0.1";
          src =  ./.;

          buildInputs = with pkgs; [rustc cargo];
          doCheck = true;
          checkPhase = "bash test.sh";
          buildPhase = "";
          installPhase = "";
        };

        defaultPackage = self.packages.${system}.default;

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/test";
        };
      }
    );
}

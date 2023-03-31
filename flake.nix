{
  description = "faerber - a tool to match your pictures to color palettes";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
    pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    pre-commit-hooks,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = let
          inherit (pkgs.stdenv.hostPlatform) isDarwin;
        in
          pkgs.mkShell {
            buildInputs = with pkgs;
              [
                (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
                libiconv
                openssl
                pkg-config
                # WASM dependencies
                binaryen
                wasm-bindgen-cli
                wasm-pack
              ]
              ++ lib.optionals isDarwin (with darwin.apple_sdk.frameworks; [
                # GUI dependencies on darwin
                AppKit
                CoreServices
                OpenGL
                Security
              ]);
            shellHook = ''
              ${self.checks.${system}.pre-commit-check.shellHook}
            '';
          };

        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              clippy.enable = true;
              rustfmt.enable = true;
            };
          };
        };

        packages = rec {
          faerber = pkgs.rustPlatform.buildRustPackage {
            name = "faerber";
            src = ./.;
            cargoSha256 = "sha256-o57JqWVO9G32J3AP0HhgJfDpOWLtjJF17Wh6xUcB6TE=";
          };
          default = faerber;
        };
      }
    );
}

{
  description = "Nix dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.nightly.latest.default;
        basePackages = with pkgs; [ pkg-config openssl ];
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "nix-deployer";
          src = ./..;
          cargoLock = { lockFile = ../Cargo.lock; };
          buildInputs = basePackages;
        };
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = with pkgs;
              [ rust-analyzer ] ++ [ rust ] ++ basePackages;
            shellHook = ''
              echo "
              🐚 Rust dev shell ready!
              Run: cargo build / cargo test / etc."
            '';
          };
        };
      });
}

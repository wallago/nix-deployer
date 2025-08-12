{
  description = "Nix dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    project-banner.url = "github:wallago/project-banner?dir=nix";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, project-banner, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.nightly.latest.default;
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "nix-deployer";
          src = ./..;
          cargoLock = { lockFile = ../Cargo.lock; };
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = with pkgs; [ openssl rust-analyzer ] ++ [ rust ];
            shellHook = ''
              ${project-banner.packages.${system}.default}/bin/project-banner \
                --owner "wallago" \
                --logo " 󰖌 " \
                --product "nix-deployer" \
                --part "CLI" \
                --code "WL25-NIXD-CL01" \
            '';
          };
        };
      });
}

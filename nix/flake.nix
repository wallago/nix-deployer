{
  description = "Nixos deployer tool";

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
        commonNativeBuildInputs = with pkgs; [ pkg-config ];
        commonBuildInputs = with pkgs; [ openssl ];
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "nix-deployer";
          src = ./..;
          cargoLock = { lockFile = ../Cargo.lock; };
          nativeBuildInputs = commonNativeBuildInputs;
          buildInputs = commonBuildInputs;
        };
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = commonNativeBuildInputs;
            buildInputs = with pkgs;
              [ rust-analyzer ] ++ [ rust ] ++ commonBuildInputs;
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

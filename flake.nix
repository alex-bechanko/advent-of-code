{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
 
  };

  description = "Advent of Code solutions written in Rust";

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk' = pkgs.callPackage naersk { };
        rust-toolchain = pkgs.symlinkJoin {
          name = "rust-toolchain";
          paths = [ pkgs.rustc pkgs.cargo pkgs.cargo-watch pkgs.rust-analyzer pkgs.rustPlatform.rustcSrc pkgs.clippy ];
        };
      in {
        defaultPackage = naersk'.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [ rust-toolchain ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
  });
}

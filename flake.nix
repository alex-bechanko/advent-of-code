{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
    }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      rust-toolchain = fenix.packages.x86_64-linux.stable.toolchain;
    in
    {

      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = [
          rust-toolchain
        ];
      };
    };
}

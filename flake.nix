{
  description = "Solutions to the Advent of Code puzzles.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {system = system;};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [pkgs.rustc pkgs.cargo pkgs.rustfmt pkgs.clippy];
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
    formatter.${system} = pkgs.alejandra;
  };
}

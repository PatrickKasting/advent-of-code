{
  description = "Solutions to the Advent of Code puzzles.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {system = system;};
  in rec {
    devShells.${system}.default = pkgs.mkShell {
      packages = let
        nix = [formatter.${system}];
        rust = with pkgs; [rustc cargo rustfmt clippy rust-analyzer];
        llvm = with pkgs; [clang libclang];
      in
        nix ++ rust ++ llvm;
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
    formatter.${system} = pkgs.alejandra;
  };
}

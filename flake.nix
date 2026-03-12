{
  description = "Solutions to the Advent of Code puzzles.";

  inputs = {
    nixpkgs2505.url = "github:nixos/nixpkgs/nixos-25.05";
    nixpkgs2511.url = "github:nixos/nixpkgs/nixos-25.11";
  };

  outputs = {
    self,
    nixpkgs2505,
    nixpkgs2511,
  }: let
    system = "x86_64-linux";
    pkgsOptions = {
      inherit system;
      config = {
        allowUnfree = true;
        cudaSupport = true;
      };
    };
    pkgs2505 = import nixpkgs2505 pkgsOptions;
    pkgs2511 = import nixpkgs2511 pkgsOptions;
  in rec {
    devShells.${system}.default = pkgs2511.mkShell {
      packages = let
        nix = [pkgs2511.nil formatter.${system}];
        rust = with pkgs2511; [rustc cargo rustfmt clippy rust-analyzer];
        llvm = with pkgs2511; [clang libclang];
        python = [(pkgs2511.python3.withPackages (pp: [pp.tensorflowWithCuda]))];
        build = [pkgs2511.pkg-config pkgs2511.openssl] ++ [pkgs2505.bazel_6];
      in
        nix ++ rust ++ llvm ++ python ++ build;
      RUST_SRC_PATH = "${pkgs2511.rust.packages.stable.rustPlatform.rustLibSrc}";
      LIBCLANG_PATH = "${pkgs2511.libclang.lib}/lib";
    };
    formatter.${system} = pkgs2511.alejandra;
  };
}

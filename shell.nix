{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.clang
    pkgs.pkg-config
    pkgs.openssl
  ];
}


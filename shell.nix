{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
        git
        rustup
        openssl
        pkg-config
    ];
    shellHook = ''
      alias run="rustup default stable && cargo run"
    '';
}
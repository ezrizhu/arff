{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc-wasm32
    cargo
    rustfmt
    rust-analyzer
    clippy
    nodejs_20
    nodePackages_latest.wrangler
    pkg-config
    openssl
    lld
    cargo-mommy
  ];

  shellHook = ''
    export PATH=/home/ezri/.cargo/bin:$PATH
    export CARGO_MOMMYS_MOODS="chill/ominous/thirsty/yikes"
  '';

  RUST_BACKTRACE = 1;
}

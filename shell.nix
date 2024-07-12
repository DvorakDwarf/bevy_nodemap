{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ 
      rustc 
      cargo 
      gcc 
      rustfmt 
      clippy 

      pkg-config

      alsa-lib
      systemd

      # xorg.libX11 
      # xorg.libXcursor 
      # xorg.libXi 
      # libxkbcommon 
  ];

  #Found I needed this as well
  shellHook = 
  ''
    export LD_LIBRARY_PATH=$NIX_LD_LIBRARY_PATH
  '';

  # Certain Rust tools won't work without this
  # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}

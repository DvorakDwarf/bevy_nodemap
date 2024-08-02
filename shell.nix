{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  # nativeBuildInputs = with pkgs; [ 
  #     pkg-config

  #     systemd
  #     udev
  #     alsa-lib
  #     vulkan-loader
  #     xorg.libX11
  #     xorg.libXcursor
  #     xorg.libXi
  #     xorg.libXrandr
  #     libxkbcommon
  #     cmake
  # ];

  nativeBuildInputs = with pkgs; [
    rustc 
    cargo 
    gcc 
    rustfmt 
    clippy 
    
    pkg-config
  ];
  buildInputs = with pkgs; [
    udev alsa-lib vulkan-loader

    # To use the x11 feature
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr 
    # To use the wayland feature
    libxkbcommon wayland 

    clang
  ];

  #Found I needed this as well
  shellHook = 
  ''
    export LD_LIBRARY_PATH=$NIX_LD_LIBRARY_PATH
    export RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  '';

  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  # Certain Rust tools won't work without this
  # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension  
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
  # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}

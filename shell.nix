{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell rec {
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

    linuxPackages_latest.perf
  ];

  # #Found I needed this as well
  shellHook = 
  ''
    LD_LIBRARY_PATH=$NIX_LD_LIBRARY_PATH;
    export RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  '';

  # LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  # LD_LIBRARY_PATH="${stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH";
  RUST_SRC_PATH = pkgs.rust.packages.stable.rustPlatform.rustLibSrc;
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

  # Certain Rust tools won't work without this
  # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension  
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
  # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}

# Nix shell script for building on Nix environments. 
# This assumes that you have a Rust toolchain installed.
# https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md#nix

# { pkgs ? import <nixpkgs> { } }:

# with pkgs;

# mkShell rec {
#   nativeBuildInputs = [
#     rustc 
#     cargo 
#     gcc 
#     rustfmt 
#     clippy 
#     cargo-flamegraph

#     pkg-config
#   ];
#   buildInputs = [
#     udev alsa-lib vulkan-loader
#     xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
#     libxkbcommon wayland # To use the wayland feature
#     linuxPackages_latest.perf #flamegraph
#   ];
#   LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
# }
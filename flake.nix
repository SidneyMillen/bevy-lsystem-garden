{
  inputs = {
    # This must be the stable nixpkgs if you're running the app on a
    # stable NixOS install.  Mixing EGL library versions doesn't work.
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        libPath = with pkgs;
          lib.makeLibraryPath [
            libGL
            libGLU
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr # To use the x11 feature
            libxkbcommon
            wayland # To use the wayland feature
          ];
        overlays = [ rust-overlay.overlay ];
        rust =
          pkgs.rust-bin.stable.latest.default; # Stable rust, default profile. If not sure, always choose this.;
      in {
        devShell = with pkgs;
          mkShell {
            buildInputs = [
              zlib
              cargo
              cargo-insta
              pkg-config
              pre-commit
              rust-analyzer
              rustc
              rustPackages.clippy
              rustfmt
              tokei
              systemd
              alsa-lib
              udev
              vulkan-loader
              pkg-config
              lld
              clang
              wayland
              xorg.libxcb
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            LD_LIBRARY_PATH = libPath;
            GIT_EXTERNAL_DIFF = "${difftastic}/bin/difft";

          };

      });
}

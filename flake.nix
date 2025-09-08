{
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";

    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    naersk,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };

        toolchain = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
            toolchain.default.override {
              extensions = [
                "rust-src"
                "rustc-codegen-cranelift-preview"
              ];
            }
        );
        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

        nativeBuildInputs = [
          pkgs.pkg-config
        ];

        buildInputs = [
          pkgs.clang
          pkgs.mold
          pkgs.udev
          pkgs.alsa-lib
          pkgs.vulkan-loader
          # x11
          pkgs.xorg.libX11
          pkgs.xorg.libXcursor
          pkgs.xorg.libXi
          pkgs.xorg.libXrandr
          pkgs.libxkbcommon
          # Wayland
          pkgs.wayland
          toolchain
        ];

        all_deps =
          [
            pkgs.rust-analyzer
          ]
          ++ buildInputs ++ nativeBuildInputs;
      in {
        packages.default = naersk'.buildPackage {
          pname = "siege-week-2";
          src = ./.;

          inherit buildInputs nativeBuildInputs;
        };

        devShells.default = pkgs.mkShell {
          # Additional dev-shell environment variables can be set directly
          # RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";

          nativeBuildInputs = all_deps;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}

{
  description = "Devenv for working with the project, as well as building the project";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:

  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in {
      devShells.default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          # General
          (rust-bin.stable.latest.default.override { extensions = ["rust-src"]; })
          libxkbcommon
          libGL

          # Audio
          alsa-lib

          # X11
          xorg.libX11
          xorg.libXcursor
          xorg.libXi

          # Wayland
          wayland
        ];
        LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
      };
      packages.default = pkgs.callPackage ./default.nix {};
    }
  );
}

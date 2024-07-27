{
  description = "Shell to add the wayland lib for building the project";

  inputs.nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          libxkbcommon
          libGL
          wayland
        ];
        LD_LIBRARY_PATH = "${pkgs.libxkbcommon}/lib:${pkgs.libGL}/lib:${pkgs.wayland}/lib";
      };
    };
}

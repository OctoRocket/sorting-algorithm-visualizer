{ pkgs, rustPlatform, ...}:

rustPlatform.buildRustPackage rec {
    pname = "sorting-algorithm-visualizer";
    version = "0.1.0";

    nativeBuildInputs = with pkgs; [ makeWrapper pkg-config ];
    buildInputs = with pkgs; [
        # General
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

    src = ./.;

    cargoLock.lockFile = ./Cargo.lock;

    postInstall = ''
        wrapProgram $out/bin/${pname} --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs};
    '';
}

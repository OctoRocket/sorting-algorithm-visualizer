{ pkgs, rustPlatform, ...}:

rustPlatform.buildRustPackage rec {
    pname = "sorting-algorithm-visualizer";
    version = "0.1.0";

    nativeBuildInputs = with pkgs; [ makeWrapper ];
    buildInputs = with pkgs; [
        libxkbcommon
        libGL
        wayland
    ];

    src = ./.;

    cargoLock.lockFile = ./Cargo.lock;

    postInstall = ''
        wrapProgram $out/bin/${pname} --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs};
    '';
}
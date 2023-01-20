{
  description = "GitHub dashboard";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      with pkgs; {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with xorg; [
            pkg-config
            python3
            cmake
            cargo
            rustc
            rustfmt
            clippy
          ];

          buildInputs = with xorg; [
            libxcb
            libXcursor
            libXrandr
            libXi
            libGL
            libGLU
            xorg.libX11
            wayland
            libxkbcommon
            fontconfig
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=/run/opengl-driver/lib/:${lib.makeLibraryPath ([libGL libGLU])}
          '';
        };
      }
    );
}


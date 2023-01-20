{
  description = "GitHub dashboard";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        linuxNativeBuildInputs = with pkgs; with xorg; [
          pkg-config
          python3
          cmake
          cargo
          rustc
          rustfmt
          clippy
        ];

        linuxBuildInputs = with pkgs; with xorg; [
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

        darwinNativeBuildInputs = [
        ];

        darwinBuildInputs = with pkgs; [
          libiconv
          darwin.apple_sdk.frameworks.OpenGL
          darwin.apple_sdk.frameworks.CoreServices
          darwin.apple_sdk.frameworks.AppKit
        ];
      in
      with pkgs; {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = if pkgs.stdenv.isDarwin then darwinNativeBuildInputs else linuxNativeBuildInputs;

          buildInputs = if pkgs.stdenv.isDarwin then darwinBuildInputs else linuxBuildInputs;

          shellHook = ''
            export LD_LIBRARY_PATH=/run/opengl-driver/lib/:${lib.makeLibraryPath ([libGL libGLU])}
          '';

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      }
    );
}


{
  description = "Flake utils demo";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        linux-build-inputs = [
          pkgs.webkitgtk
        ];

        macos-build-inputs = [
          pkgs.libiconv
        ] ++ (with pkgs.darwin.apple_sdk.frameworks;
          [
            Carbon
            WebKit
            AppKit
          ]);

        build-inputs =
          if pkgs.stdenv.isDarwin then macos-build-inputs else linux-build-inputs;
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.yarn
          ];
          buildInputs = build-inputs;

          # TODO: temporary workaround
          # https://github.com/NixOS/nixpkgs/issues/32580#issuecomment-350877197
          WEBKIT_DISABLE_COMPOSITING_MODE = 1;
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
        packages.default = pkgs.callPackage ./default.nix { };
      }
    );
}

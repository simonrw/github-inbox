{
  description = "Flake utils demo";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          node-overlay = self: super: {
            nodejs = super.nodejs-16_x;
          };

          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              node-overlay
            ];
          };

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
              pkgs.nodePackages.prettier
            ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
              pkgs.appimage-run
            ];

            buildInputs = build-inputs;

            # TODO: temporary workaround
            # https://github.com/NixOS/nixpkgs/issues/32580#issuecomment-350877197
            WEBKIT_DISABLE_COMPOSITING_MODE = 1;
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
          packages.github-inbox = import ./default.nix { inherit pkgs; };
          packages.default = self.packages.${system}.github-inbox;
          inherit pkgs;
        }
      );
}

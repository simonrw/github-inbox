{
  description = "Flake utils demo";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
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
          packages.default = pkgs.callPackage ./default.nix { };
        }
      ) // {
      # System-specific things
      packages.x86_64-linux.github-inbox-bin =
        let
          pkgs = nixpkgs.legacyPackages.x86_64-linux;

          desktopItem = pkgs.makeDesktopItem {
            categories = [ "Game" "AdventureGame" ];
            desktopName = "Github Inbox";
            exec = "github-inbox";
            name = "github-inbox";
          };

        in
        pkgs.stdenv.mkDerivation rec {
          pname = "github-inbox-bin";
          version = "0.0.0";

          src = pkgs.fetchurl {
            url = "https://github.com/simonrw/github-inbox/releases/download/app-v${version}/github-inbox_${version}_amd64.deb";
            hash = "sha256-FBYqjLkG+yBCm9wdLc7XpYqnyL27zvsf6ofJO/MwbMQ=";
          };

          nativeBuildInputs = [
            pkgs.autoPatchelfHook
            pkgs.dpkg
          ];

          buildInputs = [
            pkgs.glib-networking
            pkgs.openssl
            pkgs.webkitgtk
            pkgs.wrapGAppsHook
          ];

          unpackCmd = "dpkg-deb -x $curSrc source";

          installPhase =
            if pkgs.stdenv.isLinux then ''
              mv usr $out;
            '' else ''
              '';
        };
    };
}

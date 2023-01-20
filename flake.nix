{
  description = "Flake utils demo";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.webkitgtk
          ];
          buildInputs = [
          ];

          # TODO: temporary workaround
          # https://github.com/NixOS/nixpkgs/issues/32580#issuecomment-350877197
          WEBKIT_DISABLE_COMPOSITING_MODE = 1;
        };
      }
    );
}

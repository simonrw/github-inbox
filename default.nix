{ pkgs ? import <nixpkgs> { } }:
let
  pname = "github-inbox";
  version = "unstable";

  frontend-build = pkgs.mkYarnPackage {
    inherit version;

    src = ./.;
    pname = "github-inbox-ui";

    offlineCache = pkgs.fetchYarnDeps {
      yarnLock = ./yarn.lock;
      hash = "sha256-PAMl4/TReurrvrg/xuBaBM1oqmmXiRCqCI7qtOJS7+8=";
    };

    packageJSON = ./package.json;

    buildPhase = ''
      export HOME=$(mktemp -d)
      ${pkgs.yarn}/bin/yarn --offline build
      cp -r deps/github-inbox/dist $out
    '';

    distPhase = ":";
    dontInstall = true;
  };

  linux-build-inputs = [
    pkgs.dbus
    pkgs.openssl
    pkgs.freetype
    pkgs.libsoup
    pkgs.gtk3
    pkgs.webkitgtk
    pkgs.cmake
  ];

  macos-build-inputs = with pkgs.darwin.apple_sdk.frameworks; [
    Carbon
    WebKit
    AppKit
  ];
in
pkgs.rustPlatform.buildRustPackage rec {
  inherit version pname;

  src = ./src-tauri;

  cargoSha256 = "sha256-n9vlvFtuWwySr5rLGqL7qTwgJZQVQV0sGaNAMrnbA0c=";

  postPatch = ''
    cp -R ${frontend-build} frontend-build
    substituteInPlace tauri.conf.json --replace '"distDir": "../dist",' '"distDir": "frontend-build",'
  '';

  buildInputs = if pkgs.stdenv.isLinux then linux-build-inputs else macos-build-inputs;

  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  doCheck = false;

  passthru.frontend-build = frontend-build;
}

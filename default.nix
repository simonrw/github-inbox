{ lib
, rustPlatform
, cmake
, dbus
, fetchYarnDeps
, freetype
, gtk3
, yarn
, libsoup
, mkYarnPackage
, openssl
, pkg-config
, webkitgtk
}:
let
  pname = "github-inbox";
  version = "unstable";
  src = ./.;

  frontend-build = mkYarnPackage {
    inherit version src;
    pname = "github-inbox-ui";

    offlineCache = fetchYarnDeps {
      yarnLock = src + "/yarn.lock";
      hash = "sha256-PAMl4/TReurrvrg/xuBaBM1oqmmXiRCqCI7qtOJS7+8=";
    };

    packageJSON = ./package.json;

    buildPhase = ''
      export HOME=$(mktemp -d)
      ${yarn}/bin/yarn --offline build
      cp -r deps/github-inbox/dist $out
    '';

    distPhase = ":";
    dontInstall = true;
  };
in
rustPlatform.buildRustPackage rec {
  inherit version src pname;

  sourceRoot = "github-inbox/src-tauri";

  cargoSha256 = "sha256-rv0mqaNqFPmqDFUzTkv5SvRf5PqfCC0Oe5pTAZp8Cjk=";

  postPatch = ''
    echo "Hello world"
    mkdir -p frontend-build
    cp -R ${frontend-build} frontend-build
    substituteInPlace tauri.conf.json --replace '"distDir": "../dist",' '"distDir": "frontend-build",'
  '';

  buildInputs = [
    dbus
    openssl
    freetype
    libsoup
    gtk3
    webkitgtk
    cmake
  ];

  nativeBuildInputs = [
    pkg-config
  ];

  doCheck = false;

  passthru.frontend-build = frontend-build;
}

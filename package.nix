{
  lib,
  fetchFromGitHub,
  rustPlatform,
  openssl,
  pkg-config,
}:

rustPlatform.buildRustPackage rec {
  pname = "dantalian";
  version = "0.4.5";

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  buildInputs = [ openssl ];
  nativeBuildInputs = [ pkg-config ];

  meta = {
    description = "A nfo file generator for your anime. Source from Bangumi. ";
    homepage = "https://github.com/nanozuki/dantalian";
    license = lib.licenses.gpl3;
    maintainers = with lib.maintainers; [ nanozuki ];
  };
}

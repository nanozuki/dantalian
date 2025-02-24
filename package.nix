{
  lib,
  rustPlatform,
  openssl,
  pkg-config,
}:

rustPlatform.buildRustPackage {
  pname = "dantalian";
  version = "0.4.6";

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  src = ./.;

  buildInputs = [ openssl ];
  nativeBuildInputs = [ pkg-config ];

  meta = {
    description = "A nfo file generator for your anime. Source from Bangumi. ";
    homepage = "https://github.com/nanozuki/dantalian";
    license = lib.licenses.gpl3;
    maintainers = with lib.maintainers; [ nanozuki ];
  };
}

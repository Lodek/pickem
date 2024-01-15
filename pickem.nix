{ lib
, stdenv 
, rustPlatform
, pkg-config
, libevdev
} :

rustPlatform.buildRustPackage {
  name = "pickem";
  pname = "pickem";

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  src = ./.;
  nativeBuildInputs = [ pkg-config stdenv ];
  buildInputs = [ ];
}

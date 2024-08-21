{ lib, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "fork-manager";
  version = "0.1.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.lock
      ./Cargo.toml
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "Automatize your fork";
    homepage = "https://github.com/nim65s/fork-manager";
    changelog = "https://github.com/nim65s/fork-manager/blob/main/CHANGELOG.md";
    license = with lib.licenses; [
      asl20
      mit
    ];
    maintainers = with lib.maintainers; [ nim65s ];
    mainProgram = "fork-manager";
  };
}

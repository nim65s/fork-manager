{ lib, rustPlatform, darwin }:

rustPlatform.buildRustPackage {
  pname = "fork-manager";
  version = "0.1.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.lock
      ./Cargo.toml
      ./src
      ./tests
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  buildInputs = lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.Security;

  checkFlags = [
    # This require network access
    "--skip=pr_to_change"
  ];

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

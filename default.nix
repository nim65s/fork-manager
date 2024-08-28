{
  darwin,
  installShellFiles,
  lib,
  mainProgram ? "fork-manager",
  pkg-config,
  rustPlatform,
  stdenv,
}:

rustPlatform.buildRustPackage {
  pname = "fork-manager";
  version = "0.4.0";

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

  nativeBuildInputs = [
    installShellFiles
    pkg-config
  ];

  checkFlags = [
    # This require network access
    "--skip=pr_to_change"
  ];

  postInstall = lib.optionalString (stdenv.buildPlatform.canExecute stdenv.hostPlatform) ''
    installShellCompletion --cmd fork-manager \
      --bash <($out/bin/${mainProgram} --generate bash) \
      --fish <($out/bin/${mainProgram} --generate fish) \
      --zsh <($out/bin/${mainProgram} --generate zsh)
  '';

  meta = {
    inherit mainProgram;
    description = "Automatize your fork";
    homepage = "https://github.com/nim65s/fork-manager";
    changelog = "https://github.com/nim65s/fork-manager/blob/main/CHANGELOG.md";
    license = with lib.licenses; [
      asl20
      mit
    ];
    maintainers = with lib.maintainers; [ nim65s ];
  };
}

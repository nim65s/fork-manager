{
  description = "Automatize your fork";

  inputs = {
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    poetry2nix.url = "github:nix-community/poetry2nix";
    treefmt-nix = {
      url = "github:nim65s/treefmt-nix"; # https://github.com/numtide/treefmt-nix/pull/224
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      flake-parts,
      nixpkgs,
      poetry2nix,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.treefmt-nix.flakeModule ];
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem =
        {
          config,
          pkgs,
          self',
          ...
        }:
        let
          inherit (poetry2nix.lib.mkPoetry2Nix { inherit pkgs; }) mkPoetryApplication;
        in
        {
          apps.default = {
            type = "app";
            program = "${nixpkgs.lib.getExe self'.packages.default}";
          };
          devShells.default = pkgs.mkShell {
            POETRY_VIRTUALENVS_IN_PROJECT = "true";
            nativeBuildInputs = [ config.treefmt.build.wrapper ];
            inputsFrom = [ self'.packages.default ];
            shellHook = ''
              poetry install
              source .venv/bin/activate
            '';
          };
          packages.default = mkPoetryApplication {
            projectDir = ./.;
            meta = {
              description = "Automatize your fork";
              homepage = "https://github.com/nim65s/fork-manager";
              license = nixpkgs.lib.licenses.bsd2;
              mainProgram = "fork-manager";
              mainteners = with nixpkgs.lib.maintainers; [ nim65s ];
            };
          };
          treefmt = {
            projectRootFile = "flake.nix";
            programs = {
              deadnix.enable = true;
              nixfmt-rfc-style.enable = true;
              ruff = {
                check = true;
                format = true;
              };
              toml-sort.enable = true;
            };
          };
        };
    };
}

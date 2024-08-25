{
  description = "Automatize your fork";

  inputs = {
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
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
        {
          devShells = {
            default = pkgs.mkShell {
              nativeBuildInputs = [ config.treefmt.build.wrapper ];
              inputsFrom = [ self'.packages.default ];
              packages = [
                pkgs.cargo-machete
                pkgs.clippy
                pkgs.rustfmt
              ];
            };
            fork-manager = pkgs.mkShell { packages = [ self'.packages.default ]; };
          };
          packages.default = pkgs.callPackage ./. { };
          treefmt = {
            projectRootFile = "flake.nix";
            programs = {
              deadnix.enable = true;
              mdformat.enable = true;
              nixfmt-rfc-style.enable = true;
              rustfmt.enable = true;
              toml-sort = {
                enable = true;
                all = true;
              };
              yamlfmt.enable = true;
            };
          };
        };
    };
}

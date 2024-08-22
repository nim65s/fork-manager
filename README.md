# Fork Manager

[![CI](https://github.com/nim65s/fork-manager/actions/workflows/ci.yml/badge.svg)](https://github.com/nim65s/fork-manager/actions/workflows/ci.yml)
[![Nix](https://github.com/nim65s/fork-manager/actions/workflows/nix.yml/badge.svg)](https://github.com/nim65s/fork-manager/actions/workflows/nix.yml)
[![Release](https://github.com/nim65s/fork-manager/actions/workflows/release.yml/badge.svg)](https://github.com/nim65s/fork-manager/actions/workflows/release.yml)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/nim65s/fork-manager/main.svg)](https://results.pre-commit.ci/latest/github/nim65s/fork-manager/main)

Automatize your fork

## Configuration

```yaml
repo: https://github.com/gepetto/nixpkgs
branch: master
upstream:
  repo: https://github.com/NixOS/nixpkgs
  branch: master  # Same as your branch if not specified
changes:
  - title: "Package HPP"
    repo: https://github.com/nim65s/nixpkgs
    branch: hpp
  - PR: 306524  # get title + repo + branch from upstream github PR
```

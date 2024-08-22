# Fork Manager

[![CI](https://github.com/nim65s/fork-manager/actions/workflows/ci.yml/badge.svg)](https://github.com/nim65s/fork-manager/actions/workflows/ci.yml)
[![Nix](https://github.com/nim65s/fork-manager/actions/workflows/nix.yml/badge.svg)](https://github.com/nim65s/fork-manager/actions/workflows/nix.yml)
[![Release](https://github.com/nim65s/fork-manager/actions/workflows/release.yml/badge.svg)](https://github.com/nim65s/fork-manager/actions/workflows/release.yml)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/nim65s/fork-manager/main.svg)](https://results.pre-commit.ci/latest/github/nim65s/fork-manager/main)

Automatize your fork

## Configuration

```yaml
config: # optional, just displayed in generated README
  repo: https://github.com/gepetto/forks
  branch: master  # default branch from that repo if not specified
forks:
- name: gepetto-nixpkgs-master
  target:
    repo: https://github.com/gepetto/nixpkgs
    branch: master
  upstream:
    repo: https://github.com/NixOS/nixpkgs
    branch: master  # Same as target branch if not specified
  changes:
  - title: Package HPP
    repo: https://github.com/nim65s/nixpkgs
    branch: hpp
  - pr: 331343  # get title + repo + branch from upstream github PR
- name: gepetto-nixpkgs-devel
  target:
    repo: https://github.com/gepetto/nixpkgs
    branch: devel
  upstream:
    repo: https://github.com/gepetto/nixpkgs
    branch: master
  changes:
  - title: prepare hpp-fcl renaming to coal
    repo: https://github.com/nim65s/nixpkgs
    branch: coal
```

# Fork Manager

[![CI](https://github.com/nim65s/fork-manager/actions/workflows/ci.yml/badge.svg)](https://github.com/nim65s/fork-manager/actions/workflows/ci.yml)
[![Nix](https://github.com/nim65s/fork-manager/actions/workflows/nix.yml/badge.svg)](https://github.com/nim65s/fork-manager/actions/workflows/nix.yml)
[![Release](https://github.com/nim65s/fork-manager/actions/workflows/release.yml/badge.svg)](https://github.com/nim65s/fork-manager/actions/workflows/release.yml)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/nim65s/fork-manager/main.svg)](https://results.pre-commit.ci/latest/github/nim65s/fork-manager/main)

Automatize your fork

## Configuration

```yaml
config: # optional, just displayed in generated README
  repo: git@github.com:gepetto/forks
  branch: master  # default branch from that repo if not specified
forks:
- name: gepetto-nixpkgs-master
  target:
    repo: git@github.com:gepetto/nixpkgs
    branch: master
  upstream:
    repo: git@github.com:NixOS/nixpkgs
    branch: master  # Same as target branch if not specified
  changes:
  - title: Package HPP  # default to branch name if not specified
    repo: git@github.com:nim65s/nixpkgs
    branch: hpp
  - pr: 331343  # get title + repo + branch from upstream github PR
- name: gepetto-nixpkgs-devel
  target:
    repo: git@github.com:gepetto/nixpkgs
    branch: devel
  upstream:
    repo: git@github.com:gepetto/nixpkgs
    branch: master
  changes:
  - title: prepare hpp-fcl renaming to coal
    repo: git@github.com:nim65s/nixpkgs
    branch: coal
```

This configure the gepetto/forks github project, which manage the `gepetto-nixpkgs-master` and `gepetto-nixpkgs-devel`
forks:

- The first takes github:NixOS/nixpkgs/master, merge one branch and one PR, and force push that to github:gepetto/nixpkgs/master.
- Thes second take this fresh github:gepetto/nixpkgs/master, merge an additinal branch, and force push that to github:gepetto/nixpkgs/devel.

It will generate a `./update.sh` script which will work in one submodule per fork, and whose actual push is gated with a `push` flag.

If a file named `test-{fork}.sh` exists, it will be run.

## CLI

```
$ fork-manager -h
Automatize your fork

Usage: fork-manager [OPTIONS]

Options:
  -c, --config-file <CONFIG_FILE>      Path to the configuration file. If not given, or not a file, this will be
                                       searched according to arguments "project" and "filename"
                                       [env: FORK_MANAGER_CONFIG_FILE=] [default: ./fork-manager.yaml]
  -f, --filename <FILENAME>            Name of the configuration file to look for
                                       [env: FORK_MANAGER_CONFIG_FILENAME=] [default: fork-manager.yaml]
  -p, --project <PROJECT>              Path to the project where to look for
                                       [env: FORK_MANAGER_PROJECT=] [default: .]
  -u, --update-script <UPDATE_SCRIPT>  Name of the script to generate
                                       [env: FORK_MANAGER_UPDATE_SCRIPT=] [default: update.sh]
      --generate <GENERATOR>           If provided, outputs the completion file for given shell and exit
                                       [possible values: bash, elvish, fish, powershell, zsh]
  -d, --dry-run                        Only check config, don't run git commands
  -h, --help                           Print help
  -V, --version                        Print version
```

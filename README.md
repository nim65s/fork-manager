# Fork Manager

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

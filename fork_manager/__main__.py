"""fork-manager main entrypoint module."""

from . import args, load


def main():
    """fork-manager main entrypoint function."""
    print(load(**vars(args())))

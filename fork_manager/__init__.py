"""fork-manager main module."""

from argparse import ArgumentParser, Namespace
from dataclasses import dataclass
from pathlib import Path

from ruamel.yaml import YAML


@dataclass
class Repo:
    """A Git repository."""

    url: str
    branch: str


@dataclass
class Change(Repo):
    """A change to track."""

    title: str


@dataclass
class Config(Repo):
    """Main configuration."""

    uptsream: Repo
    changes: [Change]


def args() -> Namespace:
    """Parse arguments."""
    parser = ArgumentParser()
    parser.add_argument("--project", default=".", type=Path)
    parser.add_argument("--config-file", default="fork-manager.yaml", type=Path)
    return parser.parse_args()


def load(project: Path, config_file: Path):
    """Load configuration."""
    yaml = YAML()
    yaml.register_class(Config)

    config = (project / config_file).read_text()
    return yaml.load(config)

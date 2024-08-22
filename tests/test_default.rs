use clap::Parser;
use indoc::indoc;
use std::path::PathBuf;

use fork_manager::{Args, Change, Config, Fork, Repo, Update, PR};

#[test]
fn test_cli() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut args = Args::parse_from(["fork-manager", "-p", "tests"]);
    args.process().unwrap();
    assert_eq!(args.project, root.join("tests"));

    let mut args = Args::parse_from(["fork-manager", "-f", "tests/fork-manager.yaml"]);
    args.process().unwrap();
    assert_eq!(args.project, root);

    let mut args = Args::parse_from(["fork-manager", "-p", "tests/empty"]);
    args.process().unwrap();
    assert_eq!(args.project, root.join("tests"));
}

#[test]
fn test_serde() {
    let yaml = indoc! {"
        config:
          url: https://github.com/gepetto/forks
          branch: master
        forks:
        - name: gepetto-nixpkgs-master
          target:
            url: https://github.com/gepetto/nixpkgs
            branch: master
          upstream:
            url: https://github.com/NixOS/nixpkgs
            branch: master
          changes:
          - title: Package HPP
            url: https://github.com/nim65s/nixpkgs
            branch: hpp
          - pr: 331343
        - name: gepetto-nixpkgs-devel
          target:
            url: https://github.com/gepetto/nixpkgs
            branch: devel
          upstream:
            url: https://github.com/gepetto/nixpkgs
            branch: master
          changes:
          - url: https://github.com/nim65s/nixpkgs
            branch: coal
        "};

    let config = Config {
        config: Some(Repo {
            url: "https://github.com/gepetto/forks".to_string(),
            branch: Some("master".to_string()),
        }),
        forks: vec![
            Fork {
                name: "gepetto-nixpkgs-master".to_string(),
                target: Repo {
                    url: "https://github.com/gepetto/nixpkgs".to_string(),
                    branch: Some("master".to_string()),
                },
                upstream: Repo {
                    url: "https://github.com/NixOS/nixpkgs".to_string(),
                    branch: Some("master".to_string()),
                },
                changes: vec![
                    Update::Change(Change {
                        title: Some("Package HPP".to_string()),
                        url: "https://github.com/nim65s/nixpkgs".to_string(),
                        branch: "hpp".to_string(),
                    }),
                    Update::PR(PR { pr: 331343 }),
                ],
            },
            Fork {
                name: "gepetto-nixpkgs-devel".to_string(),
                target: Repo {
                    url: "https://github.com/gepetto/nixpkgs".to_string(),
                    branch: Some("devel".to_string()),
                },
                upstream: Repo {
                    url: "https://github.com/gepetto/nixpkgs".to_string(),
                    branch: Some("master".to_string()),
                },
                changes: vec![Update::Change(Change {
                    title: None,
                    url: "https://github.com/nim65s/nixpkgs".to_string(),
                    branch: "coal".to_string(),
                })],
            },
        ],
    };
    assert_eq!(config, serde_yml::from_str(yaml).unwrap());
    assert_eq!(serde_yml::to_string(&config).unwrap(), yaml);
}

#[tokio::test]
async fn test_pr_to_change() {
    let yaml_from = indoc! {"
        forks:
        - name: my-fork
          target:
            url: https://github.com/gepetto/nixpkgs
            branch: master
          upstream:
            url: https://github.com/NixOS/nixpkgs
          changes:
          - url: https://github.com/nim65s/nixpkgs
            branch: hpp
          - pr: 331343
        "};
    let yaml_to = indoc! {"
        forks:
        - name: my-fork
          target:
            url: https://github.com/gepetto/nixpkgs
            branch: master
          upstream:
            url: https://github.com/NixOS/nixpkgs
            branch: master
          changes:
          - title: hpp
            url: https://github.com/nim65s/nixpkgs
            branch: hpp
          - title: 'casadi: init at 3.6.6'
            url: https://github.com/nim65s/nixpkgs
            branch: casadi
        "};
    let mut config_from = Config {
        config: None,
        forks: vec![Fork {
            name: "my-fork".to_string(),
            target: Repo {
                url: "https://github.com/gepetto/nixpkgs".to_string(),
                branch: Some("master".to_string()),
            },
            upstream: Repo {
                url: "https://github.com/NixOS/nixpkgs".to_string(),
                branch: None,
            },
            changes: vec![
                Update::Change(Change {
                    title: None,
                    url: "https://github.com/nim65s/nixpkgs".to_string(),
                    branch: "hpp".to_string(),
                }),
                Update::PR(PR { pr: 331343 }),
            ],
        }],
    };
    let config_to = Config {
        config: None,
        forks: vec![Fork {
            name: "my-fork".to_string(),
            target: Repo {
                url: "https://github.com/gepetto/nixpkgs".to_string(),
                branch: Some("master".to_string()),
            },
            upstream: Repo {
                url: "https://github.com/NixOS/nixpkgs".to_string(),
                branch: Some("master".to_string()),
            },
            changes: vec![
                Update::Change(Change {
                    title: Some("hpp".to_string()),
                    url: "https://github.com/nim65s/nixpkgs".to_string(),
                    branch: "hpp".to_string(),
                }),
                Update::Change(Change {
                    title: Some("casadi: init at 3.6.6".to_string()),
                    url: "https://github.com/nim65s/nixpkgs".to_string(),
                    branch: "casadi".to_string(),
                }),
            ],
        }],
    };
    assert_eq!(config_from, serde_yml::from_str(yaml_from).unwrap());
    assert_eq!(config_to, serde_yml::from_str(yaml_to).unwrap());
    assert_eq!(serde_yml::to_string(&config_from).unwrap(), yaml_from);
    assert_eq!(serde_yml::to_string(&config_to).unwrap(), yaml_to);

    assert_ne!(config_from, config_to);
    assert_ne!(config_from, serde_yml::from_str(yaml_to).unwrap());
    assert_ne!(serde_yml::to_string(&config_from).unwrap(), yaml_to);

    config_from.update().await.unwrap();

    assert_eq!(config_from, config_to);
    assert_eq!(config_from, serde_yml::from_str(yaml_to).unwrap());
    assert_eq!(serde_yml::to_string(&config_from).unwrap(), yaml_to);
}

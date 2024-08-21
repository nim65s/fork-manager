use fork_manager::{Change, Config, Update, Upstream, PR};
use indoc::indoc;

#[test]
fn test_serde() {
    let yaml = indoc! {"
        repo: https://github.com/gepetto/nixpkgs
        branch: master
        upstream:
          repo: https://github.com/NixOS/nixpkgs
          branch: master
        changes:
        - title: Package HPP
          repo: https://github.com/nim65s/nixpkgs
          branch: hpp
        - pr: 331343
        "};

    let config = Config {
        repo: "https://github.com/gepetto/nixpkgs".to_string(),
        branch: "master".to_string(),
        upstream: Upstream {
            repo: "https://github.com/NixOS/nixpkgs".to_string(),
            branch: Some("master".to_string()),
        },
        changes: vec![
            Update::Change(Change {
                title: "Package HPP".to_string(),
                repo: "https://github.com/nim65s/nixpkgs".to_string(),
                branch: "hpp".to_string(),
            }),
            Update::PR(PR { pr: 331343 }),
        ],
    };
    assert_eq!(config, serde_yml::from_str(yaml).unwrap());
    assert_eq!(serde_yml::to_string(&config).unwrap(), yaml);
}

#[tokio::test]
async fn test_pr_to_change() {
    let yaml_from = indoc! {"
        repo: https://github.com/gepetto/nixpkgs
        branch: master
        upstream:
          repo: https://github.com/NixOS/nixpkgs
          branch: master
        changes:
        - title: Package HPP
          repo: https://github.com/nim65s/nixpkgs
          branch: hpp
        - pr: 331343
        "};
    let yaml_to = indoc! {"
        repo: https://github.com/gepetto/nixpkgs
        branch: master
        upstream:
          repo: https://github.com/NixOS/nixpkgs
          branch: master
        changes:
        - title: Package HPP
          repo: https://github.com/nim65s/nixpkgs
          branch: hpp
        - title: 'casadi: init at 3.6.6'
          repo: https://github.com/nim65s/nixpkgs
          branch: casadi
        "};
    let mut config_from = Config {
        repo: "https://github.com/gepetto/nixpkgs".to_string(),
        branch: "master".to_string(),
        upstream: Upstream {
            repo: "https://github.com/NixOS/nixpkgs".to_string(),
            branch: Some("master".to_string()),
        },
        changes: vec![
            Update::Change(Change {
                title: "Package HPP".to_string(),
                repo: "https://github.com/nim65s/nixpkgs".to_string(),
                branch: "hpp".to_string(),
            }),
            Update::PR(PR { pr: 331343 }),
        ],
    };
    let config_to = Config {
        repo: "https://github.com/gepetto/nixpkgs".to_string(),
        branch: "master".to_string(),
        upstream: Upstream {
            repo: "https://github.com/NixOS/nixpkgs".to_string(),
            branch: Some("master".to_string()),
        },
        changes: vec![
            Update::Change(Change {
                title: "Package HPP".to_string(),
                repo: "https://github.com/nim65s/nixpkgs".to_string(),
                branch: "hpp".to_string(),
            }),
            Update::Change(Change {
                title: "casadi: init at 3.6.6".to_string(),
                repo: "https://github.com/nim65s/nixpkgs".to_string(),
                branch: "casadi".to_string(),
            }),
        ],
    };
    assert_eq!(config_from, serde_yml::from_str(yaml_from).unwrap());
    assert_eq!(config_to, serde_yml::from_str(yaml_to).unwrap());
    assert_eq!(serde_yml::to_string(&config_from).unwrap(), yaml_from);
    assert_eq!(serde_yml::to_string(&config_to).unwrap(), yaml_to);

    assert_ne!(config_from, config_to);
    assert_ne!(config_from, serde_yml::from_str(yaml_to).unwrap());
    assert_ne!(serde_yml::to_string(&config_from).unwrap(), yaml_to);

    config_from.get_prs().await.unwrap();

    assert_eq!(config_from, config_to);
    assert_eq!(config_from, serde_yml::from_str(yaml_to).unwrap());
    assert_eq!(serde_yml::to_string(&config_from).unwrap(), yaml_to);
}

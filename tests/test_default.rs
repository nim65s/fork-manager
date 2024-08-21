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
        - pr: 306524
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
            Update::PR(PR { pr: 306524 }),
        ],
    };
    assert_eq!(config, serde_yml::from_str(yaml).unwrap());
    assert_eq!(serde_yml::to_string(&config).unwrap(), yaml);
}

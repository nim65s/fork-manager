use minijinja::syntax::SyntaxConfig;
use minijinja::{context, Environment};

use super::{ForkManager, Result};

pub fn remote_name(value: String) -> String {
    value
        .replace("https://", "")
        .replace("git@", "")
        .replace(":", "/")
}

pub fn generate(fm: &ForkManager) -> Result<()> {
    // use a syntax which won't mess too much with bash for shellcheck
    let syntax = SyntaxConfig::builder()
        .block_delimiters("#{", "}#")
        .variable_delimiters("'{", "}'")
        .comment_delimiters("#/*", "#*/")
        .build()?;
    let mut env = Environment::new();
    env.set_syntax(syntax);
    env.add_filter("remote_name", remote_name);
    env.add_template("update.sh", include_str!("update.sh"))?;
    let tmpl = env.get_template("update.sh").unwrap();
    println!(
        "{}",
        tmpl.render(context! {
            config => fm.config.config,
            forks => fm.config.forks,
            remotes => fm.config.remotes(),
            push => fm.args.push,
        })
        .unwrap()
    );
    Ok(())
}

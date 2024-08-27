use std::fs::{set_permissions, File};
use std::io::Write;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

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
    let ctx = context! {
        config => fm.config.config,
        forks => fm.config.forks,
        remotes => fm.config.remotes(),
        push => fm.args.push,
    };
    let script = fm.args.project.join(&fm.args.update_script);
    let mut file = File::create(&script)?;
    let content = tmpl.render(ctx)?;
    file.write_all(content.as_bytes())?;
    file.write_all(&[b'\n'])?;
    #[cfg(target_family = "unix")]
    {
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o755);
        set_permissions(script, perms)?;
    }
    Ok(())
}

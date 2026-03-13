use crate::commands::{dated_name, git_run};
use crate::context::RunContext;
use anyhow::Result;

pub fn clone(ctx: &RunContext, git_uri: String, name: Option<String>) -> Result<()> {
    let name = name.unwrap_or_else(|| generate_default_name(&git_uri));
    let dir_name = dated_name(&name);
    let target_path = ctx.prepare_target_path(&dir_name)?;

    git_run(
        ctx,
        &["clone", &git_uri, &target_path.to_string_lossy()],
        &target_path,
        &format!("git clone {git_uri}"),
    )
}

fn generate_default_name(git_uri: &str) -> String {
    let parts: Vec<&str> = git_uri.trim_end_matches(".git").split('/').collect();
    if parts.len() >= 2 {
        let user = parts[parts.len() - 2];
        let repo = parts[parts.len() - 1];
        format!("{user}-{repo}")
    } else {
        parts.last().copied().unwrap_or("repo").to_string()
    }
}

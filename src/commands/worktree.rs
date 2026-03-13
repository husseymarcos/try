use crate::commands::{dated_name, git_run};
use crate::context::RunContext;
use anyhow::Result;
use std::path::Path;

pub fn worktree_dir(ctx: &RunContext, name: Option<String>) -> Result<()> {
    let name = name.ok_or_else(|| anyhow::anyhow!("Name required for worktree"))?;
    let dir_name = dated_name(&name);
    let target_path = ctx.prepare_target_path(&dir_name)?;

    if !is_git_repo(&std::env::current_dir()?)? {
        anyhow::bail!("Not in a git repository");
    }

    git_run(
        ctx,
        &["worktree", "add", &target_path.to_string_lossy()],
        &target_path,
        "git worktree add",
    )
}

pub(crate) fn is_git_repo(path: &Path) -> Result<bool> {
    let git_dir = path.join(".git");
    Ok(git_dir.exists() && git_dir.is_dir())
}

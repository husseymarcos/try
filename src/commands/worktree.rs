use crate::context::RunContext;
use anyhow::Result;

pub fn worktree_dir(ctx: &RunContext, name: Option<String>) -> Result<()> {
    let name = name.ok_or_else(|| anyhow::anyhow!("Name required for worktree"))?;
    let dir_name = ctx.dated_name(&name);
    let target_path = ctx.prepare_target_path(&dir_name)?;

    if !ctx.is_git_repo(&std::env::current_dir()?)? {
        anyhow::bail!("Not in a git repository");
    }

    ctx.git_run(
        &["worktree", "add", &target_path.to_string_lossy()],
        &target_path,
        "git worktree add",
    )
}

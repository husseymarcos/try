use crate::context::RunContext;
use anyhow::{Context, Result};
use chrono::Local;
use std::path::Path;
use std::process::Command;

pub mod cd;
pub mod clone;
pub mod init;
pub mod worktree;

pub fn dated_name(name: &str) -> String {
    format!("{}-{}", Local::now().format("%Y-%m-%d"), name)
}

pub fn git_run(ctx: &RunContext, args: &[&str], target_path: &Path, error_msg: &str) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .status()
        .with_context(|| error_msg.to_string())?;

    if !status.success() {
        anyhow::bail!("{error_msg} failed");
    }

    ctx.print_cd(target_path);
    Ok(())
}

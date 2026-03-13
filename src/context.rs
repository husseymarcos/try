use anyhow::{Context as _, Result};
use chrono::Local;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct RunContext {
    pub root: PathBuf,
}

impl RunContext {
    pub fn prepare_target_path(&self, dir_name: &str) -> Result<PathBuf> {
        let target_path = self.root.join(dir_name);
        if target_path.exists() {
            anyhow::bail!("Directory already exists: {}", target_path.display());
        }
        std::fs::create_dir_all(&self.root)
            .with_context(|| format!("Failed to create root directory: {}", self.root.display()))?;
        Ok(target_path)
    }

    pub fn print_cd(&self, path: &Path) {
        println!("cd '{}'", path.to_string_lossy());
    }

    pub fn dated_name(&self, name: &str) -> String {
        format!("{}-{}", Local::now().format("%Y-%m-%d"), name)
    }

    pub fn git_run(&self, args: &[&str], target_path: &Path, error_msg: &str) -> Result<()> {
        let status = Command::new("git")
            .args(args)
            .status()
            .with_context(|| error_msg.to_string())?;

        if !status.success() {
            anyhow::bail!("{error_msg} failed");
        }

        self.print_cd(target_path);
        Ok(())
    }

    pub fn is_git_repo(&self, path: &Path) -> Result<bool> {
        let git_dir = path.join(".git");
        Ok(git_dir.exists() && git_dir.is_dir())
    }

    pub fn exe_path(&self) -> String {
        std::env::current_exe()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "trust".to_string())
    }
}

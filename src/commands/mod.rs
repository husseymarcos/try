use crate::context::RunContext;
use anyhow::Result;
use clap::Subcommand;
use std::path::PathBuf;

pub mod cd;
pub mod clone;
pub mod init;
pub mod worktree;

const GIT_URL_PREFIXES: &[&str] = &["http://", "https://", "git@", "git://"];

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    #[command(about = "Output shell function definition for shell integration")]
    Init { path: Option<PathBuf> },
    
    #[command(about = "Interactive directory selector with fuzzy search")]
    Cd { query: Option<String> },
    
    #[command(about = "Clone a git repository into a dated directory")]
    Clone { url: String, name: Option<String> },
    
    #[command(
        name = "worktree",
        alias = ".",
        about = "Create a git worktree in a dated directory"
    )]
    Worktree { name: String },
}

pub fn looks_like_git_url(s: &str) -> bool {
    GIT_URL_PREFIXES.iter().any(|p| s.starts_with(p)) || (s.contains("://") && s.contains(".git"))
}

pub trait Runnable {
    fn run(self, ctx: &RunContext) -> Result<()>;
}

impl Runnable for Command {
    fn run(self, ctx: &RunContext) -> Result<()> {
        match self {
            Command::Init { path } => init::init(ctx, path),
            Command::Cd { query } => cd::cd(ctx, query),
            Command::Clone { url, name } => clone::clone(ctx, url, name),
            Command::Worktree { name } => worktree::worktree_dir(ctx, Some(name)),
        }
    }
}

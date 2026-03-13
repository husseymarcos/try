use crate::commands::{Command, Runnable, looks_like_git_url};
use crate::context::RunContext;
use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "trust", 
    version,
    about = "Ephemeral workspace manager for your experiments",
    long_about = "Quickly create and jump into fresh folders for your experiments – an ephemeral workspace manager."
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(
        long,
        value_name = "PATH",
        env = "TRUST_PATH",
        help = "Override tries directory (default: ~/src/tries)"
    )]
    pub path: Option<PathBuf>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub shorthand: Vec<String>,
}

impl Args {
    pub fn run(self) -> Result<()> {
        let root = self.root_path()?;
        let ctx = RunContext { root };
        self.resolve_command().run(&ctx)
    }

    pub fn resolve_command(&self) -> Command {
        if let Some(cmd) = self.command.clone() {
            return cmd;
        }

        let query = if self.shorthand.is_empty() {
            None
        } else {
            Some(self.shorthand.join(" "))
        };

        if let Some(ref q) = query {
            if looks_like_git_url(q) {
                return Command::Clone {
                    url: q.clone(),
                    name: None,
                };
            }
        }

        Command::Cd { query }
    }

    fn root_path(&self) -> Result<PathBuf> {
        if let Some(p) = &self.path {
            return Ok(p.clone());
        }

        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        Ok(PathBuf::from(home).join("src").join("tries"))
    }
}

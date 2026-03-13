mod args;
mod commands;
mod context;
mod tui;

use anyhow::Result;
use args::Args;
use clap::Parser;

fn main() -> Result<()> {
    Args::parse().run()
}

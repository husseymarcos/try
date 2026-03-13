use crate::command::looks_like_git_url;
use crate::context::RunContext;
use anyhow::Result;

pub fn cd(ctx: &RunContext, query: Option<String>) -> Result<()> {
    if let Some(ref q) = query.as_ref().filter(|s| looks_like_git_url(s)) {
        return crate::commands::clone::clone(ctx, q.to_string(), None);
    }
    crate::tui::run(ctx, query)
}

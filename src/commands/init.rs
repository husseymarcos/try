use crate::context::RunContext;
use anyhow::Result;
use std::path::PathBuf;

pub fn init(ctx: &RunContext, path: Option<PathBuf>) -> Result<()> {
    let root = path.unwrap_or_else(|| ctx.root.clone());
    let root_str = root.to_string_lossy();
    
    let shell = std::env::var("SHELL").unwrap_or_default();
    let shell_name = std::path::Path::new(&shell)
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_else(|| "bash".into());

    let exe = std::env::current_exe()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "trust".to_string());

    let template = match shell_name.as_ref() {
        "fish" => format!(
            r#"function try
    set -x TRUST_PATH "{root_str}"
    set output ({exe} $argv)
    if [ -n "$output" ]
        eval $output
    end
end"#
        ),
        _ => format!(
            r#"try() {{
    local output
    export TRUST_PATH="{root_str}"
    output=$("{exe}" "$@")
    if [ -n "$output" ]; then
        eval "$output"
    fi
}}"#
        ),
    };
    
    println!("{template}");
    Ok(())
}

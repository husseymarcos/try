use crate::context::RunContext;
use anyhow::Result;
use std::path::PathBuf;

fn current_exe_path() -> String {
    std::env::current_exe()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "trust".to_string())
}

pub fn init(ctx: &RunContext, path: Option<PathBuf>) -> Result<()> {
    let root = path.unwrap_or_else(|| ctx.root.clone());
    let root_str = root.to_string_lossy();
    
    let shell = std::env::var("SHELL")
        .ok()
        .and_then(|s| {
            std::path::Path::new(&s)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "bash".to_string());
        
    let function = match shell.as_str() {
        "fish" => fish_function(&root_str),
        _ => bash_function(&root_str),
    };
    
    println!("{function}");
    Ok(())
}

fn bash_function(root: &str) -> String {
    let exe = current_exe_path();
    format!(
        r#"try() {{
    local output
    export TRUST_PATH="{root}"
    output=$("{exe}" "$@")
    if [ -n "$output" ]; then
        eval "$output"
    fi
}}"#
    )
}

fn fish_function(root: &str) -> String {
    let exe = current_exe_path();
    format!(
        r#"function try
    set -x TRUST_PATH "{root}"
    set output ({exe} $argv)
    if [ -n "$output" ]
        eval $output
    end
end"#
    )
}

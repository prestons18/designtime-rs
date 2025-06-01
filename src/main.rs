use designtime_rs::{workspace, Runtime, StyleMan};
use workspace::validate_and_load_workspace;
use std::path::Path;

fn main() {
    // Load workspace config (from current directory right now)
    let config_path = Path::new("./designtime.json");
    let workspace_config = match validate_and_load_workspace(config_path) {
        Ok(cfg) => {
            println!("✅ Loaded workspace config: {:?}", cfg.project.name);
            cfg
        }
        Err(e) => {
            eprintln!("Failed to load workspace config: {}", e);
            std::process::exit(1);
        }
    };

    let mut runtime = Runtime::new(workspace_config);
    let source = include_str!("examples/night01.page.dts");
    runtime.run(source);
}

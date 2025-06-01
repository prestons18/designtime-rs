use designtime_rs::{workspace, Lexer, Parser, Runtime};
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

    let runtime = Runtime::new(workspace_config);
    runtime.process_unocss();

    // Now read and parse the .dts source file
    let source = include_str!("examples/night01.page.dts");
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(node) => println!("{:#?}", node),
        Err(e) => eprintln!("Parse error: {}", e),
    }
}

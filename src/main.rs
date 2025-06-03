use designtime_rs::engine::runtime::Runtime;
use designtime_rs::{validate_and_load_workspace, Lexer, Parser, RenderLib, Watchman};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = color_eyre::install();

    let file_path = "./src/examples/night01.page.dts";
    let file_contents = std::fs::read_to_string(file_path)?;
    let lex = Lexer::new(&file_contents);
    let mut parse = Parser::new(lex);
    let parsed_nodes = parse.parse()?;

    let workspace = PathBuf::from("./designtime.json");
    let config = validate_and_load_workspace(workspace).expect("Failed to load workspace config");
    let mut runtime = Runtime::new(config);

    runtime.run(vec![parsed_nodes]);

    let render_lib = RenderLib::new(runtime);
    let watchman = Watchman::new(render_lib);
    watchman.run().await?;
    Ok(())
}
use designtime_rs::{
    engine::Runtime, validate_and_load_workspace, watchman
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let workspace = PathBuf::from("./designtime.json");
    let config = validate_and_load_workspace(workspace).expect("Failed to load workspace config");

    // Create runtime with the config
    let runtime = Runtime::new(config);
    
    // Start watchman with the runtime
    watchman(runtime).await
}
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

#[derive(Debug, Deserialize)]
pub struct WorkspaceConfig {
    pub project: ProjectInfo,
    pub packages: Option<Packages>,
    pub unocss: Option<UnoCSS>,
    pub routes: Option<Routes>,
    pub theme: Option<Theme>,
    pub components: Option<Components>,
    pub build: Option<Build>,
    pub dev_server: Option<DevServer>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Packages {
    pub dependencies: Option<Vec<String>>,
    pub dev_dependencies: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UnoCSS {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub preset: Option<String>,
    #[serde(default)]
    pub config_file: Option<String>,
    #[serde(default)]
    pub scan: Option<Scan>,
}


#[derive(Debug, Deserialize)]
pub struct Scan {
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Routes {
    pub auto_register: Option<bool>,
    pub pages_dir: Option<String>,
    pub exclude: Option<Vec<String>>,
    pub not_found_page: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    pub default: Option<String>,
    pub available: Option<Vec<String>>,
    pub custom_stylesheet: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Components {
    pub auto_register: Option<bool>,
    pub directories: Option<Vec<String>>,
    pub aliases: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Build {
    pub output_dir: Option<String>,
    pub target: Option<Vec<String>>,
    pub minify: Option<bool>,
    pub source_maps: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct DevServer {
    pub port: Option<u16>,
    pub hot_reload: Option<bool>,
    pub open_browser: Option<bool>,
}

impl WorkspaceConfig {
    pub fn is_unocss_enabled(&self) -> bool {
        self.unocss.as_ref().map_or(false, |u| u.enabled)
    }
}

pub fn watch_workspace_config(path: &Path, on_change: impl Fn() + Send + 'static) {
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx).expect("Failed to create watcher");

    watcher.watch(path, RecursiveMode::NonRecursive).expect("Failed to watch file");

    std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(event) => {
                    println!("Config changed: {:?}", event);
                    on_change();
                }
                Err(e) => println!("Watch error: {:?}", e),
            }
        }
    });
}

pub fn validate_and_load_workspace<P: AsRef<std::path::Path>>(path: P) -> Result<WorkspaceConfig, anyhow::Error> {
    let schema_str = include_str!("../../designtime.schema.json"); // todo: make this configurable
    let config_str = std::fs::read_to_string(path)?;

    let schema: Value = serde_json::from_str(schema_str)?;
    let config: Value = serde_json::from_str(&config_str)?;

    let validator = jsonschema::validator_for(&schema)?;

    let errors: Vec<String> = validator
        .validate(&config)
        .err()
        .map(|e| e.to_string())
        .into_iter()
        .collect();

    if !errors.is_empty() {
        anyhow::bail!("Config validation errors:\n{}", errors.join("\n"));
    }

    let workspace_config = serde_json::from_value(config)?;
    Ok(workspace_config)
}

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

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

#[derive(Debug, Deserialize)]
pub struct UnoCSS {
    pub enabled: bool,
    pub preset: Option<String>,
    pub config_file: Option<String>,
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

pub fn validate_and_load_workspace() -> WorkspaceConfig {
    let schema_str =
        fs::read_to_string("./designtime.schema.json").expect("Missing designtime.schema.json");
    let config_str = fs::read_to_string("./designtime.json").expect("Missing designtime.json");

    let schema: Value = serde_json::from_str(&schema_str).expect("Invalid schema JSON");
    let config: Value = serde_json::from_str(&config_str).expect("Invalid config JSON");

    let validator = jsonschema::validator_for(&schema).expect("Failed to compile schema");

    let mut errors = Vec::new();
    for error in validator.iter_errors(&config) {
        errors.push(format!(
            "❌ Validation error at {}: {}",
            error.instance_path, error
        ));
    }

    if !errors.is_empty() {
        for error in errors {
            println!("{error}");
        }
        println!("Invalid designtime.json") // todo: hook up to a global ErrorManager
    } else {
        println!("✅ designtime.json is valid!");
    }

    serde_json::from_value(config).expect("Failed to deserialize designtime.json")
}

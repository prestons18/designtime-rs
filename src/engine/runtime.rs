use crate::{workspace::WorkspaceConfig};

pub struct Runtime {
    workspace: WorkspaceConfig,
    // TODO: add more fields like style man, cache, etc
}

impl Runtime {
    pub fn new(workspace: WorkspaceConfig) -> Self {
        Self { workspace }
    }

    pub fn process_unocss(&self) {
        if let Some(unocss) = &self.workspace.unocss {
            if unocss.enabled {
                println!("UnoCSS is enabled");
                if let Some(preset) = &unocss.preset {
                    println!("Using preset: {}", preset);
                }
                if let Some(config_file) = &unocss.config_file {
                    println!("Using config file: {}", config_file);
                }
                if let Some(scan) = &unocss.scan {
                    println!("Scan includes: {:?}", scan.include);
                    println!("Scan excludes: {:?}", scan.exclude);
                }

                // TODO: Call the style man's function to load / initialize
                // For example:
                // style_man::load_unocss(unocss);
            } else {
                println!("UnoCSS is disabled");
            }
        } else {
            println!("No UnoCSS config found");
        }
    }
}
// This is basic, I plan to extend later.

use std::collections::HashSet;

pub struct StyleMan {
    // Keep unique class names seen
    class_names: HashSet<String>,
}

impl StyleMan {
    pub fn new() -> Self {
        Self {
            class_names: HashSet::new(),
        }
    }

    // Add classes found (deduplicated)
    pub fn add_classes<I: IntoIterator<Item = String>>(&mut self, classes: I) {
        for class in classes {
            self.class_names.insert(class);
        }
    }

    // Generate CSS string for all added classes
    pub fn generate_css(&self) -> String {
        let mut css = String::new();
        for class_name in &self.class_names {
            if let Some(rule) = Self::generate_css_for_class(class_name) {
                css.push_str(&format!(".{} {{ {} }}\n", class_name, rule));
            }
        }
        css
    }

    // Core rule matcher: input class name -> CSS rule body
    fn generate_css_for_class(class_name: &str) -> Option<String> {
        if let Some(css) = Self::spacing_rule(class_name) {
            return Some(css);
        }
        if let Some(css) = Self::display_rule(class_name) {
            return Some(css);
        }
        if let Some(css) = Self::color_rule(class_name) {
            return Some(css);
        }
        None
    }

    // Spacing rules for p- and m-
    fn spacing_rule(class_name: &str) -> Option<String> {
        if class_name.starts_with("p-") {
            let val = class_name.trim_start_matches("p-").parse::<f32>().ok()?;
            let rem = val * 0.25;
            return Some(format!("padding: {}rem;", rem));
        }
        if class_name.starts_with("m-") {
            let val = class_name.trim_start_matches("m-").parse::<f32>().ok()?;
            let rem = val * 0.25;
            return Some(format!("margin: {}rem;", rem));
        }
        None
    }

    // Display rules for flex and grid
    fn display_rule(class_name: &str) -> Option<String> {
        match class_name {
            "flex" => Some("display: flex;".to_string()),
            "grid" => Some("display: grid;".to_string()),
            _ => None,
        }
    }

    // Color rules for bg-{color}
    fn color_rule(class_name: &str) -> Option<String> {
        if let Some(color_name) = class_name.strip_prefix("bg-") {
            let color = Self::color_map(color_name)?;
            return Some(format!("background-color: {};", color));
        }
        None
    }

    // Basic color map
    fn color_map(name: &str) -> Option<&'static str> {
        match name {
            "red" => Some("#f44336"),
            "blue" => Some("#2196f3"),
            "green" => Some("#4caf50"),
            "yellow" => Some("#ffeb3b"),
            "black" => Some("#000000"),
            "white" => Some("#ffffff"),
            _ => None,
        }
    }
}

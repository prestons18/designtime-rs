use std::collections::HashSet;

pub struct StyleMan {
    class_names: HashSet<String>,
}

impl StyleMan {
    pub fn new() -> Self {
        Self {
            class_names: HashSet::new(),
        }
    }

    pub fn add_classes<I: IntoIterator<Item = String>>(&mut self, classes: I) {
        for class in classes {
            self.class_names.insert(class);
        }
    }

    pub fn generate_css(&self) -> String {
        let mut css = String::new();
        for class_name in &self.class_names {
            if let Some(rule) = Self::generate_css_for_class(class_name) {
                css.push_str(&format!(".{} {{ {} }}\n", class_name, rule));
            }
        }
        css
    }

    fn generate_css_for_class(class_name: &str) -> Option<String> {
        Self::spacing_rule(class_name)
            .or_else(|| Self::display_rule(class_name))
            .or_else(|| Self::color_rule(class_name))
            .or_else(|| Self::text_color_rule(class_name))
            .or_else(|| Self::font_weight_rule(class_name))
            .or_else(|| Self::flex_direction_rule(class_name))
            .or_else(|| Self::align_items_rule(class_name))
            .or_else(|| Self::justify_content_rule(class_name))
    }

    // Spacing rules for p- and m-
    fn spacing_rule(class_name: &str) -> Option<String> {
        if let Some(val_str) = class_name.strip_prefix("p-") {
            let val = val_str.parse::<f32>().ok()?;
            let rem = val * 0.25;
            return Some(format!("padding: {}rem;", rem));
        }
        if let Some(val_str) = class_name.strip_prefix("m-") {
            let val = val_str.parse::<f32>().ok()?;
            let rem = val * 0.25;
            return Some(format!("margin: {}rem;", rem));
        }
        None
    }

    // Display rules
    fn display_rule(class_name: &str) -> Option<String> {
        match class_name {
            "flex" => Some("display: flex;".to_string()),
            "grid" => Some("display: grid;".to_string()),
            "block" => Some("display: block;".to_string()),
            "inline-block" => Some("display: inline-block;".to_string()),
            "inline" => Some("display: inline;".to_string()),
            _ => None,
        }
    }

    // Background color rules
    fn color_rule(class_name: &str) -> Option<String> {
        if let Some(color_name) = class_name.strip_prefix("bg-") {
            let color = Self::color_map(color_name)?;
            return Some(format!("background-color: {};", color));
        }
        None
    }

    // Text color rules
    fn text_color_rule(class_name: &str) -> Option<String> {
        if let Some(color_name) = class_name.strip_prefix("text-") {
            let color = Self::color_map(color_name)?;
            return Some(format!("color: {};", color));
        }
        None
    }

    // Font weight rules
    fn font_weight_rule(class_name: &str) -> Option<String> {
        match class_name {
            "font-thin" => Some("font-weight: 100;".to_string()),
            "font-light" => Some("font-weight: 300;".to_string()),
            "font-normal" => Some("font-weight: 400;".to_string()),
            "font-medium" => Some("font-weight: 500;".to_string()),
            "font-semibold" => Some("font-weight: 600;".to_string()),
            "font-bold" => Some("font-weight: 700;".to_string()),
            "font-extrabold" => Some("font-weight: 800;".to_string()),
            "font-black" => Some("font-weight: 900;".to_string()),
            _ => None,
        }
    }

    // Flex direction rules
    fn flex_direction_rule(class_name: &str) -> Option<String> {
        match class_name {
            "flex-row" => Some("flex-direction: row;".to_string()),
            "flex-row-reverse" => Some("flex-direction: row-reverse;".to_string()),
            "flex-col" => Some("flex-direction: column;".to_string()),
            "flex-col-reverse" => Some("flex-direction: column-reverse;".to_string()),
            _ => None,
        }
    }

    // Align items rules (flexbox align-items)
    fn align_items_rule(class_name: &str) -> Option<String> {
        match class_name {
            "items-start" => Some("align-items: flex-start;".to_string()),
            "items-center" => Some("align-items: center;".to_string()),
            "items-end" => Some("align-items: flex-end;".to_string()),
            "items-baseline" => Some("align-items: baseline;".to_string()),
            "items-stretch" => Some("align-items: stretch;".to_string()),
            _ => None,
        }
    }

    // Justify content rules (flexbox justify-content)
    fn justify_content_rule(class_name: &str) -> Option<String> {
        match class_name {
            "justify-start" => Some("justify-content: flex-start;".to_string()),
            "justify-center" => Some("justify-content: center;".to_string()),
            "justify-end" => Some("justify-content: flex-end;".to_string()),
            "justify-between" => Some("justify-content: space-between;".to_string()),
            "justify-around" => Some("justify-content: space-around;".to_string()),
            "justify-evenly" => Some("justify-content: space-evenly;".to_string()),
            _ => None,
        }
    }

    fn color_map(name: &str) -> Option<&'static str> {
        match name {
            "red" => Some("#f44336"),
            "blue" => Some("#2196f3"),
            "green" => Some("#4caf50"),
            "yellow" => Some("#ffeb3b"),
            "black" => Some("#000000"),
            "white" => Some("#ffffff"),
            "gray" => Some("#9e9e9e"),
            "purple" => Some("#9c27b0"),
            "pink" => Some("#e91e63"),
            _ => None,
        }
    }
}

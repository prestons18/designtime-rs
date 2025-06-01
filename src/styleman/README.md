# Chapter 4: The StyleMan Engine

StyleMan is a CSS generator that takes a set of classes in tailwindcss format and generates CSS for them.

## Usage
```rust
let style_manager = StyleMan::new();
style_manager.add_classes(vec!["p-4".to_string(), "m-2".to_string()]);
let css = style_manager.generate_css();
println!("{}", css);
```

## Features
- Spacing rules for p- and m-
- Display rules for flex, grid, etc.
- Color rules for text-color, bg-color, etc.

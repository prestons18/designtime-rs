use designtime_ast::Node;
use std::fs;
use std::path::Path;
use tokio;
use warp::Filter;

mod html_generator;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Watchman - Starting up...");
    
    // Create the AST nodes for our page
    let ast_nodes = create_sample_ast();
    
    // Generate HTML file with embedded AST
    html_generator::generate_html_file(&ast_nodes)?;
    
    // Copy WASM files if they exist
    copy_wasm_files()?;
    
    // Start the web server
    println!("🚀 Starting web server on http://localhost:3030");
    server::start_server().await;
    
    Ok(())
}

fn create_sample_ast() -> Vec<Node> {
    vec![
        Node::Element {
            tag_name: "div".to_string(),
            attributes: vec![
                ("class".to_string(), "container".to_string()),
            ],
            children: vec![
                Node::Element {
                    tag_name: "h1".to_string(),
                    attributes: vec![
                        ("class".to_string(), "title".to_string()),
                    ],
                    children: vec![
                        Node::Text("🔍 Watchman Demo".to_string())
                    ],
                    class_names: vec!["title".to_string()],
                },
                Node::Element {
                    tag_name: "div".to_string(),
                    attributes: vec![
                        ("class".to_string(), "greeting".to_string()),
                        ("id".to_string(), "hello".to_string()),
                    ],
                    children: vec![
                        Node::Text("Hello worl!".to_string())
                    ],
                    class_names: vec!["greeting".to_string()],
                },
                Node::Element {
                    tag_name: "p".to_string(),
                    attributes: vec![
                        ("class".to_string(), "description".to_string()),
                    ],
                    children: vec![
                        Node::Text("This page was generated using render-core and served with Watchman.".to_string())
                    ],
                    class_names: vec!["description".to_string()],
                }
            ],
            class_names: vec!["container".to_string()],
        }
    ]
}

fn copy_wasm_files() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_pkg_path = "/home/preston/designtime-rs/src/render-wasm/pkg";
    let output_dir = "dist";
    
    // Create output directory
    fs::create_dir_all(output_dir)?;
    
    // Check if WASM pkg directory exists
    if Path::new(wasm_pkg_path).exists() {
        println!("📦 Copying WASM files from {}", wasm_pkg_path);
        
        // Copy specific WASM files we need
        let files_to_copy = vec![
            "render_wasm.js",
            "render_wasm_bg.wasm", 
            "render_wasm.d.ts"
        ];
        
        for file in files_to_copy {
            let src = format!("{}/{}", wasm_pkg_path, file);
            let dst = format!("{}/{}", output_dir, file);
            
            if Path::new(&src).exists() {
                fs::copy(&src, &dst)?;
                println!("  ✅ Copied {}", file);
            } else {
                println!("  ⚠️  {} not found, skipping", file);
            }
        }
    } else {
        println!("⚠️  WASM pkg directory not found at {}", wasm_pkg_path);
        println!("   Run 'wasm-pack build' in your render-wasm directory first");
    }
    
    Ok(())
}
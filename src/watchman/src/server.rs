use warp::Filter;
use std::path::Path;
use std::fs;

pub async fn start_server() {
    // Ensure dist directory exists
    fs::create_dir_all("dist").expect("Failed to create dist directory");
    
    // Copy WASM files from pkg to dist if they don't exist
    let wasm_files = vec![
        ("src/render-wasm/pkg/render_wasm.js", "dist/render_wasm.js"),
        ("src/render-wasm/pkg/render_wasm_bg.wasm", "dist/render_wasm_bg.wasm"),
        ("src/render-wasm/pkg/render_wasm.d.ts", "dist/render_wasm.d.ts"),
    ];
    
    for (src, dst) in &wasm_files {
        if !Path::new(dst).exists() {
            if Path::new(src).exists() {
                fs::copy(src, dst).expect(&format!("Failed to copy {} to {}", src, dst));
                println!("📦 Copied {} to {}", src, dst);
            }
        }
    }
    
    // Serve static files from dist directory
    let static_files = warp::fs::dir("dist");
    
    // Proper WASM MIME types
    let wasm_files = warp::path::end()
        .and(warp::fs::file("dist/index.html"))
        .or(warp::path("render_wasm.js")
            .and(warp::fs::file("dist/render_wasm.js"))
            .with(warp::reply::with::header("content-type", "application/javascript")))
        .or(warp::path("render_wasm_bg.wasm")
            .and(warp::fs::file("dist/render_wasm_bg.wasm"))
            .with(warp::reply::with::header("content-type", "application/wasm")))
        .or(warp::path("render_wasm.d.ts")
            .and(warp::fs::file("dist/render_wasm.d.ts"))
            .with(warp::reply::with::header("content-type", "text/plain")));
    
    let routes = wasm_files.or(static_files);
    
    // Add CORS headers for development
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "DELETE"]);
    
    let routes_with_cors = routes.with(cors);
    
    println!("📡 Server starting on http://localhost:3030");
    
    // Check if WASM files exist and provide helpful information
    check_wasm_files();
    
    println!("\nPress Ctrl+C to stop the server");
    
    warp::serve(routes_with_cors)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn check_wasm_files() {
    let wasm_files = vec![
        ("dist/render_wasm.js", "JavaScript bindings"),
        ("dist/render_wasm_bg.wasm", "WebAssembly binary"),
        ("dist/render_wasm.d.ts", "TypeScript definitions"),
    ];
    
    println!("\n📋 WASM Files Status:");
    let mut all_present = true;
    
    for (file_path, description) in &wasm_files {
        if Path::new(file_path).exists() {
            println!("   ✅ {} - {}", file_path, description);
        } else {
            println!("   ❌ {} - {} (missing)", file_path, description);
            all_present = false;
        }
    }
    
    if !all_present {
        println!("\n💡 To build WASM files, run:");
        println!("   cd /home/preston/designtime-rs/src/render-wasm");
        println!("   wasm-pack build --target web --out-dir pkg");
        println!("   Then restart watchman");
    } else {
        println!("\nAll WASM files are present! We're ready to go.");
    }
}
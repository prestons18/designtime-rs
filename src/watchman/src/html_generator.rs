use designtime_ast::Node;
use serde_json;
use std::fs;
use std::error::Error;

const ICONS_HTML: &str = r#"
 <a href='#' class='icon' title='Design'>🎨</a>
 <a href='#' class='icon' title='Edit'>📝</a>
 <a href='#' class='icon' title='Run'>⚡</a>
"#;

pub fn generate_html_file(ast_nodes: &[Node]) -> Result<(), Box<dyn Error>> {
    println!("📝 Generating HTML file...");
    
    // Serialize AST nodes to JSON (compact for embedding in JS)
    let ast_json_raw = serde_json::to_string(ast_nodes)?;
    
    // Also create a pretty version for readable <pre>
    let ast_json_pretty = serde_json::to_string_pretty(ast_nodes)?
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('&', "&amp;");
    
    // Build the HTML content
    let html_template = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DesignTime</title>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=Dancing+Script:wght@700&display=swap');
        
        body {{
            margin: 0;
            padding: 0;
            min-height: 100vh;
            background: #ffffff;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            color: #333;
            padding-bottom: 100px; /* Space for floating button */
        }}
        
        .floating-circle {{
            position: fixed;
            bottom: 2rem;
            right: 2rem;
            width: 60px;
            height: 60px;
            background: #1a1a1a;
            border-radius: 50%;
            display: flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
            transition: all 0.3s ease;
            box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
            z-index: 1000;
        }}
        
        .floating-circle:hover {{
            transform: scale(1.1);
            background: #2a2a2a;
            box-shadow: 0 6px 25px rgba(0, 0, 0, 0.25);
        }}
        
        .floating-circle .letter {{
            font-family: 'Dancing Script', cursive;
            font-size: 2rem;
            color: #fff;
            user-select: none;
        }}
        
        .hover-icons {{
            position: absolute;
            bottom: 80px;
            right: 10px;
            display: flex;
            flex-direction: column;
            gap: 0.5rem;
            opacity: 0;
            transform: translateY(20px);
            transition: all 0.3s ease;
            pointer-events: none;
        }}
        
        .floating-circle:hover .hover-icons {{
            opacity: 1;
            transform: translateY(0);
            pointer-events: all;
        }}
        
        .icon {{
            width: 40px;
            height: 40px;
            background: rgba(26, 26, 26, 0.9);
            backdrop-filter: blur(10px);
            border-radius: 50%;
            display: flex;
            justify-content: center;
            align-items: center;
            color: #fff;
            text-decoration: none;
            transition: all 0.2s ease;
            font-size: 1.2rem;
            border: 1px solid rgba(255, 255, 255, 0.1);
        }}
        
        .icon:hover {{
            transform: scale(1.1);
            background: rgba(42, 42, 42, 0.9);
        }}
        
        #root {{
            width: 100%;
            min-height: 100vh;
            position: relative;
        }}
        
        .debug {{
            position: fixed;
            top: 1rem;
            right: 1rem;
            background: rgba(0, 0, 0, 0.9);
            color: #fff;
            padding: 1rem;
            border-radius: 8px;
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            font-size: 0.8rem;
            max-width: 400px;
            max-height: 80vh;
            overflow: auto;
            display: none;
            white-space: pre-wrap;
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.1);
            z-index: 999;
        }}
        
        /* Ensure content is properly spaced */
        .content-wrapper {{
            padding: 2rem;
            max-width: 1200px;
            margin: 0 auto;
        }}
        
        /* Loading state */
        .loading {{
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 50vh;
            font-size: 1.2rem;
            color: #666;
        }}
        
        .loading::after {{
            content: '';
            width: 20px;
            height: 20px;
            border: 2px solid #ddd;
            border-top: 2px solid #333;
            border-radius: 50%;
            animation: spin 1s linear infinite;
            margin-left: 10px;
        }}
        
        @keyframes spin {{
            0% {{ transform: rotate(0deg); }}
            100% {{ transform: rotate(360deg); }}
        }}
    </style>
</head>
<body>
    <div id="root">
        <div class="loading">Loading DesignTime</div>
    </div>
    
    <div class="floating-circle">
        <div class="letter">D</div>
        <div class="hover-icons">
{icons}
        </div>
    </div>
    
    <div id="debug" class="debug">
        <strong>AST JSON:</strong>
        <pre>{{}}</pre>
    </div>
    
    <script type="module">
        // Store the AST data
        const astData = {ast_json_raw};
        
        async function initWasm() {{
            try {{
                console.log('🔄 Initializing WASM...');
                const wasmModule = await import('./render_wasm.js');
                await wasmModule.default();
                console.log('✅ WASM loaded successfully');
                
                // Clear loading state
                const root = document.getElementById('root');
                root.innerHTML = '';
                
                console.log('🎨 Rendering AST:', astData);
                
                // Set the target container for WASM rendering
                if (wasmModule.setRenderTarget) {{
                    wasmModule.setRenderTarget('#root');
                }}
                
                wasmModule.renderFromJson(JSON.stringify(astData));
                console.log('✅ Rendering completed');
                
                // If WASM still renders to body, move content to root
                setTimeout(() => {{
                    const bodyChildren = Array.from(document.body.children);
                    const unwantedElements = bodyChildren.filter(el => 
                        !el.id || !['root', 'debug'].includes(el.id) && 
                        !el.classList.contains('floating-circle')
                    );
                    
                    unwantedElements.forEach(el => {{
                        root.appendChild(el);
                    }});
                }}, 100);
                
            }} catch (error) {{
                console.error('❌ WASM initialization failed:', error);
                
                // Show error state
                const root = document.getElementById('root');
                root.innerHTML = `
                    <div class="content-wrapper">
                        <h1>DesignTime - Error</h1>
                        <p>Failed to initialize WASM module. Please check the console for details.</p>
                        <p><strong>Error:</strong> ${{error.message}}</p>
                    </div>
                `;
                
                // Show debug panel
                const debugPanel = document.getElementById('debug');
                debugPanel.style.display = 'block';
                debugPanel.querySelector('pre').textContent = `Error: ${{error.message}}\n\nStack: ${{error.stack}}`;
            }}
        }}
        
        // Initialize
        initWasm();
        
        // Debug toggle
        document.addEventListener('keydown', (e) => {{
            if (e.key === 'd' || e.key === 'D') {{
                const debug = document.getElementById('debug');
                debug.style.display = debug.style.display === 'none' ? 'block' : 'none';
            }}
        }});
        
        // Handle icon clicks
        document.querySelectorAll('.icon').forEach(icon => {{
            icon.addEventListener('click', (e) => {{
                e.preventDefault();
                const title = icon.getAttribute('title');
                console.log(`${{title}} clicked`);
                // Add your icon-specific functionality here
            }});
        }});
    </script>
</body>
</html>"#, icons = ICONS_HTML, ast_json_raw = ast_json_raw);
    
    let html_content = html_template.replace("{{}}", &ast_json_pretty);
    
    // Ensure dist directory exists
    fs::create_dir_all("dist")?;
    
    // Write the HTML file
    fs::write("dist/index.html", html_content)?;
    
    println!("✅ HTML file generated at dist/index.html");
    Ok(())
}
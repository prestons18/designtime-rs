use std::{
    path::PathBuf,
    time::Duration,
};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::broadcast;
use warp::Filter;
use crate::engine::runtime::Runtime;

pub async fn watchman(mut runtime: Runtime) -> anyhow::Result<()> {
    // Channel to broadcast reload events to SSE clients
    // Capacity 16 reload events before dropping old messages
    let (reload_tx, _) = broadcast::channel::<()>(16);

    // Clone for file watcher closure
    let reload_tx_watcher = reload_tx.clone();

    // Spawn blocking thread for file watcher
    tokio::task::spawn_blocking(move || {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher: RecommendedWatcher =
            Watcher::new(tx, Config::default().with_poll_interval(Duration::from_secs(1))).unwrap();

        // Watch for .dts files in src/ and examples/ directories
        for dir in &["src/", "./src/examples/"] {
            if let Err(e) = watcher.watch(&PathBuf::from(dir), RecursiveMode::Recursive) {
                eprintln!("Failed to watch directory {}: {}", dir, e);
            }
        }

        println!("Watching for changes to .dts files in src/ and examples/...");
        println!("Server running at http://localhost:3000");

        for res in rx {
            match res {
                Ok(event) => {
                    // Only process Modify events on files, ignore directory access
                    if let notify::EventKind::Modify(_) = event.kind {
                        if let Some(path) = event.paths.get(0) {
                            // Skip directories
                            if path.is_dir() {
                                continue;
                            }

                            // Only process .dts files
                            if let Some(ext) = path.extension() {
                                if ext == "dts" {
                                    println!("Processing .dts file: {}", path.display());
                                    
                                    match std::fs::read_to_string(path) {
                                        Ok(source) => {
                                            match runtime.process_source(&source) {
                                                Ok(css) => println!("Generated CSS from {}:\n{}", path.display(), css),
                                                Err(e) => eprintln!("Runtime error in {}: {}", path.display(), e),
                                            }
                                        }
                                        Err(e) => {
                                            if path.exists() {
                                                eprintln!("Failed to read {}: {}", path.display(), e);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Notify SSE clients for reload
                    let _ = reload_tx_watcher.send(());
                }
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        }
    });

    // Serve the main page at "/"
    let index_route = warp::path::end().map(|| {
        warp::reply::html(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8" />
<title>Watchman (warp)</title>
<style>body { font-family: sans-serif; padding: 2rem; }</style>
</head>
<body>
<h1>Watchman running!</h1>
<p>Change a file in src/examples or designtime.json to trigger a reload.</p>
<script>
  const evtSource = new EventSource('/reload');
  evtSource.onmessage = function(event) {
    if (event.data === 'reload') {
      console.log('Reload signal received, reloading page...');
      location.reload();
    }
  };
</script>
</body>
</html>"#)
    });

    // SSE endpoint for reload notifications
    let reload_route = warp::path("reload").and(warp::get()).map(move || {
        // Convert broadcast Receiver into a Stream of SSE messages
        let mut rx = reload_tx.subscribe();

        // Create a stream of SSE text messages
        let stream = async_stream::stream! {
            loop {
                match rx.recv().await {
                    Ok(_) => {
                        // Send reload event
                        yield Ok::<_, std::convert::Infallible>(warp::sse::Event::default().data("reload"));
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // Missed messages, just continue
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        };

        warp::sse::reply(warp::sse::keep_alive().stream(stream))
    });

    // Compose routes
    let routes = index_route.or(reload_route);

    println!("Watchman server running at http://localhost:3000");

    // Start warp server
    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;

    Ok(())
}

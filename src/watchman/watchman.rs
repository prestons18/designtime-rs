use std::{path::PathBuf, time::Duration};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::{broadcast, mpsc};
use warp::Filter;
use serde_json::json;
use crate::render::RenderLib;

pub struct Watchman {
    render_lib: RenderLib,
    reload_tx: broadcast::Sender<()>,
    error_tx: mpsc::UnboundedSender<String>,
    error_rx: mpsc::UnboundedReceiver<String>,
}

impl Watchman {
    pub fn new(render_lib: RenderLib) -> Self {
        let (reload_tx, _) = broadcast::channel(16);
        let (error_tx, error_rx) = mpsc::unbounded_channel();
        Self {
            render_lib,
            reload_tx,
            error_tx,
            error_rx,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let reload_tx_watcher = self.reload_tx.clone();
        let error_tx_watcher = self.error_tx.clone();

        // Channel to send file changes from watcher thread to async task
        let (file_change_tx, mut file_change_rx) = mpsc::unbounded_channel::<PathBuf>();

        // Spawn blocking watcher thread
        std::thread::Builder::new()
            .name("file-watcher".into())
            .spawn(move || {
                let (tx, rx) = std::sync::mpsc::channel();
                let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default().with_poll_interval(Duration::from_secs(1)))
                    .expect("Failed to create watcher");

                for dir in &["src/", "./src/examples/"] {
                    if let Err(e) = watcher.watch(&PathBuf::from(dir), RecursiveMode::Recursive) {
                        let _ = error_tx_watcher.send(format!("Failed to watch '{}': {}", dir, e));
                    }
                }

                println!("Watching for .dts changes... Server at http://localhost:3000");

                for res in rx {
                    match res {
                        Ok(event) => {
                            if matches!(event.kind, notify::EventKind::Modify(_)) {
                                if let Some(path) = event.paths.get(0) {
                                    if path.extension().map_or(false, |ext| ext == "dts") {
                                        if file_change_tx.send(path.clone()).is_err() {
                                            eprintln!("File change receiver dropped, exiting watcher.");
                                            break;
                                        }
                                        let _ = reload_tx_watcher.send(());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let _ = error_tx_watcher.send(format!("Watcher error: {:?}", e));
                        }
                    }
                }

                println!("Watcher thread exiting.");
            })?;

        let mut render_lib = self.render_lib;
        let error_tx_clone = self.error_tx.clone();
        tokio::spawn(async move {
            while let Some(path) = file_change_rx.recv().await {
                println!("Changed file: {}", path.display());
                match std::fs::read_to_string(&path) {
                    Ok(source) => {
                        if let Err(e) = render_lib.process_source(&source) {
                            eprintln!("Error processing {}: {}", path.display(), e);
                        }
                    }
                    Err(e) => {
                        let _ = error_tx_clone.send(format!("Failed to read {}: {}", path.display(), e));
                    }
                }
            }
            println!("File processor task ended.");
        });

        let mut error_rx = self.error_rx;
        let error_sender = self.error_tx.clone();
        tokio::spawn(async move {
            while let Some(err) = error_rx.recv().await {
                eprintln!("[Watchman Error] {}", err);
                let _ = error_sender.send(err);
            }
            println!("Error handler task ended.");
        });

        // Helper to list .dts files
        let list_files = || -> Vec<serde_json::Value> {
            let mut files = Vec::new();
            for dir in &["src/", "./src/examples/"] {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "dts") {
                            if let Some(name) = path.file_name() {
                                files.push(json!({
                                    "name": name.to_string_lossy(),
                                    "path": path.to_string_lossy()
                                }));
                            }
                        }
                    }
                }
            }
            files
        };

        let reload_tx_filter = self.reload_tx.clone();

        let index_route = warp::path::end().map(|| {
            warp::reply::html(r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="UTF-8"><title>DesignTime Watchman</title></head>
<body>
<h1>DesignTime Watchman</h1>
<div id="status">Watching for changes...</div>
<div id="files"></div>
<pre id="output">Select a file</pre>
<div id="error" style="color: red; display:none;"></div>
<script>
const evt = new EventSource('/reload');
const status = document.getElementById('status');
const filesEl = document.getElementById('files');
const output = document.getElementById('output');
const errorEl = document.getElementById('error');

evt.onmessage = e => {
    if (e.data === 'reload') {
        status.textContent = 'Changes detected, refreshing files...';
        loadFiles();
    } else if (e.data.startsWith('ERROR:')) {
        errorEl.textContent = e.data.slice(6);
        errorEl.style.display = 'block';
        status.textContent = 'Error detected';
    }
};
evt.onerror = () => {
    status.textContent = 'Connection lost, retrying...';
};
evt.onopen = () => {
    status.textContent = 'Connected, watching for changes...';
};

async function loadFiles() {
    try {
        const res = await fetch('/api/files');
        const files = await res.json();
        errorEl.style.display = 'none';
        if (files.length === 0) {
            filesEl.textContent = 'No .dts files found';
            output.textContent = '';
            return;
        }
        filesEl.innerHTML = files.map(f => 
            `<div style="cursor:pointer; padding:4px; border-bottom:1px solid #ccc;" onclick="loadFile('${f.path}')">${f.name}</div>`
        ).join('');
        output.textContent = 'Select a file';
    } catch {
        filesEl.textContent = 'Failed to load files';
    }
}
async function loadFile(path) {
    try {
        const res = await fetch('/api/file?path=' + encodeURIComponent(path));
        const data = await res.json();
        if (data.error) throw new Error(data.error);
        output.textContent = data.content;
        errorEl.style.display = 'none';
        status.textContent = `Loaded ${path.split('/').pop()}`;
    } catch (err) {
        errorEl.textContent = err.message;
        errorEl.style.display = 'block';
        status.textContent = 'Error loading file';
    }
}
window.loadFile = loadFile;
loadFiles();
</script>
</body>
</html>"#)
        });

        let reload_route = warp::path("reload")
            .and(warp::get())
            .map(move || {
                let mut rx = reload_tx_filter.subscribe();
                let stream = async_stream::stream! {
                    loop {
                        match rx.recv().await {
                            Ok(_) => yield Ok::<_, std::convert::Infallible>(warp::sse::Event::default().data("reload")),
                            Err(broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(broadcast::error::RecvError::Closed) => break,
                        }
                    }
                };
                warp::sse::reply(warp::sse::keep_alive().stream(stream))
            });

        let files_route = warp::path!("api" / "files")
            .and(warp::get())
            .map(move || warp::reply::json(&list_files()));

        let file_route = warp::path!("api" / "file")
            .and(warp::get())
            .and(warp::query::<std::collections::HashMap<String, String>>())
            .map(|params: std::collections::HashMap<String, String>| {
                if let Some(path) = params.get("path") {
                    match std::fs::read_to_string(path) {
                        Ok(content) => warp::reply::json(&json!({ "content": content })),
                        Err(e) => warp::reply::json(&json!({ "error": format!("Failed to read file: {}", e) })),
                    }
                } else {
                    warp::reply::json(&json!({ "error": "Missing 'path' parameter" }))
                }
            });

        let routes = index_route.or(reload_route).or(files_route).or(file_route);

        println!("Watchman server running at http://localhost:3000");
        warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;

        Ok(())
    }
}

# Chapter 5: The Watchman Engine

Watchman is a file watcher that watches for changes to .dts files and reloads the server when a change is detected.

## Usage
```rust
let watchman = Watchman::new();
watchman.watch();
```

## Features
- Starts a warp server on port 3000
- Watches for changes to .dts files
- Reloads the server when a change is detected

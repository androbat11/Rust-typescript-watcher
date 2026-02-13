# Rust TypeScript Watcher

A lightweight filesystem watcher built in Rust that monitors TypeScript file changes in real time. It detects modifications to `.ts` files and reports where the change occurred and what happened.

## Features

- Watch for TypeScript file changes (create, modify, delete)
- Report the file path and type of change detected
- Cross-platform support (Linux, macOS, Windows, BSD)

## Project Structure

```
watcher/src/
├── main.rs        # Entry point
├── model.rs       # Core types (WatcherKind, etc.)
├── adapters.rs    # Watcher trait definition
└── watch.rs       # Watch logic
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

### Build & Run

```sh
cd watcher
cargo build
cargo run
```

## How It Works

The watcher observes a given directory and listens for filesystem events on `.ts` files. When a change is detected, it reports:

- **What** happened — file created, modified, or deleted
- **Where** it happened — the full path to the affected file

The `Watcher` trait in `adapters.rs` defines the interface, and `WatcherKind` in `model.rs` determines which platform-specific backend is used (inotify on Linux, FSEvents on macOS, ReadDirectoryChangesW on Windows, kqueue on BSD).

## License

This project is unlicensed. Feel free to add a license of your choice.
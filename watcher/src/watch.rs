use notify::{Config, EventHandler, EventKind, INotifyWatcher, RecursiveMode, Result, Watcher};
use notify::event::{ModifyKind, CreateKind, RemoveKind, AccessKind};
use colored::Colorize;
use std::path::Path;

use crate::ports::FileWatcher;

pub struct TypeScriptWatcher {
    inner: notify::RecommendedWatcher,
}

pub fn is_typescript(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => ext == "ts" || ext == "tsx",
        None => false,
    }
}

pub fn format_event_kind(kind: &EventKind) -> colored::ColoredString {
    match kind {
        EventKind::Create(CreateKind::File) => "FILE CREATED".green(),
        EventKind::Create(CreateKind::Folder) => "FOLDER CREATED".green(),
        EventKind::Create(_) => "CREATED".green(),
        EventKind::Modify(ModifyKind::Data(_)) => "CONTENT MODIFIED".yellow(),
        EventKind::Modify(ModifyKind::Name(_)) => "RENAMED".cyan(),
        EventKind::Modify(ModifyKind::Metadata(_)) => "METADATA CHANGED".purple(),
        EventKind::Modify(_) => "MODIFIED".yellow(),
        EventKind::Remove(RemoveKind::File) => "FILE DELETED".red(),
        EventKind::Remove(RemoveKind::Folder) => "FOLDER DELETED".red(),
        EventKind::Remove(_) => "DELETED".red(),
        EventKind::Access(AccessKind::Read) => "READ".blue(),
        EventKind::Access(AccessKind::Open(_)) => "OPENED".blue(),
        EventKind::Access(AccessKind::Close(_)) => "CLOSED".blue(),
        EventKind::Access(_) => "ACCESSED".blue(),
        _ => "EVENT".white(),
    }
}

pub fn handle_event(result: std::result::Result<notify::Event, notify::Error>) {
    match result {
        Ok(event) => {
            let ts_paths: Vec<_> = event.paths.iter()
                .filter(|p| is_typescript(p))
                .collect();

            if !ts_paths.is_empty() {
                let label = format_event_kind(&event.kind);
                let home = std::env::var("HOME").unwrap_or_default();
                let home_path = std::path::Path::new(&home).join("Documents").join("projects");
                for path in &ts_paths {
                    let display_path = path.strip_prefix(&home_path).unwrap_or(path);
                    println!("[{}] {}", label, display_path.display().to_string().bold());
                }
            }
        }
        Err(err) => eprintln!("{} {:?}", "Watch error:".red(), err),
    }
}

impl FileWatcher for TypeScriptWatcher {
    fn new<F: EventHandler>(event_handler: F, config: Config) -> Result<Self>
    where
        Self: Sized,
    {
        let inner: INotifyWatcher = match notify::RecommendedWatcher::new(event_handler, config) {
            Ok(watch) => watch,
            Err(error) => {
                eprintln!("Failed to create TypeScript watcher: {error}");
                return Err(error);
            }
        };

        Ok(TypeScriptWatcher { inner })
    }

    fn watch(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        self.inner.watch(path, recursive_mode)
    }

    fn kind() -> notify::WatcherKind
    where
        Self: Sized,
    {
        notify::RecommendedWatcher::kind()
    }
}

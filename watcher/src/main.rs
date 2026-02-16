use std::path::Path;
use notify::{Config, RecursiveMode};
use colored::Colorize;

pub mod ports;
pub mod watch;

use watch::TypeScriptWatcher;
use ports::FileWatcher;

fn main() {
    let config = Config::default();

    let mut watcher = match TypeScriptWatcher::new(watch::handle_event, config) {
        Ok(w) => w,
        Err(err) => {
            eprintln!("{} {err}", "Failed to create watcher:".red());
            return;
        }
    };

    match watcher.watch(Path::new("."), RecursiveMode::Recursive) {
        Ok(_) => println!("{}", "Watching for TypeScript changes...".green().bold()),
        Err(err) => {
            eprintln!("{} {}", "Failed to watch:".red(), err);
            return;
        }
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

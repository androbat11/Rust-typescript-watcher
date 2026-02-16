use notify::{EventHandler, RecursiveMode, Result, Config, WatcherKind};
use std::path::Path;

pub trait FileWatcher {
    fn new<F: EventHandler>(event_handler: F, config: Config) -> Result<Self>
        where Self: Sized;

    fn watch(
        &mut self,
        path: &Path,
        recursive_mode: RecursiveMode,
    ) -> Result<()>;

    fn kind() -> WatcherKind
        where Self: Sized;
}

use crate::model::{WatcherKind};
use notify::{EventHandler, RecursiveMode, Result};
use std::path::{Path};
// How is supposed to look the model of a watcher?
pub trait Watcher {
    // EventHandler is an enum that represents all the probable
    // type of event
    fn new<F: EventHandler>(event_handler: F, config: Config) -> Result<Self>
       where Self: Sized;
    fn watch(
        &mut self,
        path: &Path,
        recursive_mode: RecursiveMode,
    ) -> Result<()>;
    // fn unwatch(&mut self, path: &Path) -> Result<()>;
    // Still don't know what's kind and what it does.
    // https://docs.rs/notify/latest/notify/enum.WatcherKind.html
    fn kind() -> WatcherKind
       where Self: Sized;

    // configure function just sets the configuration.
    // _option: Config -> must only take the 
    fn configure(&mut self, _option: Config) -> Result<bool> {}
}
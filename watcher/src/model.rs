#[non_exhaustive]
pub enum WatcherKind {
   /// Linux backend using the inotify API for monitoring filesystem events.
   Inotify,
   /// macOS backend using the FSEvents API for monitoring filesystem events.
   Fsevent,
   /// BSD (FreeBSD, OpenBSD, NetBSD) and macOS backend using the kqueue API.
   Kqueue,
   /// Windows backend using the ReadDirectoryChangesW API.
   ReadDirectoryChangesWatcher,
   /// No-op backend that does nothing. Useful for testing or unsupported platforms.
   NullWatcher,
}
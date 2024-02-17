use std::ffi::OsStr;
use std::path::Path;

use inotify::{Event, EventMask};

pub enum FileAction {
    Created,
    Modified,
    Moved,
    Deleted,
}

impl FileAction {
    pub fn from_event(event: &Event<&OsStr>, filepath: &str) -> Option<FileAction> {
        if event.mask.contains(EventMask::MODIFY) {
            return Some(FileAction::Modified);
        }
        let event_filename = event.name?;
        let filename = Path::new(filepath).file_name()?;
        if event_filename != filename {
            return None;
        }
        if event.mask.contains(EventMask::CREATE) {
            return Some(FileAction::Created);
        }
        if event.mask.contains(EventMask::MOVED_FROM) {
            return Some(FileAction::Moved);
        }
        if event.mask.contains(EventMask::DELETE) {
            return Some(FileAction::Deleted);
        }
        return None;
    }
}

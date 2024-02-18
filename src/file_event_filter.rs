use std::ffi::OsStr;

use inotify::{Event, EventMask};

pub struct FileEventFilter {
    filename: String,
}

pub enum FileAction {
    Created,
    Modified,
    Moved,
    Deleted,
}

impl FileEventFilter {
    pub fn new(filename: &str) -> FileEventFilter {
        return FileEventFilter {
            filename: String::from(filename),
        };
    }

    pub fn get_action(&self, event: &Event<&OsStr>) -> Option<FileAction> {
        if event.mask.contains(EventMask::MODIFY) {
            return Some(FileAction::Modified);
        }
        let event_filename = event.name?.to_str()?;
        if event_filename != self.filename {
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

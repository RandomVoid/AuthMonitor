use std::error::Error;
use std::io::ErrorKind;

use inotify::{Inotify, WatchMask};

use crate::auth_file_reader::AuthFileReader;
use crate::file_event_filter::{FileAction, FileEventFilter};
use crate::file_path::FilePath;

const EVENT_BUFFER_SIZE: usize = 1024;
const READER_BUFFER_SIZE: usize = 1024;

pub struct AuthFileWatcher {
    filepath: String,
    inotify: Inotify,
    event_buffer: [u8; EVENT_BUFFER_SIZE],
    reader: Option<AuthFileReader>,
    event_filter: FileEventFilter,
}

impl AuthFileWatcher {
    pub fn new(filepath: &str) -> Result<AuthFileWatcher, Box<dyn Error>> {
        let FilePath {
            directory,
            filename,
        } = FilePath::from(filepath)?;
        let inotify = Inotify::init()?;
        let directory_watch_mask = WatchMask::CREATE | WatchMask::DELETE | WatchMask::MOVED_FROM;
        inotify.watches().add(directory, directory_watch_mask)?;
        let mut auth_file_watcher = AuthFileWatcher {
            filepath: String::from(filepath),
            inotify,
            event_buffer: [0u8; EVENT_BUFFER_SIZE],
            reader: None,
            event_filter: FileEventFilter::new(&filename),
        };
        auth_file_watcher.open_existing_file();
        return Ok(auth_file_watcher);
    }

    fn open_existing_file(&mut self) {
        self.open_file();
        match &mut self.reader {
            Some(reader) => {
                reader.seek_to_end().unwrap_or_else(|error| {
                    eprintln!("Error seeking to end of file: {}", error);
                });
            }
            None => {}
        }
    }

    fn open_file(&mut self) {
        let reader = match AuthFileReader::new(&self.filepath, READER_BUFFER_SIZE) {
            Ok(reader) => reader,
            Err(error) => {
                eprintln!("Unable to open monitored file: {}", error);
                return;
            }
        };
        match self
            .inotify
            .watches()
            .add(&self.filepath, WatchMask::MODIFY)
        {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error adding file watch: {}", error);
                return;
            }
        }
        println!("Monitored file opened");
        self.reader = Some(reader);
    }

    pub fn update(&mut self, parse_line: impl FnMut(&String)) {
        let events = match self.inotify.read_events(&mut self.event_buffer) {
            Ok(events) => events,
            Err(error) => {
                if error.kind() != ErrorKind::WouldBlock {
                    eprintln!("Failed to read inotify events: {}", error);
                }
                return;
            }
        };

        let mut file_modified = false;

        for event in events {
            let action = self.event_filter.get_action(&event);
            if action.is_none() {
                continue;
            }
            match action.unwrap() {
                FileAction::Created => {
                    println!("New monitored file has been created");
                    self.open_new_file();
                    file_modified = true;
                    break;
                }
                FileAction::Modified => {
                    file_modified = true;
                }
                FileAction::Moved | FileAction::Deleted => {
                    println!("Monitored file has been deleted or moved");
                    self.reader = None;
                    continue;
                }
            };
        }

        if !file_modified {
            return;
        }

        match &mut self.reader {
            Some(reader) => {
                reader.read_new_lines(parse_line);
            }
            None => {}
        };
    }

    fn open_new_file(&mut self) {
        self.open_file();
    }
}

#[cfg(test)]
#[path = "./auth_file_watcher.test.rs"]
mod test;

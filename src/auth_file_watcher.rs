use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Seek, SeekFrom};

use inotify::{EventMask, Inotify, WatchMask};

pub struct AuthFileWatcher {
    inotify: Inotify,
    reader: BufReader<File>,
    event_buffer: [u8; 1024],
    file_content_buffer: String,
}

impl AuthFileWatcher {
    pub fn new(filepath: &str) -> Result<AuthFileWatcher, Box<dyn Error>> {
        let inotify = Inotify::init()?;
        let mut file = File::open(filepath)?;
        file.seek(SeekFrom::End(0))?;
        inotify.watches().add(filepath, WatchMask::MODIFY)?;
        return Ok(AuthFileWatcher {
            inotify,
            reader: BufReader::new(file),
            event_buffer: [0u8; 1024],
            file_content_buffer: String::with_capacity(1024),
        });
    }

    pub fn update<F: FnMut(&String)>(&mut self, mut parse_line: F) {
        let events = match self.inotify.read_events(&mut self.event_buffer) {
            Ok(events) => events,
            Err(error) => {
                if error.kind() != ErrorKind::WouldBlock {
                    eprintln!("Failed to read inotify events: {}", error);
                }
                return;
            }
        };
        for event in events {
            println!("Event: {:?}", event);
            if event.mask & EventMask::MODIFY == EventMask::MODIFY {
                loop {
                    self.file_content_buffer.clear();
                    let bytes_read = match self.reader.read_line(&mut self.file_content_buffer) {
                        Ok(bytes_read) => bytes_read,
                        Err(error) => {
                            eprintln!("Error reading file: {}", error);
                            return;
                        }
                    };
                    if bytes_read == 0 {
                        break;
                    }
                    print!("Line added: {}", self.file_content_buffer);
                    parse_line(&self.file_content_buffer);
                }
            }
        }
    }
}

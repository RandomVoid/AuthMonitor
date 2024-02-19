use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

pub struct AuthFileReader {
    reader: BufReader<File>,
    file_content_buffer: String,
}

impl AuthFileReader {
    pub fn new(filepath: &str, buffer_size: usize) -> Result<AuthFileReader, Box<dyn Error>> {
        let file = File::open(filepath)?;
        return Ok(AuthFileReader {
            reader: BufReader::new(file),
            file_content_buffer: String::with_capacity(buffer_size),
        });
    }

    pub fn seek_to_end(&mut self) -> Result<(), Box<dyn Error>> {
        self.reader.seek(SeekFrom::End(0))?;
        return Ok(());
    }

    pub fn read_new_lines<F: FnMut(&String)>(&mut self, mut parse_line: F) {
        loop {
            self.file_content_buffer.clear();
            let bytes_read = self
                .reader
                .read_line(&mut self.file_content_buffer)
                .unwrap_or_else(|error| {
                    eprintln!("Error reading file: {}", error);
                    return 0;
                });
            if bytes_read > 0 {
                parse_line(&self.file_content_buffer);
                continue;
            }
            if !self.is_file_has_been_truncated() {
                break;
            }
            match self.reader.seek(SeekFrom::Start(0)) {
                Ok(position) => println!("Resetting position in file to {}", position),
                Err(error) => {
                    eprintln!("Error resetting position in file: {}", error);
                    break;
                }
            }
        }
    }

    fn is_file_has_been_truncated(&self) -> bool {
        let length = match self.reader.get_ref().metadata() {
            Ok(metadata) => metadata.len(),
            Err(error) => {
                eprintln!("Error getting file metadata: {}", error);
                return false;
            }
        };
        let position = match self.reader.get_ref().stream_position() {
            Ok(position) => position,
            Err(error) => {
                eprintln!("Error getting current position in file: {}", error);
                return false;
            }
        };
        return position > length;
    }
}

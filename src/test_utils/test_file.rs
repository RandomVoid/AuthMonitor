use std::env::temp_dir;
use std::fs::{remove_file, rename, File};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::Local;

pub struct TestFile {
    path: String,
    file: File,
}

impl TestFile {
    pub fn with_unique_name() -> TestFile {
        let filename = format!(
            "auth-monitor-test-{}-{}.log",
            Self::next_id(),
            Local::now().timestamp_micros()
        );
        let path_buffer = temp_dir().join(filename);
        let path = path_buffer.to_str().expect("Error creating file path");
        return Self::new(path);
    }

    fn next_id() -> usize {
        static ID: AtomicUsize = AtomicUsize::new(1);
        return ID.fetch_add(1, Ordering::Relaxed);
    }

    pub fn new(path: &str) -> TestFile {
        println!("Creating test file: {}", path);
        return TestFile {
            path: String::from(path),
            file: File::create(path).expect("Error creating test file"),
        };
    }

    pub fn path(&self) -> &str {
        return &self.path;
    }

    pub fn create(&mut self) {
        println!("Creating test file: {}", &self.path);
        self.file = File::create(&self.path).expect("Error creating test file");
    }

    pub fn write(&mut self, message: &str) {
        let bytes_to_add = message.as_bytes();
        let bytes_written = self
            .file
            .write(bytes_to_add)
            .expect("Error writing to file");
        assert_eq!(bytes_written, bytes_to_add.len());
    }

    pub fn truncate(&mut self) {
        println!("Truncating test file: {}", self.path);
        self.file.set_len(0).expect("Error truncating file");
    }

    pub fn remove(&mut self) {
        println!("Removing test file: {}", self.path);
        remove_file(&self.path).expect("Unable to remove test file");
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        self.remove();
    }
}

pub fn rename_file(filepath: &str, new_filename: &str) {
    println!("Renaming test file {} to {}", filepath, new_filename);
    let new_path = Path::new(&filepath)
        .parent()
        .expect("Unable to get directory")
        .join(new_filename);
    let new_filepath = new_path.to_str().expect("Unable to build file path");
    rename(filepath, new_filepath).expect("Unable to rename test file");
}

pub fn create_log_line(message: &str) -> String {
    let date_time = Local::now().format("%+");
    return format!("{} {}\n", date_time, message);
}

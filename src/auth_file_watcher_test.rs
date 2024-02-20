use std::env::temp_dir;
use std::fs::File;
use std::fs::{remove_file, rename};
use std::io::Write;
use std::path::Path;

use chrono::Local;

use crate::auth_file_watcher::AuthFileWatcher;

const AUTH_FAILED_MESSAGES: [&str; 6] = [
    "workstation sudo: pam_unix(sudo:auth): authentication failure; logname=john uid=1000 euid=0 tty=/dev/pts/7 ruser=john rhost=  user=john",
    "workstation kscreenlocker_greet: pam_unix(kde:auth): authentication failure; logname= uid=1000 euid=1000 tty= ruser= rhost=  user=john",
    "workstation dbus-daemon[1988]: [system] Failed to activate service 'org.bluez': timed out (service_start_timeout=25000ms)",
    "workstation CRON[9419]: pam_unix(cron:session): session opened for user root(uid=0) by (uid=0)",
    "workstation CRON[9419]: pam_unix(cron:session): session closed for user root",
    "workstation PackageKit: uid 1000 is trying to obtain org.freedesktop.packagekit.system-sources-refresh auth (only_trusted:0)",
];

struct TestFile {
    pub filepath: String,
    file: File,
}

impl TestFile {
    pub fn new(prefix: &str) -> TestFile {
        let filename = format!("{}-{}.log", prefix, Local::now().timestamp_micros());
        let filepath_buffer = temp_dir().join(filename);
        let filepath = filepath_buffer.to_str().expect("Error creating filepath");
        println!("Creating test file: {}", filepath);
        return TestFile {
            filepath: String::from(filepath),
            file: File::create(filepath).expect("Error creating test file"),
        };
    }

    pub fn create(&mut self) {
        println!("Creating test file: {}", &self.filepath);
        self.file = File::create(&self.filepath).expect("Error creating test file");
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
        println!("Truncating test file: {}", self.filepath);
        self.file.set_len(0).expect("Error truncating file");
    }

    pub fn remove(&mut self) {
        println!("Removing test file: {}", self.filepath);
        remove_file(&self.filepath).expect("Unable to remove test file");
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        self.remove();
    }
}

#[test]
fn when_monitored_file_does_not_exist_then_new_does_not_return_error() {
    let filepath_buffer = temp_dir().join("auth-monitor-non-existing-file.log");
    let filepath = filepath_buffer.to_str().expect("Error creating filepath");
    AuthFileWatcher::new(filepath).expect("Error creating AuthFileWatcher");
}

#[test]
fn when_new_line_is_added_to_file_then_update_callback_is_called() {
    let mut file = TestFile::new("auth-monitor-test");
    let mut auth_file_watcher =
        AuthFileWatcher::new(&file.filepath).expect("Error creating AuthFileWatcher");
    expect_no_update_callback_call(&mut auth_file_watcher);
    expect_update_callback_is_called_when_file_is_modified(&mut file, &mut auth_file_watcher);
}

fn expect_update_callback_is_called_when_file_is_modified(
    file: &mut TestFile,
    auth_file_watcher: &mut AuthFileWatcher,
) {
    let mut call_count = 0;
    for i in 0..10 {
        let message = AUTH_FAILED_MESSAGES[i % AUTH_FAILED_MESSAGES.len()];
        let line_to_add = create_log_line(message);
        file.write(&line_to_add);
        auth_file_watcher.update(|line| {
            call_count += 1;
            assert_eq!(line, &line_to_add);
        });
        assert_eq!(call_count, i + 1, "Callback call was expected");
    }
}

fn create_log_line(message: &str) -> String {
    let date_time = Local::now().format("%+");
    return format!("{} {}\n", date_time, message);
}

#[test]
fn when_more_than_one_line_is_added_then_update_callback_is_called_for_each_line() {
    let mut file = TestFile::new("auth-monitor-test");
    let mut auth_file_watcher =
        AuthFileWatcher::new(&file.filepath).expect("Error creating AuthFileWatcher");
    expect_no_update_callback_call(&mut auth_file_watcher);

    let mut call_count = 0;
    for i in 0..10 {
        let lines_to_add = AUTH_FAILED_MESSAGES.map(create_log_line);
        for line in &lines_to_add {
            file.write(line)
        }
        auth_file_watcher.update(|line| {
            assert_eq!(line, &lines_to_add[call_count % lines_to_add.len()]);
            call_count += 1;
        });
        assert_eq!(
            call_count,
            (i + 1) * lines_to_add.len(),
            "{} callback calls was expected",
            lines_to_add.len()
        );
    }
}

#[test]
fn when_new_file_was_created_after_old_was_deleted_then_changes_in_new_file_are_monitored() {
    let mut file = TestFile::new("auth-monitor-test");
    let mut auth_file_watcher =
        AuthFileWatcher::new(&file.filepath).expect("Error creating AuthFileWatcher");
    expect_no_update_callback_call(&mut auth_file_watcher);

    file.remove();
    expect_no_update_callback_call(&mut auth_file_watcher);

    file.create();
    expect_no_update_callback_call(&mut auth_file_watcher);

    expect_update_callback_is_called_when_file_is_modified(&mut file, &mut auth_file_watcher);
}

fn expect_no_update_callback_call(auth_file_watcher: &mut AuthFileWatcher) {
    auth_file_watcher.update(|_| {
        panic!("Callback call was not expected");
    });
}

#[test]
fn when_new_file_has_been_created_after_old_was_renamed_then_changes_in_new_file_are_monitored() {
    let mut file = TestFile::new("auth-monitor-test");
    let mut auth_file_watcher =
        AuthFileWatcher::new(&file.filepath).expect("Error creating AuthFileWatcher");
    expect_no_update_callback_call(&mut auth_file_watcher);

    rename_file(&file.filepath, "auth-monitor-test.bak");
    expect_no_update_callback_call(&mut auth_file_watcher);

    file.create();
    expect_no_update_callback_call(&mut auth_file_watcher);

    expect_update_callback_is_called_when_file_is_modified(&mut file, &mut auth_file_watcher);
}

fn rename_file(filepath: &str, new_filename: &str) {
    println!("Renaming test file {} to {}", filepath, new_filename);
    let new_path = Path::new(&filepath)
        .parent()
        .expect("Unable to get directory")
        .join(new_filename);
    let new_filepath = new_path.to_str().expect("Unable to build file path");
    rename(filepath, new_filepath).expect("Unable to rename test file");
}

#[test]
fn when_monitored_file_has_been_truncated_then_changes_are_still_monitored() {
    let mut file = TestFile::new("auth-monitor-test");
    let mut auth_file_watcher =
        AuthFileWatcher::new(&file.filepath).expect("Error creating AuthFileWatcher");
    expect_no_update_callback_call(&mut auth_file_watcher);

    file.truncate();
    expect_no_update_callback_call(&mut auth_file_watcher);

    expect_update_callback_is_called_when_file_is_modified(&mut file, &mut auth_file_watcher);
}

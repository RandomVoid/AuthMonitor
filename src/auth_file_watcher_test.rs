use std::env::temp_dir;
use std::fs::remove_file;
use std::fs::File;
use std::io::Write;

use chrono::Local;

use crate::auth_file_watcher::AuthFileWatcher;

#[test]
fn file_not_exist() {
    let filepath_buffer = temp_dir().join("auth-monitor-non-existing-file.log");
    let filepath = filepath_buffer.to_str().expect("Error creating filepath");
    match AuthFileWatcher::new(filepath) {
        Ok(_) => panic!("Error was expected"),
        Err(error) => assert_eq!(error.to_string(), "No such file or directory (os error 2)"),
    };
}

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

    pub fn write(&mut self, message: &str) {
        let bytes_to_add = message.as_bytes();
        let bytes_written = self
            .file
            .write(bytes_to_add)
            .expect("Error writing to file");
        assert_eq!(bytes_written, bytes_to_add.len());
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        println!("Removing test file: {}", self.filepath);
        remove_file(&self.filepath).expect("Unable to delete test file");
    }
}

#[test]
fn update_callback_is_called_when_new_line_is_added_to_file() {
    let mut file = TestFile::new("auth-monitor-test");
    let mut auth_file_watcher =
        AuthFileWatcher::new(&file.filepath).expect("Error creating AuthFileWatcher");
    auth_file_watcher.update(|_| {
        panic!("Callback call was not expected");
    });

    let mut call_count = 0;
    for i in 0..10 {
        let date_time = Local::now().format("%+");
        let message = AUTH_FAILED_MESSAGES[i % AUTH_FAILED_MESSAGES.len()];
        let line_to_add = format!("{} {}\n", date_time, message);
        file.write(&line_to_add);
        auth_file_watcher.update(|line| {
            call_count += 1;
            assert_eq!(line, &line_to_add);
        });
        assert_eq!(call_count, i + 1, "Callback call was expected");
    }
}

#[test]
fn update_callback_is_called_for_each_line_added_to_file() {
    let mut file = TestFile::new("auth-monitor-test");
    let mut auth_file_watcher =
        AuthFileWatcher::new(&file.filepath).expect("Error creating AuthFileWatcher");
    auth_file_watcher.update(|_| {
        panic!("Callback call was not expected");
    });

    let mut call_count = 0;
    for i in 0..10 {
        let lines_to_add = AUTH_FAILED_MESSAGES.map(|message| {
            let date_time = Local::now().format("%+");
            return format!("{} {}\n", date_time, message);
        });
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

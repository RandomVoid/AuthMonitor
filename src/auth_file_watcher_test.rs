use std::fs::File;
use std::io::Write;

use chrono::Local;

use crate::auth_file_watcher::AuthFileWatcher;

const AUTH_FAILED_MESSAGES: [&str; 2] = [
    "workstation sudo: pam_unix(sudo:auth): authentication failure; logname=john uid=1000 euid=0 tty=/dev/pts/7 ruser=john rhost=  user=john",
    "workstation kscreenlocker_greet: pam_unix(kde:auth): authentication failure; logname= uid=1000 euid=1000 tty= ruser= rhost=  user=john",
];

#[test]
fn update_callback_is_called_when_new_line_is_added_to_file() {
    let filepath = "/tmp/auth-monitor-test.log";
    let mut file = File::create(filepath).expect("Error creating test file");
    let mut auth_file_watcher =
        AuthFileWatcher::new(filepath).expect("Error creating AuthFileWatcher");
    auth_file_watcher.update(|_| {
        panic!("Callback call was not expected");
    });

    let mut call_count = 0;
    for i in 0..10 {
        let date_time = Local::now().format("%+");
        let line_to_add = format!("{} {}\n", date_time, AUTH_FAILED_MESSAGES[i % 2]);
        let bytes_to_add = line_to_add.as_bytes();
        let bytes_written = file.write(bytes_to_add).expect("Error writing to file");
        assert_eq!(bytes_written, bytes_to_add.len());
        auth_file_watcher.update(|line| {
            call_count += 1;
            assert_eq!(line, &line_to_add);
        });
        assert_eq!(call_count, i + 1, "Callback call was expected");
    }

    std::fs::remove_file(filepath).expect("Unable to delete test file");
}

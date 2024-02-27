use std::thread::sleep;
use std::time::Duration;

use crate::auth_monitor::AuthMonitor;
use crate::auth_monitor_options::AuthMonitorOptions;
use crate::auth_monitor_params::AuthMonitorParams;
use crate::test_utils::test_file::{create_log_line, TestFile};

const AUTH_FAILED_TEST_MESSAGES: [&str; 2] = [
    "workstation sudo: pam_unix(sudo:auth): authentication failure; logname=john uid=1000 euid=0 tty=/dev/pts/7 ruser=john rhost=  user=john",
    "workstation kscreenlocker_greet: pam_unix(kde:auth): authentication failure; logname= uid=1000 euid=1000 tty= ruser= rhost=  user=john",
];

const OTHER_TEST_MESSAGES: [&str; 4] = [
    "workstation dbus-daemon[1988]: [system] Failed to activate service 'org.bluez': timed out (service_start_timeout=25000ms)",
    "workstation CRON[9419]: pam_unix(cron:session): session opened for user root(uid=0) by (uid=0)",
    "workstation CRON[9419]: pam_unix(cron:session): session closed for user root",
    "workstation PackageKit: uid 1000 is trying to obtain org.freedesktop.packagekit.system-sources-refresh auth (only_trusted:0)",
];

#[test]
pub fn when_max_failed_attempts_limit_is_reached_then_update_callback_is_called() {
    for max_failed_attempts in 2..10 {
        let mut file = TestFile::with_unique_name();
        let mut auth_monitor = AuthMonitor::new(AuthMonitorParams {
            filepath: file.filepath.clone(),
            options: AuthMonitorOptions {
                max_failed_attempts,
                ..AuthMonitorOptions::default()
            },
        })
        .expect("Error creating AuthMonitor");

        expect_no_update_callback_call(&mut auth_monitor);
        expect_update_callback_is_not_called_after_writing_auth_failed_messages(
            &mut auth_monitor,
            &mut file,
            (max_failed_attempts - 1) as usize,
        );

        expect_update_callback_is_not_called_after_writing_other_messages(
            &mut auth_monitor,
            &mut file,
        );

        expect_update_callback_is_called_after_writing_auth_failed_message(
            &mut auth_monitor,
            &mut file,
        );
    }
}

fn expect_no_update_callback_call(auth_monitor: &mut AuthMonitor) {
    auth_monitor.update(|| {
        panic!("Callback call was not expected");
    });
}

fn expect_update_callback_is_not_called_after_writing_auth_failed_messages(
    auth_monitor: &mut AuthMonitor,
    file: &mut TestFile,
    message_count: usize,
) {
    for i in 0usize..message_count {
        let message_index = i % AUTH_FAILED_TEST_MESSAGES.len();
        let message = create_log_line(AUTH_FAILED_TEST_MESSAGES[message_index]);
        file.write(&message);
        expect_no_update_callback_call(auth_monitor);
    }
}

fn expect_update_callback_is_not_called_after_writing_other_messages(
    auth_monitor: &mut AuthMonitor,
    file: &mut TestFile,
) {
    for other_message in OTHER_TEST_MESSAGES {
        let message = create_log_line(other_message);
        file.write(&message);
        expect_no_update_callback_call(auth_monitor);
    }
}

fn expect_update_callback_is_called_after_writing_auth_failed_message(
    auth_monitor: &mut AuthMonitor,
    file: &mut TestFile,
) {
    let message = create_log_line(AUTH_FAILED_TEST_MESSAGES[0]);
    file.write(&message);
    expect_update_callback_is_called_once(auth_monitor);
}

fn expect_update_callback_is_called_once(auth_monitor: &mut AuthMonitor) {
    let mut call_count = 0;
    auth_monitor.update(|| call_count += 1);
    assert_eq!(call_count, 1, "One callback call was expected")
}

#[test]
pub fn when_reset_after_seconds_passed_then_failed_attempts_count_is_reset() {
    let options = AuthMonitorOptions {
        max_failed_attempts: 3,
        reset_after_seconds: 5,
    };
    let mut file = TestFile::with_unique_name();
    let mut auth_monitor = AuthMonitor::new(AuthMonitorParams {
        filepath: file.filepath.clone(),
        options,
    })
    .expect("Error creating AuthMonitor");

    expect_no_update_callback_call(&mut auth_monitor);
    expect_update_callback_is_not_called_after_writing_auth_failed_messages(
        &mut auth_monitor,
        &mut file,
        (options.max_failed_attempts - 1) as usize,
    );

    let sleep_duration = Duration::from_secs((options.reset_after_seconds + 1) as u64);
    println!("Sleeping for {} sec", sleep_duration.as_secs());
    sleep(sleep_duration);

    expect_update_callback_is_not_called_after_writing_auth_failed_messages(
        &mut auth_monitor,
        &mut file,
        (options.max_failed_attempts - 1) as usize,
    );

    expect_update_callback_is_called_after_writing_auth_failed_message(
        &mut auth_monitor,
        &mut file,
    );
}

#[test]
pub fn when_max_failed_attempts_limit_is_reached_before_update_is_called_then_update_callback_is_called(
) {
    for max_failed_attempts in 2..10 {
        let mut file = TestFile::with_unique_name();
        let mut auth_monitor = AuthMonitor::new(AuthMonitorParams {
            filepath: file.filepath.clone(),
            options: AuthMonitorOptions {
                max_failed_attempts,
                ..AuthMonitorOptions::default()
            },
        })
        .expect("Error creating AuthMonitor");

        expect_no_update_callback_call(&mut auth_monitor);
        write_auth_failed_messages(&mut file, (max_failed_attempts + 1) as usize);
        write_other_messages(&mut file, max_failed_attempts as usize);
        expect_update_callback_is_called_once(&mut auth_monitor);
    }
}

fn write_auth_failed_messages(file: &mut TestFile, count: usize) {
    write_messages(file, &AUTH_FAILED_TEST_MESSAGES, count);
}

fn write_other_messages(file: &mut TestFile, count: usize) {
    write_messages(file, &OTHER_TEST_MESSAGES, count);
}

fn write_messages(file: &mut TestFile, messages: &[&str], count: usize) {
    for i in 0usize..(count - 1) {
        let message_index = i % messages.len();
        let message = create_log_line(messages[message_index]);
        file.write(&message);
    }
}

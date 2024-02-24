use std::error::Error;
use std::time::{Duration, SystemTime};

use crate::auth_file_watcher::AuthFileWatcher;
use crate::auth_monitor_options::AuthMonitorOptions;
use crate::auth_monitor_params::AuthMonitorParams;
use crate::message_parser::is_auth_failed_message;

pub struct AuthMonitor {
    failed_attempts: i32,
    options: AuthMonitorOptions,
    file_watcher: AuthFileWatcher,
    last_failed_auth: SystemTime,
}

impl AuthMonitor {
    pub fn new(params: AuthMonitorParams) -> Result<AuthMonitor, Box<dyn Error>> {
        params.validate()?;
        return Ok(AuthMonitor {
            failed_attempts: 0,
            options: params.options,
            file_watcher: AuthFileWatcher::new(&params.filepath)?,
            last_failed_auth: SystemTime::UNIX_EPOCH,
        });
    }

    pub fn update(&mut self, on_max_failed_attempts: impl FnOnce()) {
        if self.should_reset_failed_attempts() {
            self.reset_failed_attempts();
        }
        let mut failed_attempts = 0;
        self.file_watcher.update(|line| {
            if is_auth_failed_message(line) {
                failed_attempts += 1;
            }
        });
        if failed_attempts > 0 {
            self.increase_failed_attempts(failed_attempts, on_max_failed_attempts);
        }
    }

    fn should_reset_failed_attempts(&self) -> bool {
        if self.failed_attempts <= 0 || self.failed_attempts >= self.options.max_failed_attempts {
            return false;
        }
        let seconds_from_last_error = SystemTime::now()
            .duration_since(self.last_failed_auth)
            .unwrap_or(Duration::ZERO)
            .as_secs();
        return seconds_from_last_error > self.options.reset_after_seconds as u64;
    }

    fn reset_failed_attempts(&mut self) {
        println!("Resetting failed attempts");
        self.failed_attempts = 0;
        self.last_failed_auth = SystemTime::now();
    }

    fn increase_failed_attempts(
        &mut self,
        failed_attempts: i32,
        on_max_failed_attempts: impl FnOnce(),
    ) {
        self.last_failed_auth = SystemTime::now();
        self.failed_attempts += failed_attempts;
        println!("Authentication failed {} time(s)", self.failed_attempts);
        if self.failed_attempts >= self.options.max_failed_attempts {
            println!("Authentication fail limit reached, shutting down");
            on_max_failed_attempts();
        }
    }
}

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::{Duration, SystemTime};

use crate::auth_file_watcher::AuthFileWatcher;
use crate::message_parser::is_auth_failed_message;

pub struct AuthMonitorParams {
    pub filepath: String,
    pub max_failed_attempts: i32,
    pub reset_after_seconds: i32,
}

impl Default for AuthMonitorParams {
    fn default() -> Self {
        return AuthMonitorParams {
            filepath: String::new(),
            max_failed_attempts: 3,
            reset_after_seconds: 1800,
        };
    }
}

impl Display for AuthMonitorParams {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(
            formatter,
            "filepath={}, max_failed_attempts={}, reset_after_seconds={}",
            self.filepath, self.max_failed_attempts, self.reset_after_seconds
        );
    }
}

pub struct AuthMonitor {
    failed_attempts: i32,
    max_failed_attempts: i32,
    reset_after_seconds: u64,
    file_watcher: AuthFileWatcher,
    last_failed_auth: SystemTime,
}

impl AuthMonitor {
    pub fn new(params: AuthMonitorParams) -> Result<AuthMonitor, Box<dyn Error>> {
        if params.max_failed_attempts <= 0 {
            return Err("max-failed-attempts must be greater than 0")?;
        }
        if params.reset_after_seconds <= 0 {
            return Err("reset-after-seconds must be greater than 0")?;
        }
        return Ok(AuthMonitor {
            failed_attempts: 0,
            max_failed_attempts: params.max_failed_attempts,
            reset_after_seconds: params.reset_after_seconds as u64,
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
        if self.failed_attempts <= 0 || self.failed_attempts >= self.max_failed_attempts {
            return false;
        }
        let seconds_from_last_error = SystemTime::now()
            .duration_since(self.last_failed_auth)
            .unwrap_or(Duration::ZERO)
            .as_secs();
        return seconds_from_last_error > self.reset_after_seconds;
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
        if self.failed_attempts >= self.max_failed_attempts {
            println!("Authentication fail limit reached, shutting down");
            on_max_failed_attempts();
        }
    }
}

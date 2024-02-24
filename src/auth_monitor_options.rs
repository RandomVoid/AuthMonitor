use std::fmt::{Display, Formatter};

const MAX_FAILED_ATTEMPTS: i32 = 3;
const RESET_AFTER_SECONDS: i32 = 1800;

pub struct AuthMonitorOptions {
    pub max_failed_attempts: i32,
    pub reset_after_seconds: i32,
}

impl Default for AuthMonitorOptions {
    fn default() -> Self {
        return AuthMonitorOptions {
            max_failed_attempts: MAX_FAILED_ATTEMPTS,
            reset_after_seconds: RESET_AFTER_SECONDS,
        };
    }
}

impl Display for AuthMonitorOptions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(
            formatter,
            "max_failed_attempts={}, reset_after_seconds={}",
            self.max_failed_attempts, self.reset_after_seconds
        );
    }
}

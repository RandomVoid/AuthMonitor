use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct AuthMonitorOptions {
    pub max_failed_attempts: i32,
    pub reset_after_seconds: i32,
}

impl Default for AuthMonitorOptions {
    fn default() -> Self {
        return AuthMonitorOptions {
            max_failed_attempts: 3,
            reset_after_seconds: 1800,
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

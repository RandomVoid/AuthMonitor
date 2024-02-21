use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const OPTION_PREFIX: &str = "--";
const OPTION_PREFIX_LENGTH: usize = OPTION_PREFIX.len();

const OPTION_VALUE_SEPARATOR: char = '=';
const OPTION_VALUE_SEPARATOR_LENGTH: usize = 1;

const MAX_FAILED_ATTEMPTS_OPTION: &str = "max-failed-attempts";
const RESET_AFTER_SECONDS_OPTION: &str = "reset-after-seconds";

const MAX_FAILED_ATTEMPTS: i32 = 3;
const RESET_AFTER_SECONDS: i32 = 1800;

pub struct AuthMonitorParams {
    pub filepath: String,
    pub max_failed_attempts: i32,
    pub reset_after_seconds: i32,
}

impl AuthMonitorParams {
    pub fn from_arguments(arguments: &[String]) -> Result<AuthMonitorParams, Box<dyn Error>> {
        let mut params = AuthMonitorParams::default();
        let arguments_iterator = arguments.iter();

        for argument in arguments_iterator {
            if !argument.starts_with(OPTION_PREFIX) {
                if !params.filepath.is_empty() {
                    Err("File path specified more than once")?;
                }
                params.filepath = String::from(argument);
                continue;
            }
            let (option_name, option_value) = match argument.find(OPTION_VALUE_SEPARATOR) {
                Some(separator_position) => {
                    let (name, value) = argument.split_at(separator_position);
                    (name, Some(&value[OPTION_VALUE_SEPARATOR_LENGTH..]))
                }
                None => (argument.as_str(), None),
            };
            match &option_name[OPTION_PREFIX_LENGTH..] {
                MAX_FAILED_ATTEMPTS_OPTION => {
                    params.max_failed_attempts =
                        Self::parse_option_value(option_name, option_value)?;
                }
                RESET_AFTER_SECONDS_OPTION => {
                    params.reset_after_seconds =
                        Self::parse_option_value(option_name, option_value)?;
                }
                _ => Err(format!("Unknown option {}", argument))?,
            }
        }

        params.validate()?;

        return Ok(params);
    }

    fn parse_option_value<T: FromStr>(
        name: &str,
        optional_value: Option<&str>,
    ) -> Result<T, Box<dyn Error>> {
        let value = match optional_value {
            Some(value) => value,
            None => Err(format!("Missing value for option {}", name))?,
        };
        if value.is_empty() {
            Err(format!("Missing value for option {}", name))?;
        }
        return match value.parse::<T>() {
            Ok(value) => Ok(value),
            Err(_) => Err(format!(
                "\"{}\" is not a valid value for option {}",
                value, name
            ))?,
        };
    }

    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.filepath.is_empty() {
            Err("File path not specified")?;
        }
        if self.max_failed_attempts <= 0 {
            return Err(format!(
                "{} must be greater than 0",
                MAX_FAILED_ATTEMPTS_OPTION
            ))?;
        }
        if self.reset_after_seconds <= 0 {
            return Err(format!(
                "{} must be greater than 0",
                RESET_AFTER_SECONDS_OPTION
            ))?;
        }
        return Ok(());
    }
}

impl Default for AuthMonitorParams {
    fn default() -> Self {
        return AuthMonitorParams {
            filepath: String::new(),
            max_failed_attempts: MAX_FAILED_ATTEMPTS,
            reset_after_seconds: RESET_AFTER_SECONDS,
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

#[cfg(test)]
#[path = "./auth_monitor_params_test.rs"]
mod test;

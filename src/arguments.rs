use std::error::Error;
use std::str::FromStr;

use crate::auth_monitor::AuthMonitorParams;

const OPTION_PREFIX: &str = "--";
const OPTION_PREFIX_LENGTH: usize = OPTION_PREFIX.len();

const OPTION_VALUE_SEPARATOR: char = '=';
const OPTION_VALUE_SEPARATOR_LENGTH: usize = 1;

const MAX_FAILED_ATTEMPTS_OPTION: &str = "max-failed-attempts";
const RESET_AFTER_SECONDS_OPTION: &str = "reset-after-seconds";

const DEFAULT_MAX_FAILED_ATTEMPTS: i32 = 3;
const DEFAULT_RESET_AFTER_SECONDS: i32 = 30 * 60;

pub fn parse_arguments(arguments: &[String]) -> Result<AuthMonitorParams, Box<dyn Error>> {
    let mut filepath: Option<String> = None;
    let mut max_failed_attempts = DEFAULT_MAX_FAILED_ATTEMPTS;
    let mut reset_after_seconds = DEFAULT_RESET_AFTER_SECONDS;
    let arguments_iterator = arguments.iter();

    for argument in arguments_iterator {
        if !argument.starts_with(OPTION_PREFIX) {
            if filepath.is_some() {
                Err("File path specified more than once")?;
            }
            filepath = Some(String::from(argument));
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
                max_failed_attempts = parse_option_value(option_name, option_value)?;
            }
            RESET_AFTER_SECONDS_OPTION => {
                reset_after_seconds = parse_option_value(option_name, option_value)?;
            }
            _ => Err(format!("Unknown option {}", argument))?,
        }
    }

    if filepath.is_none() {
        Err("File path not specified")?;
    }

    return Ok(AuthMonitorParams {
        filepath: filepath.unwrap(),
        max_failed_attempts,
        reset_after_seconds,
    });
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

#[cfg(test)]
#[path = "./arguments_test.rs"]
mod test;

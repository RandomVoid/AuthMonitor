use std::error::Error;

use crate::auth_monitor::AuthMonitorParams;

const DEFAULT_MAX_FAILED_ATTEMPTS: i32 = 3;
const DEFAULT_RESET_AFTER_SECONDS: i32 = 30 * 60;

pub fn parse_arguments(arguments: &Vec<String>) -> Result<AuthMonitorParams, Box<dyn Error>> {
    let mut filepath: Option<String> = None;
    let mut max_failed_attempts = DEFAULT_MAX_FAILED_ATTEMPTS;
    let mut reset_after_seconds = DEFAULT_RESET_AFTER_SECONDS;
    let mut iterator = arguments.iter();
    while let Some(arg) = iterator.next() {
        if !arg.starts_with("--") {
            if filepath.is_some() {
                return Err("File path specified more than once")?;
            }
            filepath = Some(String::from(arg));
            continue;
        }
        let mut arguments = arg.split("=");
        let name = match arguments.next() {
            Some(name) => String::from_iter(name.chars().skip(2)),
            None => Err("Missing option name")?,
        };
        let value = match arguments.next() {
            Some(value) => value,
            None => Err(format!("Missing value for option {}", name))?,
        };
        match name.as_str() {
            "max-failed-attempts" => {
                max_failed_attempts = match value.parse::<i32>() {
                    Ok(value) => value,
                    Err(error) => Err(format!("Invalid value for option {}: {}", name, error))?,
                };
            }
            "reset-after-seconds" => {
                reset_after_seconds = match value.parse::<i32>() {
                    Ok(value) => value,
                    Err(error) => Err(format!("Invalid value for option {}: {}", name, error))?,
                };
            }
            _ => Err(format!("Unknown option \"{}\"", name))?,
        }
    }
    if filepath.is_none() {
        return Err("File path not specified")?;
    }
    return Ok(AuthMonitorParams {
        filepath: filepath.unwrap(),
        max_failed_attempts,
        reset_after_seconds,
    });
}

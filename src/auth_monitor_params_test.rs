use std::error::Error;

use crate::auth_monitor_params::{
    AuthMonitorParams, MAX_FAILED_ATTEMPTS_OPTION, RESET_AFTER_SECONDS_OPTION,
};

const FILEPATH: &str = "/var/log/auth.log";
const ALL_OPTIONS: [&str; 2] = [MAX_FAILED_ATTEMPTS_OPTION, RESET_AFTER_SECONDS_OPTION];

#[test]
fn when_parsing_no_arguments_then_return_error() {
    expect_error(
        AuthMonitorParams::from_arguments(&[]),
        "File path not specified",
    );
}

fn expect_error(result: Result<AuthMonitorParams, Box<dyn Error>>, expected: &str) {
    match result {
        Ok(_) => panic!("Error \"{}\" was expected", expected),
        Err(error) => assert_eq!(error.to_string(), expected),
    }
}

#[test]
fn when_parsing_filepath_without_options_then_return_struct_with_default_values() {
    let arguments = [String::from(FILEPATH)];
    let expected = AuthMonitorParams {
        filepath: String::from(FILEPATH),
        ..AuthMonitorParams::default()
    };
    expect_equals(AuthMonitorParams::from_arguments(&arguments), &expected);
}

fn expect_equals(result: Result<AuthMonitorParams, Box<dyn Error>>, expected: &AuthMonitorParams) {
    let params = result.unwrap();
    assert_eq!(
        params.filepath, expected.filepath,
        "File path does not match"
    );
    assert_eq!(
        params.max_failed_attempts, expected.max_failed_attempts,
        "Max failed attempts does not match"
    );
    assert_eq!(
        params.reset_after_seconds, expected.reset_after_seconds,
        "Reset after seconds does not match"
    );
}

#[test]
fn when_parsing_filepath_with_one_option_with_correct_value_then_return_params_with_that_value() {
    let default_params = AuthMonitorParams::default();
    for option in ALL_OPTIONS {
        let value = 30;
        let option_argument = format!("--{}={}", option, value);
        let arguments = [String::from(FILEPATH), option_argument];
        let max_failed_attempts = match option == MAX_FAILED_ATTEMPTS_OPTION {
            true => value,
            false => default_params.max_failed_attempts,
        };
        let reset_after_seconds = match option == RESET_AFTER_SECONDS_OPTION {
            true => value,
            false => default_params.reset_after_seconds,
        };
        let expected = AuthMonitorParams {
            filepath: String::from(FILEPATH),
            max_failed_attempts,
            reset_after_seconds,
        };
        expect_equals(AuthMonitorParams::from_arguments(&arguments), &expected);
    }
}

#[test]
fn when_parsing_option_without_value_then_return_error() {
    for option in ALL_OPTIONS {
        let option_arguments = [format!("--{}", option), format!("--{}=", option)];
        for option_argument in option_arguments {
            let arguments = [String::from(FILEPATH), option_argument];
            let expected = format!("Missing value for option --{}", option);
            expect_error(AuthMonitorParams::from_arguments(&arguments), &expected);
        }
    }
}

#[test]
fn when_parsing_option_with_invalid_value_then_return_error() {
    for option in ALL_OPTIONS {
        let values = ["test", "3a", "--", "b23", "12@"];
        for value in values {
            let option_argument = format!("--{}={}", option, value);
            let arguments = [String::from(FILEPATH), option_argument];
            let expected = format!("\"{}\" is not a valid value for option --{}", value, option);
            expect_error(AuthMonitorParams::from_arguments(&arguments), &expected);
        }
    }
}

#[test]
fn when_parsing_filename_and_multiple_options_then_return_params_with_parsed_values() {
    let max_failed_attempts = 10;
    let reset_after_seconds = 3600;
    let arguments = [
        String::from(FILEPATH),
        format!("--{}={}", MAX_FAILED_ATTEMPTS_OPTION, max_failed_attempts),
        format!("--{}={}", RESET_AFTER_SECONDS_OPTION, reset_after_seconds),
    ];
    let expected = AuthMonitorParams {
        filepath: String::from(FILEPATH),
        max_failed_attempts,
        reset_after_seconds,
    };
    expect_equals(AuthMonitorParams::from_arguments(&arguments), &expected);
}

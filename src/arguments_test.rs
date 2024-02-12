use crate::arguments::{
    parse_arguments, DEFAULT_MAX_FAILED_ATTEMPTS, DEFAULT_RESET_AFTER_SECONDS,
    MAX_FAILED_ATTEMPTS_OPTION, RESET_AFTER_SECONDS_OPTION,
};

const FILEPATH: &str = "/var/log/auth.log";

#[test]
fn parse_no_arguments() {
    let arguments = [];
    match parse_arguments(&arguments) {
        Ok(_) => panic!("Error was expected"),
        Err(error) => assert_eq!(error.to_string(), "File path not specified"),
    }
}

#[test]
fn parse_filepath_without_options() {
    let arguments = [String::from(FILEPATH)];
    test_parse_arguments(
        &arguments,
        FILEPATH,
        DEFAULT_MAX_FAILED_ATTEMPTS,
        DEFAULT_RESET_AFTER_SECONDS,
    );
}

fn test_parse_arguments(
    arguments: &[String],
    filepath: &str,
    max_failed_attempts: i32,
    reset_after_seconds: i32,
) {
    match parse_arguments(arguments) {
        Ok(params) => {
            assert_eq!(params.filepath, filepath);
            assert_eq!(params.max_failed_attempts, max_failed_attempts);
            assert_eq!(params.reset_after_seconds, reset_after_seconds);
        }
        Err(error) => panic!("Error: {}", error),
    }
}

#[test]
fn parse_options_with_correct_values() {
    let options = [MAX_FAILED_ATTEMPTS_OPTION, RESET_AFTER_SECONDS_OPTION];
    for option in options {
        let value = 30;
        let option_argument = format!("--{}={}", option, value);
        let arguments = [String::from(FILEPATH), option_argument];
        let max_failed_attempts = match option == MAX_FAILED_ATTEMPTS_OPTION {
            true => value,
            false => DEFAULT_MAX_FAILED_ATTEMPTS,
        };
        let reset_after_seconds = match option == RESET_AFTER_SECONDS_OPTION {
            true => value,
            false => DEFAULT_RESET_AFTER_SECONDS,
        };
        test_parse_arguments(
            &arguments,
            FILEPATH,
            max_failed_attempts,
            reset_after_seconds,
        );
    }
}

#[test]
fn parse_option_without_value() {
    let options = [MAX_FAILED_ATTEMPTS_OPTION, RESET_AFTER_SECONDS_OPTION];
    for option in options {
        let option_arguments = [format!("--{}", option), format!("--{}=", option)];
        for option_argument in option_arguments {
            let arguments = [String::from(FILEPATH), option_argument];
            match parse_arguments(&arguments) {
                Ok(_) => panic!("Error was expected"),
                Err(error) => {
                    assert_eq!(
                        error.to_string(),
                        format!("Missing value for option --{}", option)
                    );
                }
            }
        }
    }
}

#[test]
fn parse_option_with_invalid_value() {
    let options = [MAX_FAILED_ATTEMPTS_OPTION, RESET_AFTER_SECONDS_OPTION];
    for option in options {
        let values = ["test", "3a", "--", "b23", "12@"];
        for value in values {
            let option_argument = format!("--{}={}", option, value);
            let arguments = [String::from(FILEPATH), option_argument];
            match parse_arguments(&arguments) {
                Ok(_) => panic!("Should return an error"),
                Err(error) => {
                    assert_eq!(
                        error.to_string(),
                        format!("\"{}\" is not a valid value for option --{}", value, option)
                    );
                }
            }
        }
    }
}

#[test]
fn parse_multiple_options() {
    let max_failed_attempts = 10;
    let reset_after_seconds = 3600;
    let arguments = [
        String::from(FILEPATH),
        format!("--{}={}", MAX_FAILED_ATTEMPTS_OPTION, max_failed_attempts),
        format!("--{}={}", RESET_AFTER_SECONDS_OPTION, reset_after_seconds),
    ];
    test_parse_arguments(
        &arguments,
        FILEPATH,
        max_failed_attempts,
        reset_after_seconds,
    );
}

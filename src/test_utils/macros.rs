#[macro_export]
macro_rules! assert_error {
    ($result:expr, $expected:expr) => {
        match &$result {
            Ok(_) => panic!("Error \"{}\" was expected", $expected),
            Err(error) => assert_eq!(error.to_string(), $expected),
        }
    };
}

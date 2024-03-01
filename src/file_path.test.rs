use crate::assert_error;
use crate::file_path::FilePath;

#[test]
fn when_parsing_path_to_existing_file_then_return_canonical_directory_path_and_filename() {
    let paths = ["/var/log/auth.log", "/var/log/../log/../log/auth.log"];
    for path in paths {
        let path = FilePath::from(path).unwrap();
        assert_eq!(path.directory, "/var/log");
        assert_eq!(path.filename, "auth.log");
    }
}

#[test]
fn when_parsing_file_path_to_non_existing_file_then_return_no_such_file_error() {
    let paths = [
        "/nonexistent/path/file.log",
        "/nonexistent/../path/../file.log",
    ];
    for path in paths {
        assert_error!(
            FilePath::from(path),
            "Error parsing directory path: No such file or directory (os error 2)"
        );
    }
}

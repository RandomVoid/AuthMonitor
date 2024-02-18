use std::error::Error;
use std::path::Path;

pub struct FilePath {
    pub directory: String,
    pub filename: String,
}

impl FilePath {
    pub fn parse(filepath: &str) -> Result<FilePath, Box<dyn Error>> {
        let full_path = Path::new(filepath);
        let directory_path = match full_path.parent() {
            Some(dir) => dir,
            None => Err("Unable to get directory name from file path")?,
        };
        let directory_canonical_path = match directory_path.canonicalize() {
            Ok(path) => path,
            Err(error) => Err(format!("Error parsing directory path: {}", error))?,
        };
        let directory = match directory_canonical_path.to_str() {
            Some(directory) => {
                if directory.is_empty() {
                    Err("Directory name is empty")?;
                }
                String::from(directory)
            }
            None => Err("Unable to convert directory name to string")?,
        };
        let optional_filename = match full_path.file_name() {
            Some(filename) => filename.to_str(),
            None => Err("Unable to get file name from file path")?,
        };
        let filename = match optional_filename {
            Some(filename) => {
                if filename.is_empty() {
                    Err("File name is empty")?;
                }
                String::from(filename)
            }
            None => Err("Unable to convert file name to string")?,
        };

        return Ok(FilePath {
            directory,
            filename,
        });
    }
}

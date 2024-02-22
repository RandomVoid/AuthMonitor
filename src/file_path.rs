use std::error::Error;
use std::path::Path;

pub struct FilePath {
    pub directory: String,
    pub filename: String,
}

impl FilePath {
    pub fn from(filepath: &str) -> Result<FilePath, Box<dyn Error>> {
        let full_path = Path::new(filepath);
        return Ok(FilePath {
            directory: Self::get_directory(full_path)?,
            filename: Self::get_filename(full_path)?,
        });
    }

    fn get_directory(full_path: &Path) -> Result<String, Box<dyn Error>> {
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
        return Ok(directory);
    }

    fn get_filename(full_path: &Path) -> Result<String, Box<dyn Error>> {
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
        return Ok(filename);
    }
}

#[cfg(test)]
#[path = "./file_path_test.rs"]
mod test;

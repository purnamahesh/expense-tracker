use directories;
use std::error::Error;
use std::fs::DirBuilder;
use std::path::{Path, PathBuf};
use std::process::exit;

use directories::ProjectDirs;

use crate::constants::FILE_NAME;

pub fn get_project_db_file_path() -> Result<String, Box<dyn Error>> {
    if let Some(p) = ProjectDirs::from("", "", "pense") {
        // self.config_dir = Some(p.config_dir().to_path_buf());
        let data_dir = p.data_dir().to_path_buf();

        let path = match data_dir.to_str() {
            Some(path_str) => {
                create_project_dir_if_not_exists(path_str)?;
                path_str
            }
            None => {
                eprintln!("utf-8: Failed to get data dir");
                exit(1);
            }
        };

        Ok(format!("{}/{}", path, FILE_NAME))
    } else {
        Err("Non utf-8 character in home path".into())
    }
}

pub fn validate_file_path(path: &Path) -> Result<(), &'static str> {
    if path.is_dir() {
        return Err("directory path is provided instead of a file path; eg: ../filename.db");
    }
    if let Some(path_str) = path.to_str() {
        if !path_str.ends_with(".db") {
            return Err(
                "Not a db file; Should be <some relative or absolute path>/some_filename.db",
            );
        }
    } else {
        return Err("Non utf-8 characters found in path");
    }

    Ok(())
}

pub fn construct_file_path(path: &str) -> Result<PathBuf, &'static str> {
    let input_path = Path::new(&path);

    let path = if input_path.is_relative() {
        if input_path.starts_with("~/") {
            let input_path_string = match input_path.to_str() {
                Some(path) => &path[2..],
                None => return Err("Invalid characters in path"),
            };
            let path_from_home_dir = Path::new(input_path_string);

            if let Some(base_dirs) = directories::BaseDirs::new() {
                let mut hdir = base_dirs.home_dir().to_path_buf();
                hdir.push(path_from_home_dir);
                return Ok(hdir);
            } else {
                return Err("Unable to fetch homedir");
            }
        } else {
            input_path.to_path_buf()
        }
    } else {
        input_path.to_path_buf()
    };

    validate_file_path(&path)?;

    Ok(input_path.to_path_buf())
}

pub fn create_project_dir_if_not_exists(dir_name: &str) -> Result<(), Box<dyn Error>> {
    let p_dir = Path::new(dir_name);

    if !p_dir.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(p_dir)
            .unwrap_or_else(|err| {
                eprintln!(
                    "Error creating directory at {}\nError: {}",
                    &p_dir.display(),
                    err
                );
            });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};
    use std::env;

    use super::*;

    #[fixture]
    fn current_file_name() -> PathBuf {
        let args: Vec<String> = env::args().collect();
        PathBuf::from(args[0].to_owned())
    }

    #[rstest]
    fn test_construct_file_path() {
        assert!(construct_file_path("README.md").is_err());

        assert_eq!(
            construct_file_path("./temp.db"),
            Ok(PathBuf::from("./temp.db"))
        );

        // assert_eq!(
        //     construct_file_path("~/filename.psv"),
        //     Ok(PathBuf::from(home_path))
        // );
    }

    // Testing validate_file_pat
    #[rstest]
    fn test_validate_file_path_valid(current_file_name: PathBuf) {
        assert!(
            validate_file_path(
                &current_file_name
                    .as_path()
                    .parent()
                    .unwrap()
                    .join("temp.db")
                    .to_path_buf()
            )
            .is_ok()
        );
    }

    #[rstest]
    fn test_validate_file_path_invalid_file(current_file_name: PathBuf) {
        assert!(
            validate_file_path(
                &current_file_name
                    .as_path()
                    .parent()
                    .unwrap()
                    .join("no_exists.txt")
                    .to_path_buf()
            )
            .is_err()
        );
    }

    #[rstest]
    fn test_validate_file_path_invalid_dir(current_file_name: PathBuf) {
        assert!(validate_file_path(current_file_name.as_path().parent().unwrap()).is_err());
    }
}

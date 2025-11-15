use std::env::home_dir;
use std::error::Error;
use std::fs::DirBuilder;
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::{ErrorKind, Read};
use std::process::exit;

use crate::config::FILE_NAME;

pub fn validate_file_path(arg_path: &Option<PathBuf>) -> Result<(), &'static str> {
    if let Some(path) = arg_path {
        if !path.exists() {
            return Err("File not found!");
        } else if !path.is_file() {
            return Err("Not a file");
        }
    }

    Ok(())
}

pub fn construct_file_path(path: &str) -> Result<PathBuf, &'static str> {
    let input_path = Path::new(&path);

    if input_path.is_relative() {
        if input_path.starts_with("~/") {
            let input_path_string = match input_path.to_str() {
                Some(path) => &path[2..],
                None => return Err("Invalid characters in path"),
            };
            let path_from_home_dir = Path::new(input_path_string);

            if let Some(mut hdir) = home_dir() {
                hdir.push(path_from_home_dir);
                return Ok(hdir);
            } else {
                return Err("Error constructing home directory");
            }
        } else {
            return Ok(input_path.to_path_buf());
        }
    }

    Ok(input_path.to_path_buf())
}

pub fn validate_project_dir(dir_name: &Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    let p_dir = dir_name.as_ref().unwrap();

    if !p_dir.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(p_dir.as_path())
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

pub fn generate_read_path(
    file_path: Option<PathBuf>,
    project_dir: &Option<PathBuf>,
) -> Result<PathBuf, Box<dyn Error + 'static>> {
    validate_project_dir(project_dir)?;
    let p_dir = project_dir.as_ref().unwrap();
    let path = file_path.unwrap_or(p_dir.join(FILE_NAME));
    Ok(path)
}

pub fn read_file_content(
    file_path: Option<PathBuf>,
    project_dir: &Option<PathBuf>,
) -> Result<String, Box<dyn Error + 'static>> {
    let path = generate_read_path(file_path, project_dir)?;
    let mut file = OpenOptions::new()
        .read(true)
        .open(&path)
        .unwrap_or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                eprintln!("No records yet at {}", path.display());
                exit(1)
            } else {
                eprintln!("Error {}", err);
                exit(1)
            };
        });

    let mut content = String::from("");

    file.read_to_string(&mut content)?;

    Ok(content)
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     #[should_panic(expected="No records yet at ")]
//     fn test_load_expenses_from_psv() {
//         let _ = read_file_content(Some("./expense_db.csv"));
//     }
// }

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};
    use std::env;

    use super::*;

    // #[should_panic]
    #[fixture]
    fn home_path() -> String {
        #[cfg(target_os = "windows")]
        let path = env::var("USERPROFILE").unwrap();

        #[cfg(target_os = "linux")]
        let path = env::var("HOME").unwrap();

        #[cfg(target_os = "macos")]
        let path = env::var("HOME").unwrap();

        path
    }

    #[fixture]
    fn current_file_name() -> Option<PathBuf> {
        let arrgs: Vec<String> = env::args().collect();
        Some(PathBuf::from(arrgs[0].to_owned()))
    }

    #[rstest]
    fn test_construct_file_path(home_path: String) {
        assert_eq!(
            construct_file_path("filename.psv"),
            Ok(PathBuf::from("filename.psv"))
        );

        assert_eq!(
            construct_file_path("./filename.psv"),
            Ok(PathBuf::from("./filename.psv"))
        );

        assert_eq!(
            construct_file_path("~/filename.psv"),
            Ok(PathBuf::from(home_path).join("filename.psv"))
        );
    }

    // Testing validate_file_pat
    #[rstest]
    fn test_validate_file_path_valid(current_file_name: Option<PathBuf>) {
        assert!(validate_file_path(&current_file_name).is_ok());
    }

    #[rstest]
    fn test_validate_file_path_invalid_file(current_file_name: Option<PathBuf>) {
        assert!(
            validate_file_path(&Some(
                current_file_name
                    .unwrap()
                    .as_path()
                    .parent()
                    .unwrap()
                    .join("no_exists.txt")
                    .to_path_buf()
            ))
            .is_err()
        );
    }

    #[rstest]
    fn test_validate_file_path_invalid_dir(current_file_name: Option<PathBuf>) {
        assert!(
            validate_file_path(&Some(
                current_file_name
                    .unwrap()
                    .as_path()
                    .parent()
                    .unwrap()
                    .to_path_buf()
            ))
            .is_err()
        );
    }
}

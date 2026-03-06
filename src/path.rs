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
            },
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

pub fn validate_file_path(path: &PathBuf) -> Result<(), &'static str> {
    if !path.exists() {
        return Err("File not found!");
    } else if !path.is_file() {
        return Err("Not a file");
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
                hdir
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

// pub fn generate_read_path(file_path: Option<PathBuf>) -> Result<PathBuf, Box<dyn Error + 'static>> {
//     validate_project_dir(project_dir)?;
//     let p_dir = project_dir.as_ref().unwrap();
//     let path = file_path.unwrap_or(p_dir.join(FILE_NAME));
//     Ok(path)
// }

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
    fn current_file_name() -> PathBuf {
        let arrgs: Vec<String> = env::args().collect();
        PathBuf::from(arrgs[0].to_owned())
    }

    #[rstest]
    fn test_construct_file_path(home_path: String) {
        println!("{:?}", construct_file_path("filename.psv"));
        assert_eq!(
            construct_file_path("README.md"),
            Ok(PathBuf::from("README.md"))
        );

        assert_eq!(
            construct_file_path("./README.md"),
            Ok(PathBuf::from("./README.md"))
        );

        // assert_eq!(
        //     construct_file_path("~/filename.psv"),
        //     Ok(PathBuf::from(home_path).join("filename.psv"))
        // );
    }

    // Testing validate_file_pat
    #[rstest]
    fn test_validate_file_path_valid(current_file_name: PathBuf) {
        assert!(validate_file_path(&current_file_name).is_ok());
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
        assert!(
            validate_file_path(&current_file_name.as_path().parent().unwrap().to_path_buf())
                .is_err()
        );
    }
}

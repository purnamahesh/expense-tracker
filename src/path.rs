use std::env::home_dir;
use std::fs::DirBuilder;
use std::path::{Path, PathBuf};
use std::process::exit;

pub fn construct_file_path(path: &str) -> PathBuf {
    let input_path = Path::new(&path);

    if input_path.is_relative() {
        if input_path.starts_with("~/") {
            let input_path_string = match input_path.to_str() {
                Some(path) => &path[2..],
                None => {
                    eprintln!("Invalid characters in path {}", &path);
                    exit(1)
                }
            };
            let path_from_home_dir = Path::new(input_path_string);

            if let Some(mut hdir) = home_dir() {
                hdir.push(path_from_home_dir);
                return hdir;
            } else {
                eprintln!("Error constructing home directory");
                exit(1)
            }
        } else {
            return input_path.to_path_buf();
        }
    }

    input_path.to_path_buf()
}

pub fn _initiate_config_file() {
    let config_dir = match home_dir() {
        Some(home_dir) => {
            let config_dir: std::path::PathBuf = home_dir.join(Path::new("expense-tracker/"));
            if !config_dir.exists() {
                DirBuilder::new()
                    .recursive(false)
                    .create(config_dir.as_path())
                    .unwrap_or_else(|err| {
                        eprintln!(
                            "Error creating directory at {}\nError: {}",
                            config_dir.display(),
                            err
                        );
                        exit(1)
                    });
            };
            config_dir
        }
        None => {
            eprint!("Home directory might have "); // TODO
            exit(1);
        }
    };

    let db_file = config_dir.join(Path::new("expense_db.psv"));

    if db_file.exists() {};

    // let y = Path::new(home_dir);

    // println!("{}", x.display());
}

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

    #[rstest]
    fn test_construct_file_path(home_path: String) {
        assert_eq!(
            construct_file_path("filename.psv"),
            PathBuf::from("filename.psv")
        );

        assert_eq!(
            construct_file_path("./filename.psv"),
            PathBuf::from("./filename.psv")
        );

        assert_eq!(
            construct_file_path("~/filename.psv"),
            PathBuf::from(home_path).join("filename.psv")
        );
    }
}

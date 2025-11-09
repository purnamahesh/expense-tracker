use std::env::home_dir;
use std::path::{Path, PathBuf};

// fn validate_file_path(arg: &str) -> Result<PathBuf, &'static str> {
//     let actual_path = construct_file_path(arg);

//     if !actual_path.exists() {
//         return Err("File doesn't exist");
//     } else if !actual_path.is_file() {
//         return Err("Not a file");
//     }

//     Ok(actual_path)
// }

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

// use std::fs::DirBuilder;
// use std::process::exit;
// pub fn _initiate_config_file() {
//     let config_dir = match home_dir() {
//         Some(home_dir) => {
//             let config_dir: std::path::PathBuf = home_dir.join(Path::new("expense-tracker/"));
//             if !config_dir.exists() {
//                 DirBuilder::new()
//                     .recursive(false)
//                     .create(config_dir.as_path())
//                     .unwrap_or_else(|err| {
//                         eprintln!(
//                             "Error creating directory at {}\nError: {}",
//                             config_dir.display(),
//                             err
//                         );
//                         exit(1)
//                     });
//             };
//             config_dir
//         }
//         None => {
//             eprint!("Home directory might have "); // TODO
//             exit(1);
//         }
//     };

//     let db_file = config_dir.join(Path::new("expense_db.psv"));

//     if db_file.exists() {};

//     // let y = Path::new(home_dir);

//     // println!("{}", x.display());
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
}

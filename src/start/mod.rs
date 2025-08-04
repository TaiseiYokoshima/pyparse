use std::env;
use std::ffi::OsString;
use std::fs;
use std::path;
use std::process::exit;

pub fn parse_path() -> path::PathBuf {
    let mut args: Vec<OsString> = env::args_os().collect();

    if args.len() > 2 {
        eprintln!("Error: More than 1 argument passed");
        exit(1);
    } else if args.len() < 2 {
        eprintln!("Error: File path not given");
        exit(1);
    }

    let mut path = path::PathBuf::from(args.remove(1));

    path = if path.is_relative() {
        match env::current_dir() {
            Ok(cwd) => cwd.join(path),
            Err(e) => {
                eprintln!(
                    "Error: Could not read current working directly due to {}",
                    e.kind()
                );
                exit(1);
            }
        }
    } else {
        path
    };

    if !path.exists() {
        eprintln!("Error: File does not exist");
        exit(1);
    }

    if path.is_dir() {
        eprintln!("Error: Filepath is a directory not a python file");
        exit(1);
    }

    return path;
}

pub fn load_src(path: &path::PathBuf) -> String {
    match fs::read_to_string(path) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("Error: Could not load python file due to {}", e.kind());
            exit(1);
        }
    }
}

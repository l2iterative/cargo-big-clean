use std::env;
use std::path::{Path, PathBuf};

fn check_if_rust_project_with_target(path_buf: &PathBuf) -> bool {
    if !path_buf.join("Cargo.toml").is_file() {
        return false;
    }

    if !path_buf.join("Cargo.lock").is_file() {
        return false;
    }

    if !path_buf.join("target").is_dir() {
        return false;
    }

    if !path_buf.join("target").join("debug").is_dir()
        && !path_buf.join("target").join("release").is_dir()
    {
        return false;
    }

    if !path_buf.join("target").join("CACHEDIR.TAG").is_file() {
        return false;
    }

    return true;
}

fn walk_dir(path_buf: PathBuf, depth: usize) {
    if depth > 3 {
        return;
    }
    let path = Path::new(&path_buf);
    let res = path.read_dir();
    if res.is_ok() {
        for entry in res.unwrap() {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    if check_if_rust_project_with_target(&entry_path) {
                        println!("Cleaning {}", entry_path.to_str().unwrap());
                        let _ = std::fs::remove_dir_all(entry_path.join("target"));
                    } else {
                        if !std::fs::symlink_metadata(&entry_path)
                            .unwrap()
                            .file_type()
                            .is_symlink()
                        {
                            walk_dir(entry_path, depth + 1);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.to_str().unwrap());

    if check_if_rust_project_with_target(&current_dir) {
        println!("Cleaning {}", current_dir.to_str().unwrap());
        std::fs::remove_dir_all(current_dir.join("target")).unwrap();
    }

    walk_dir(current_dir, 0);
}

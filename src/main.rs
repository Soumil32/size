use std::fs::{self, metadata};
use std::env::args;
use humansize::{format_size, DECIMAL};

enum PathType {
    File,
    Folder,
    Other
}

fn main() {
    let args = args().collect::<Vec<String>>();
    let path = args[1].as_str();
    let path_type = is_file_or_folder(path);
    match path_type {
        PathType::File => {
            let file_size = get_size_of_file(path);
            println!("{}", format_size(file_size, DECIMAL));
        },
        PathType::Folder => {
            let dir_size = get_size_of_folder(path);
            println!("{}", format_size(dir_size, DECIMAL));
        },
        PathType::Other => {
            println!("Other");
        }
    }
}

fn get_size_of_file(path: &str) -> u64 {
    let metadata = metadata(path).unwrap();
    metadata.len()
}

fn get_size_of_folder(path: &str) -> u64 {
    let mut total_size = 0;
    for entry in fs::read_dir(path).unwrap() {
        let entry = match entry {
            Ok(res) => {res},
            Err(_) => {continue},
        };
        let path = entry.path();
        let metadata = match metadata(&path) {
            Ok(res) => {res},
            Err(_) => {continue},
        };
        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            total_size += get_size_of_folder(path.to_str().unwrap());
        }
    }
    total_size
}

fn is_file_or_folder(path: &str) -> PathType {
    let metadata = metadata(path).unwrap();
    if metadata.is_file() {
        PathType::File
    } else if metadata.is_dir() {
        PathType::Folder
    } else {
        PathType::Other
    }
}
